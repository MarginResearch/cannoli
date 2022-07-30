//! The core implementation of Cannoli. This is "hidden" as it's not the right
//! place to really be adding customization and hooks unless you know what
//! you're doing.

// Make sure the target OS is Linux
#[cfg(not(target_os = "linux"))]
compile_error!("Currently only Linux support is available. This includes \
    raw assembly instructions for qemu-user, which is Linux only");

// Make sure we're only targeting x86_64
#[cfg(not(target_arch = "x86_64"))]
compile_error!("This code literally has x86_64 assembly at its core, so uhh \
    x86_64 only right now :)");

use std::io::Write;
use std::ffi::CStr;
use std::net::TcpStream;
use std::mem::{ManuallyDrop, size_of};
use std::cell::{RefCell, UnsafeCell, RefMut};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
use cannoli::{Architecture, ClientConn};
use mempipe::{SendPipe, ChunkWriter};

/// Chunk size to use when streaming data over IPC
const CHUNK_SIZE: usize = 256 * 1024;

/// Number of chunks to use with IPC
const NUM_BUFFERS: usize = 16;

// Pull in the FFI bindings we generated
include!(concat!(env!("OUT_DIR"), "/ffi_bindings.rs"));

extern "Rust" {
    fn hook_inst(_pc: u64) -> HookType;
    fn hook_mem(_pc: u64, _write: bool, _size: usize) -> bool;
}

/// Different types of hooks
#[derive(Clone, Copy)]
pub enum HookType {
    /// The callback for a given hook gets invoked only once for a given PC.
    ///
    /// This uses self-modifying code to disable the callback once it has been
    /// hit once. It's possible due to race conditions or to QEMU re-JITting
    /// the code that the callback fires again. This is effectively a massive
    /// filter to prevent getting callbacks every single instruction, rather
    /// you get them once per JIT of that code.
    ///
    /// Use this if you only care about things like binary coverage, and you
    /// are not collecting a trace, you just want to know about the first event
    /// for a given hook.
    Once,

    /// Hook fires every single time the instruction is hit
    Always,

    /// Hook fires every time an instruction is hit, and reports PC and the
    /// GPR state for the target architecture
    Register,

    /// Don't hook at all
    Never,
}

/// The per-thread storage for an active hook. This has to be per-thread
/// storage because we have to conjure it out of thin air from globals, since
/// we can't really transfer state inside of QEMU without grossness.
struct HookState {
    /// Pipe to use to send data out of QEMU to the processing process
    pipe: SendPipe<CHUNK_SIZE, NUM_BUFFERS>,

    /// Connection to the server for sending metadata needed to establish IPC
    _server: TcpStream,

    /// Currently active buffer. This is set upon JIT entries, and taken on JIT
    /// exits.
    ///
    /// This is technically not `'static` we're doing some gross things to live
    /// through the boundaries of the JIT entry and exit
    active_buffer: Option<
        ManuallyDrop<ChunkWriter<'static, CHUNK_SIZE, NUM_BUFFERS>>>,
}

impl Default for HookState {
    fn default() -> Self {
        // Create a new pipe
        let pipe = SendPipe::create().expect("Cannoli: Failed to create pipe");

        // Connect to the server
        let mut server = TcpStream::connect("127.0.0.1:11458")
            .expect("Cannoli: Failed to connect to Cannoli server");

        // Get QEMU target information
        let qi = QEMU_INFO.get().expect("Cannoli: QEMU_INFO not set!?");

        // Get the current running threads PPID, PID, and TID
        let (ppid, pid, tid) = unsafe {(
            libc::getppid(), libc::getpid(), libc::gettid(),
        )};

        // Get the executable name of the parent
        let pcomm = std::fs::read(format!("/proc/{ppid}/comm"))
            .expect("Cannoli: Failed to read `/proc/<ppid>/comm`");

        // Get the executable name
        let comm = std::fs::read(format!("/proc/{pid}/comm"))
            .expect("Cannoli: Failed to read `/proc/<pid>/comm`");

        // Construct the payload to send to the server
        let header = ClientConn {
            uid:        pipe.uid(),
            arch:       qi.arch       as i32,
            big_endian: qi.big_endian as i32,
            pcomm_len:  pcomm.len()   as u32,
            comm_len:   comm.len()    as u32,
            ppid,
            pid,
            tid,
        };

        // Construct the header
        let mut payload = Vec::new();
        payload.extend_from_slice(unsafe {
            std::slice::from_raw_parts(
                &header as *const ClientConn as *const u8,
                std::mem::size_of_val(&header))
        });

        // Add the parent and self comm values
        payload.extend_from_slice(&pcomm);
        payload.extend_from_slice(&comm);

        // Send the data!
        server.write_all(&payload)
            .expect("Cannoli: Failed to send initial greeting");

        Self {
            active_buffer: None,
            _server: server,
            pipe,
        }
    }
}

/// Global state about the QEMU process we're in. This can only hold values
/// which are constant through execution of the target.
///
/// These are initialized when QEMU calls `query_version`, and thus, only
/// include information about the main thread
#[derive(Debug)]
struct QemuInfo {
    /// The target architecture, converted to the enum from the `UNAME_MACHINE`
    /// define in QEMU
    arch: Architecture,

    /// Tracks if this is a big endian target
    big_endian: bool,
}

/// Global state holding information about the QEMU being used
static QEMU_INFO: OnceLock<QemuInfo> = OnceLock::new();

thread_local! {
    /// The thread-local QEMU hook state
    ///
    /// The `AtomicI32` holds the thread ID of the thread that owns the thread
    /// locals. This sounds silly since they're uhh... thread locals, but when
    /// a fork() occurs with `CLONE_VM`, and no new TLS is set up, then this
    /// value will be _reused_ from a previously initialized state. We want to
    /// detect when we may have forked and need to re-initialize the hook state
    static HOOK_STATE: (AtomicI32, UnsafeCell<RefCell<HookState>>) = (
        AtomicI32::new(unsafe { libc::gettid() }),
        UnsafeCell::new(RefCell::new(HookState::default()))
    );
}

/// Patch a buffer by searching for `magic` and replacing it with `new`
///
/// Panics if no instances of `magic` are found
fn patch(buf: &mut [u8], magic: impl AsRef<[u8]>, new: impl AsRef<[u8]>) {
    // Get the slice versions of the arguments
    let magic = magic.as_ref();
    let new   = new.as_ref();

    // Make sure this usage makes sense
    assert!(magic.len() == new.len() && buf.len() >= magic.len(),
        "Cannoli: Invalid arguments passed to patch()");

    // Find the patch location
    let mut patched = false;
    for ii in 0..buf.len() - (magic.len() - 1) {
        let win = &mut buf[ii..ii + magic.len()];

        if win == magic {
            // Patch if magic matched
            win.copy_from_slice(new);
            patched = true;
        }
    }

    assert!(patched, "Cannoli: Failed to find patch location");
}

/// We might have to re-create the thread locals as sometimes the TLS is not
/// re-initialized on `fork()` (however it is on `pthread_create()`)
fn with_hook<F: FnOnce(RefMut<'_, HookState>)>(callback: F) {
    HOOK_STATE.with(|hook| {
        // Check if the hook state changed threads (fork() without setting a
        // new TLS)
        if hook.0.load(Ordering::Acquire) != unsafe { libc::gettid() } {
            // Re-initialize the hook-state
            unsafe {
                hook.1.get().write(RefCell::new(HookState::default()));
            }

            // Update the thread ID
            hook.0.store(unsafe { libc::gettid() }, Ordering::Release);
        }

        // Okay, now we have an exclusive thread state, invoke the callback
        callback(unsafe { (*hook.1.get()).borrow_mut() });
    });
}

// ============================================================================

/// Byte offset to register state off of `rbp` for the target architecture
static REGISTER_OFFSET: AtomicUsize = AtomicUsize::new(0);

/// Size of the register state for the target architecture
static REGISTER_SIZE: AtomicUsize = AtomicUsize::new(0);

/// Gross macro we use to generate both the 32-bit and 64-bit versions of code
/// for handling QEMU targets of different bitnesses. Unfortunately we kind of
/// have to do this as we don't want the user to have to build different
/// binaries for different workloads.
///
/// Effectively, this macro allows us to specify all the correct symbols and
/// types needed to generate the implementations for a given bitness
///
/// The arguments to this macro:
///
/// - `$tusize`  - The type for a usize on the emulated target. Always either
///                [`u32`] or [`u64`]
/// - `$cannoli` - The correct Cannoli instance for this target, either
///                [`Cannoli32`] or [`Cannoli64`]
/// - `$init`    - Identifier for the initializer for this target
/// - `$lift`    - Identifier for the per-instruction hook for this target
/// - `$entry`   - Identifier for the JIT entry hook for this target
/// - `$exit`    - Identifier for the JIT exit hook for this target
/// - `$mmap`    - Identifier for the callback for mmap()s
/// - `$munmap`  - Identifier for the callback for munmap()s
/// - `$flush`   - Identifier for safe-to-call-from-JIT assembly which performs
///                a "fake" JIT exit and entry to flush the IPC data and get
///                a new buffer.
/// - `$memop`   - Identifier for the memory access hook
macro_rules! create_bitness {
    (
        $tusize:ty, $cannoli:tt, $init:ident, $lift:ident, $entry:ident,
        $exit:ident, $flush:ident, $memop:ident, $mmap:ident, $munmap:ident
    ) => {

/// Called by QEMU to initialize this library, we also return version
/// information to QEMU so it knows that we're using the right version
#[no_mangle]
extern fn $init(arch: *const i8, big_endian: i32, gpr_offset: usize,
        gpr_width: usize, num_gprs: usize) -> &'static $cannoli {
    /// Bindings information for QEMU
    const BINDINGS: $cannoli = $cannoli {
        version:          CANNOLI_VERSION,
        lift_instruction: Some($lift),
        lift_memop:       Some($memop),
        jit_entry:        Some($entry),
        jit_exit:         Some($exit),
        mmap:             Some($mmap),
        munmap:           Some($munmap),
    };

    // Save the register offset and size in the globals.
    REGISTER_OFFSET.store(gpr_offset, Ordering::Relaxed);
    REGISTER_SIZE.store(num_gprs * gpr_width, Ordering::Relaxed);

    // Convert architecture string to enum
    let arch = unsafe { Architecture::from_cstr(arch) };

    // Convert big endian to a bool
    let big_endian = big_endian != 0;

    // Set up global QEMU info
    QEMU_INFO.set(QemuInfo {
        arch,
        big_endian,
    }).expect("Cannoli: Whoa, set QEMU info twice!?");

    &BINDINGS
}

/// Output the host assembly for a given guest instruction at `pc`.
///
/// QEMU gives a temporary buffer pointed to by `buf`, for `buf_size` bytes
/// which is used as instruction storage.
///
/// This function returns the number of bytes to insert into the JIT stream at
/// this instruction start. If it is zero, no additional instructions are
/// generated
#[no_mangle]
unsafe extern fn $lift(pc: $tusize, buf: *mut u8, buf_size: usize) -> usize {
    // Get the requested hook type for this instruction
    let hook_type = hook_inst(pc as u64);

    // Get the start and end address of the shellcode
    //
    // Check the size of `$tusize` to determine the correct shellcode to use
    let (start, end) = match (<$tusize>::BITS, hook_type) {
        (32, HookType::Always) => {
            (
                core::ptr::addr_of!(cannoli_insthook32)     as usize,
                core::ptr::addr_of!(cannoli_insthook32_end) as usize,
            )
        }
        (64, HookType::Always) => {
            (
                core::ptr::addr_of!(cannoli_insthook64)     as usize,
                core::ptr::addr_of!(cannoli_insthook64_end) as usize,
            )
        }
        (32, HookType::Once) => {
            (
                core::ptr::addr_of!(cannoli_insthook32_once)     as usize,
                core::ptr::addr_of!(cannoli_insthook32_once_end) as usize,
            )
        }
        (64, HookType::Once) => {
            (
                core::ptr::addr_of!(cannoli_insthook64_once)     as usize,
                core::ptr::addr_of!(cannoli_insthook64_once_end) as usize,
            )
        }
        (32, HookType::Register) => {
            (
                core::ptr::addr_of!(cannoli_reghook32)     as usize,
                core::ptr::addr_of!(cannoli_reghook32_end) as usize,
            )
        }
        (64, HookType::Register) => {
            (
                core::ptr::addr_of!(cannoli_reghook64)     as usize,
                core::ptr::addr_of!(cannoli_reghook64_end) as usize,
            )
        }
        (_, HookType::Never) => {
            // Don't hook at all
            return 0;
        }
        (_, _) => {
            // At this point we've covered all the types we support
            panic!("Invalid target bitness");
        }
    };

    // Get a slice to the shellcode
    let shellcode = core::slice::from_raw_parts(
        start as *const u8, end - start);

    // Make sure the shellcode will fit in the temporary buffer we got from
    // QEMU
    assert!(shellcode.len() <= buf_size,
        "Cannoli: Exec shellcode too large for QEMU buffer");

    // Initialize the QEMU-based buffer
    buf.copy_from_nonoverlapping(shellcode.as_ptr(), shellcode.len());

    // Create safe, mutable access to the buffer
    let tmp = std::slice::from_raw_parts_mut(buf as *mut u8, shellcode.len());

    // Patch the PC placeholder with the actual PC
    patch(tmp, (REPLACE_WITH_PC as $tusize).to_le_bytes(), pc.to_le_bytes());

    // So, we can't use an address in our shellcode since we don't know that
    // information at compile time. Thus, we replace the `REPLACE_WITH_FLUSH`
    // with the run-time address where that has been loaded
    patch(tmp, REPLACE_WITH_FLUSH.to_le_bytes(),
        ($flush as usize).to_le_bytes());

    // Register hooks have extra patches
    if matches!(hook_type, HookType::Register) {
        // Patch register hook size and offset
        patch(tmp, REPLACE_WITH_REGHOOK_OFFSET.to_le_bytes(),
            (REGISTER_OFFSET.load(Ordering::Relaxed) as u32).to_le_bytes());
        patch(tmp, REPLACE_WITH_REGHOOK_SIZE.to_le_bytes(),
            (REGISTER_SIZE.load(Ordering::Relaxed) as u32).to_le_bytes());
    }

    // Return the size of the shellcode we want to inject
    tmp.len()
}

/// Invoked from QEMU when entering the JIT. This provides an opportunity for
/// us to introduce some register state to the JIT.
///
/// `out_regs` points to an array of 3 `size_t`s which should be filled with
/// the values for `r12`, `r13`, and `r14`, respectively
#[no_mangle]
unsafe extern fn $entry(out_regs: *mut usize) {
    // Make sure the hook state is thread-local
    with_hook(|mut hook| {
        // Make sure we didn't do a double entry. This is to catch places where
        // we might exit the JIT without calling JIT exit, indicating where we
        // need more QEMU hooks
        assert!(hook.active_buffer.is_none(),
            "Cannoli: Whoa, got JIT entry without a JIT exit!");

        // Allocate a new buffer in our pipe
        let mut buffer = hook.pipe.alloc_buffer();

        // Populate `r12`, `r13` and `r14` with:
        //
        // r12 - Pointer to the current free byte in the output buffer
        // r13 - Pointer to the end of the output buffer
        // r14 - Zero, used as scratch in the JIT
        let (r12, r13, r14) = (
            buffer.get_raw() as usize,
            buffer.get_raw() as usize + CHUNK_SIZE,
            0,
        );

        // Store this as the active buffer, also switch the lifetime to static
        hook.active_buffer = Some(
            ManuallyDrop::new(core::mem::transmute(buffer))
        );

        // Write the register states requested
        out_regs.offset(0).write(r12);
        out_regs.offset(1).write(r13);
        out_regs.offset(2).write(r14);
    });
}

/// Invoked from QEMU when exiting the JIT. This is then provided with the
/// values of `r12`, `r13`, and `r14` upon exit of the JIT, giving the user an
/// opportunity to observe the changes to the registers
#[no_mangle]
unsafe extern fn $exit(r12: usize, _r13: usize, _r14: usize) {
    with_hook(|mut hook| {
        // QEMU is duct-taped together with longjmps and these can happen
        // inside the JIT. We're trying to carefully hook everything that can
        // possibly exit the JIT, and sometimes we get it wrong :(
        // Blame QEMU for making this impossible to do :(
        assert!(r12 as u64 != CANNOLI_POISON,
            "Cannoli: Oh no! Cannoli hit a poisoned JIT exit. This means that \
             something in QEMU exited the JIT that we do not hook. This means \
             that the trace is no longer valid as it may not have been \
             flushed");

        // Get the active buffer and replace it with `None`
        let ab = hook.active_buffer.take()
            .expect("Cannoli: JIT active buffer missing");

        // We allow dropping of the buffer now
        let mut ab = ManuallyDrop::into_inner(ab);
        let to_send = r12 - ab.get_raw() as usize;
        ab.send_raw(to_send);
    });
}

/// Invoked when QEMU is lifting a memory operation. Similar to the
/// execution hook, this function is provided with a `buf` and a
/// `buf_size` which are to be populated with shellcode to inject into the
/// JIT stream. The return value from this function is the number of bytes
/// written.
///
/// This shellcode is injected prior to writes, and after reads, allowing
/// the value that is being read/written to be observed
///
/// This function is called with the parameters:
///
/// - `pc`       - Target program counter associated with this operation
/// - `is_write` - `0` if this is a read, `1` if this is a write
/// - `data_reg` - x86_64 register index to the register which holds the
///                value that was read/written
/// - `addr_reg` - x86_64 register index to the register which holds the
///                emulated guest's address that is being accessed
/// - `memop`    - One of the QEMU memops from `MemOp` (eg. `MO_8`). This
///                tells us the size of the operation
/// - `buf`      - Pointer to QEMU-allocated memory for where to copy
///                shellcode
/// - `buf_size` - Size of `buf` in bytes
unsafe extern fn $memop(pc: $tusize, is_write: i32, data_reg: usize,
        addr_reg: usize, memop: i32, buf: *mut u8, buf_size: usize) -> usize {
    /// QEMU 8-bit memory operation
    const MO_8:  i32 = 0;

    /// QEMU 16-bit memory operation
    const MO_16: i32 = 1;

    /// QEMU 32-bit memory operation
    const MO_32: i32 = 2;

    /// QEMU 64-bit memory operation
    const MO_64: i32 = 3;

    // Make sure the `memop` and `is_write` parameters match the strict
    // expectations we have of them.
    assert!(matches!(memop, MO_8 | MO_16 | MO_32 | MO_64),
        "Cannoli: Unsupported memop");
    assert!(matches!(is_write, 0 | 1), "Cannoli unsupported is_write value");

    // Make sure the register values make sense
    assert!(data_reg < 16, "Cannoli: Invalid data_reg input to memop");
    assert!(addr_reg < 16, "Cannoli: Invalid addr_reg input to memop");

    // Do nothing if the hook doesn't want to hook this operation
    let memsize = [1, 2, 4, 8];
    if !hook_mem(pc as u64, is_write != 0, memsize[memop as usize]) {
        return 0;
    }

    // Get the start and end of the shellcode for the respective memory hook
    // with these given parameters
    //
    // Memory hook table, indexed by:
    //   `MEMHOOK_TABLE[bitness][access_width][access_type][data][addr]`
    // where `bitness` is 0 for 32-bit and 1 for 64-bit, and `data` and `addr`
    // being the register indicies that currently holds those values.
    //
    // `bitness`, the size of the emulated target's `usize`
    //   0 - 32-bit
    //   1 - 64-bit
    //
    // `access_width`:
    //   0 - 8-bit
    //   1 - 16-bit
    //   2 - 32-bit
    //   3 - 64-bit
    //
    // `access_type`:
    //   0 - read
    //   1 - write
    let (start, end) = crate::cannoli_memops::MEMHOOK_TABLE
        [size_of::<$tusize>() / 4 - 1][memop as usize]
        [is_write as usize][data_reg][addr_reg];

    // Convert the addresses to `usize`s
    let (start, end) =
        (start as *const u8 as usize, end as *const u8 as usize);

    // Get a slice to the shellcode
    let shellcode = core::slice::from_raw_parts(
        start as *const u8, end - start);

    // Make sure the shellcode will fit in the temporary buffer we got from
    // QEMU
    assert!(shellcode.len() <= buf_size,
        "Cannoli: Memop shellcode too large for QEMU buffer");

    // Initialize the QEMU-based buffer
    buf.copy_from_nonoverlapping(shellcode.as_ptr(), shellcode.len());

    // Create access to buffer
    let tmp = std::slice::from_raw_parts_mut(buf as *mut u8, shellcode.len());

    // Patch the PC placeholder with the actual PC
    patch(tmp, (REPLACE_WITH_PC as $tusize).to_le_bytes(), pc.to_le_bytes());

    // So, we can't use an address in our shellcode since we don't know that
    // information at compile time. Thus, we replace the `REPLACE_WITH_FLUSH`
    // with the run-time address where that has been loaded
    patch(tmp, REPLACE_WITH_FLUSH.to_le_bytes(),
        ($flush as usize).to_le_bytes());

    tmp.len()
}

/// Called _directly_ from the JIT without preserving any registers. We have
/// to preserve all registers in the JIT, this is not a standard extern FFI!
///
/// This is the code that is invoked by the calls to `REPLACE_WITH_FLUSH`!
///
/// This is normal assembly, not shellcode, and won't be copied or moved around
/// so we can actually use symbols in it! Yay!
#[naked]
unsafe extern fn $flush() {
    std::arch::asm!(r#"
        // x86-64 SYS-V ABI:
        // Callee-saved: rbp, rbx, r12, r13, r14, r15, rsp
        // Caller-saved: rax, rdi, rsi, rdx, rcx, r8, r9, r10, r11

        // Save all registers that aren't preserved by our callees
        push rax
        push rdi
        push rsi
        push rdx
        push rcx
        push r8
        push r9
        push r10
        push r11

        // Allocate some space on the stack for args and align it
        push rbp
        mov  rbp, rsp
        sub  rsp, 8 * 3
        and  rsp, ~0xf

        // Load up the arguments and call into the exit routine, this forces
        // a flush of the buffer
        mov  rdi, r12
        mov  rsi, r13
        mov  rdx, r14
        call {exit}

        // Load a pointer to the stack where the three arguments are to be
        // stored
        mov  rdi, rsp
        call {entry}

        // Load the registers specified by the entry
        mov r12, [rsp + 0x00]
        mov r13, [rsp + 0x08]

        // Don't update r14 as when we flush we need to preserve the r14
        // register which still holds the address being read
        //mov r14, [rsp + 0x10]

        // Restore the stack
        mov rsp, rbp
        pop rbp

        // Restore all registers that aren't preserved by our callees
        pop r11
        pop r10
        pop r9
        pop r8
        pop rcx
        pop rdx
        pop rsi
        pop rdi
        pop rax
        ret
    "#, entry = sym $entry, exit = sym $exit, options(noreturn));
}

/// Called on successful mappings
#[no_mangle]
unsafe extern fn $mmap(start: $tusize, len: $tusize,
        anon: i32, read: i32, write: i32, exec: i32, path: *mut i8,
        offset: $tusize) {
    // Make sure the hook state is thread-local
    with_hook(|mut hook| {
        // Shouldn't have an active buffer
        assert!(hook.active_buffer.is_none(), "mmap from inside the JIT?");

        // Allocate a new buffer in our pipe
        let buffer = hook.pipe.alloc_buffer();

        // Convert the path into bytes
        let path: &[u8] = if path.is_null() {
            &[]
        } else {
            CStr::from_ptr(path).to_bytes()
        };

        // Temporary vector for building packet
        let mut tmp = Vec::new();

        // Opcode
        tmp.push(if <$tusize>::BITS == 64 { 0xb0 } else { 0x30 });

        // Parameters
        tmp.extend_from_slice(&start.to_le_bytes());
        tmp.extend_from_slice(&len.to_le_bytes());
        tmp.push(anon  as u8);
        tmp.push(read  as u8);
        tmp.push(write as u8);
        tmp.push(exec  as u8);
        tmp.extend_from_slice(&(path.len() as u32).to_le_bytes());
        tmp.extend_from_slice(&offset.to_le_bytes());
        tmp.extend_from_slice(path);

        // Send the payload
        buffer.send(tmp);
    });
}

/// Called on successful mappings
#[no_mangle]
unsafe extern fn $munmap(start: $tusize, len: $tusize) {
    // Make sure the hook state is thread-local
    with_hook(|mut hook| {
        // Shouldn't have an active buffer
        assert!(hook.active_buffer.is_none(), "munmap from inside the JIT?");

        // Allocate a new buffer in our pipe
        let buffer = hook.pipe.alloc_buffer();

        // Temporary vector for building packet
        let mut tmp = Vec::new();

        // Opcode
        tmp.push(if <$tusize>::BITS == 64 { 0xb1 } else { 0x31 });

        // Parameters
        tmp.extend_from_slice(&start.to_le_bytes());
        tmp.extend_from_slice(&len.to_le_bytes());

        // Send the payload
        buffer.send(tmp);
    });
}

}} // macro_rules!

// ============================================================================

// Declare some symbols we defined in our global assembly below
extern {
    static cannoli_insthook32:          u8;
    static cannoli_insthook32_end:      u8;
    static cannoli_insthook64:          u8;
    static cannoli_insthook64_end:      u8;
    static cannoli_insthook32_once:     u8;
    static cannoli_insthook32_once_end: u8;
    static cannoli_insthook64_once:     u8;
    static cannoli_insthook64_once_end: u8;
    static cannoli_reghook32:           u8;
    static cannoli_reghook32_end:       u8;
    static cannoli_reghook64:           u8;
    static cannoli_reghook64_end:       u8;
}

/// Magic value to replace with the address of the respective `flush_buffer`
/// function
const REPLACE_WITH_FLUSH: usize = 0xa03e2cd1b94c78fd;

/// Magic value to replace with the current instructions PC
const REPLACE_WITH_PC: usize = 0xcc5fe07bf3cfe384;

/// Magic value to replace with the register byte offset off of rbp
const REPLACE_WITH_REGHOOK_OFFSET: u32 = 0x3fcc88a3;

/// Magic value to replace with the register state size
const REPLACE_WITH_REGHOOK_SIZE: u32 = 0x652a1e21;

// All of our shellcode is written in this global assembly block, and it is
// ripped out and placed into the JIT. It's kinda neat. It seems ugly, but I
// think this is way easier to make tweaks to than some weird assembler at
// runtime
std::arch::global_asm!(r#"

// ============================================================================

// Macro invoked when creating an instruction hook. This is where we log the PC
// of the instruction being executed into the buffer!
//
// bits  - The bitness of the emulated target, either 32 or 64
// width - The bitness divided by eight (number of bytes per target usize)
// once  - Determines if this is a single-shot instruction hook, or an always
//         hook
.macro create_insthook bits, width, once

// This code is injected _directly_ into QEMUs JIT, we have to make sure we
// don't touch _any_ registers without preserving them
.global cannoli_insthook\bits\()\once\()
cannoli_insthook\bits\()\once\():
    // r12 - Pointer to trace buffer
    // r13 - Pointer to end of trace buffer
    // r14 - Scratch

.ifnb \once
    // Clear the zero flag
    xor r14d, r14d

    // Conditionally branch to the end of code
    jnz 10f

    // Replace branch above with a `jmp`
    mov byte ptr [rip - 9], 0xeb
.endif

    // Allocate room in the buffer
    lea r14, [r12 + \width + 1]

    // Make sure we didn't run out of buffer space
    cmp r14, r13
    jbe 2f

    // We're out of space! This happens "rarely", only when the buffer is full,
    // so we can do much more complex work here. We can also save and restore
    // some registers.
    //
    // We directly call into our Rust to reduce the icache pollution and to get
    // some code sharing for the much more complex flushing operation
    //
    // Flushing gets us a new r12, r13, and r14
    mov  r13, {REPLACE_WITH_FLUSH}
    call r13

2:
.if \bits == 32
    // Opcode
    mov byte ptr [r12], 0x00

    // PC, directly put into memory from an immediate
    mov dword ptr [r12 + 1], {REPLACE_WITH_PC}
.elseif \bits == 64
    // Opcode
    mov byte ptr [r12], 0x80

    // Move PC into a register so we can use imm64 encoding
    mov r14, {REPLACE_WITH_PC}
    mov qword ptr [r12 + 1], r14
.else
.error "Invalid bitness passed to create_insthook"
.endif

    // Advance buffer
    add r12, \width + 1

    // End of hook
10:

.global cannoli_insthook\bits\()\once\()_end
cannoli_insthook\bits\()\once\()_end:

.endm // create_insthook

// Create both the 32-bit and 64-bit instruction hooks
create_insthook 32, 4
create_insthook 64, 8
create_insthook 32, 4, _once
create_insthook 64, 8, _once

// ============================================================================

// Okay. This macro is gnarly. This defines the shellcode we use for our memory
// hooks. Unlike the PC shellcode, we actually have 2 register inputs from
// QEMU's JIT. These registers could be "any" register that is scheduled to the
// JIT. Thus, we have to create different shellcode templates for every
// combination of registers.
//
// So, we've made this macro. The way this macro is invoked is disgusting, but
// the macro itself is pretty clean. Inside the macro you have a few different
// values you can access:
//
// \access    - Either 'read' or 'write' (no quotes) depending on the operation
//              type
// \datawidth - The size of the read/write being performed (1, 2, 4, or 8)
// \width     - The size of the target's usize, in bytes (4 or 8)
// \data      - The register name of the register which holds the data that
//              was read/written
// \addr      - The register name of the register which holds the address
.macro create_memhook access, datawidth, width, data, addr
.global cannoli_memhook_\access\()_\data\()_\addr\()
cannoli_memhook_\access\()_\data\()_\addr\():
    // r12 - Pointer to trace buffer
    // r13 - Pointer to end of trace buffer
    // r14 - For reads, this always holds the address, for writes, it's scratch

    // Allocate room in the buffer (we have to preserve r14 here)
    lea r12, [r12 + (\width * 2 + \datawidth + 1)]
    cmp r12, r13
    lea r12, [r12 - (\width * 2 + \datawidth + 1)]
    jbe 2f

    // We're out of space! This happens "rarely", only when the buffer is full,
    // so we can do much more complex work here. We can also save and restore
    // some registers.
    //
    // We directly call into our Rust to reduce the icache pollution and to get
    // some code sharing for the much more complex flushing operation
    //
    // Flushing gets us a new r12, r13, and r14
    mov  r13, {REPLACE_WITH_FLUSH}
    call r13

2:
.ifc \access, read
    // Read opcode
    mov byte ptr [r12], (((\width / 4) - 1) << 7) | 0x10 | \datawidth
.endif
.ifc \access, write
    // Write opcode
    mov byte ptr [r12], (((\width / 4) - 1) << 7) | 0x20 | \datawidth
.endif

    // Address and data
    mov [r12          + 1], \addr
    mov [r12 + \width + 1], \data

    // Store the PC
.if \width == 4
    mov dword ptr [r12 + \width + 1 + \datawidth], {REPLACE_WITH_PC}
.elseif \width == 8
    mov r14, {REPLACE_WITH_PC}
    mov [r12 + \width + 1 + \datawidth], r14
.endif

    // Advance buffer
    add r12, \width * 2 + \datawidth + 1

.global cannoli_memhook_\access\()_\data\()_\addr\()_end
cannoli_memhook_\access\()_\data\()_\addr\()_end:
.endm // create_memhook

// Macro invoked when creating an register hook.
//
// bits  - The bitness of the emulated target, either 32 or 64
// width - The bitness divided by eight (number of bytes per target usize)
.macro create_reghook bits, width

.global cannoli_reghook\bits\()
cannoli_reghook\bits\():
    // Determine size required for register hook metadata
    lea r14, [r12 + 1 + 4 + \width]
    add r14, {REPLACE_WITH_REGHOOK_SIZE}

    // Make sure we're in bounds
    cmp r14, r13
    jbe 2f

    // We're out of space! This happens "rarely", only when the buffer is full,
    // so we can do much more complex work here. We can also save and restore
    // some registers.
    //
    // We directly call into our Rust to reduce the icache pollution and to get
    // some code sharing for the much more complex flushing operation
    //
    // Flushing gets us a new r12, r13, and r14
    mov  r13, {REPLACE_WITH_FLUSH}
    call r13

2:
.if \bits == 32
    // Opcode
    mov byte ptr [r12], 0x01

    // Size of payload
    mov dword ptr [r12 + 1], {REPLACE_WITH_REGHOOK_SIZE}

    // PC, directly put into memory from an immediate
    mov dword ptr [r12 + 1 + 4], {REPLACE_WITH_PC}
.elseif \bits == 64
    // Opcode
    mov byte ptr [r12], 0x81

    // Size of payload
    mov dword ptr [r12 + 1], {REPLACE_WITH_REGHOOK_SIZE}

    // Move PC into a register so we can use imm64 encoding
    mov r14, {REPLACE_WITH_PC}
    mov qword ptr [r12 + 1 + 4], r14
.else
.error "Invalid bitness passed to cannoli_reghook"
.endif

    // Fill in the registers
    push rdi
    push rsi
    push rcx
    lea rdi, [r12 + \width + 1 + 4]
    lea rsi, [rbp + {REPLACE_WITH_REGHOOK_OFFSET}]
    mov ecx, {REPLACE_WITH_REGHOOK_SIZE}
    rep movsb
    pop rcx
    pop rsi
    pop rdi

    // Advance buffer
    add r12, \width + 1 + 4
    add r12, {REPLACE_WITH_REGHOOK_SIZE}

.global cannoli_reghook\bits\()_end
cannoli_reghook\bits\()_end:

.endm // create_reghook

create_reghook 32, 4
create_reghook 64, 8

// ===========================================================================
// !!! WARNING !!!
//
// Don't look below, the code is disgusting. This code generates all of the
// possible combinations of the memory operations. Based on bitness, operation
// size, 2 register inputs, idk probably some other stuff.
//
// It might look gross, but honestly, I think it gives us a really cool
// environment above to write the memory hook shellcode. Thus, don't complain
// about it. Go away.
// ===========================================================================

// For each `addr` in `regs`, create the read and write memhooks. Using the
// `addr` as the address register name when creating the code
.macro multiple_create_memhook_int datawidth, width, data, addr, regs:vararg
    // Create the memhook
    create_memhook read,  \datawidth, \width, \data, \addr
    create_memhook write, \datawidth, \width, \data, \addr

    // Continue creating memhooks until we're out of regs
    .ifnb \regs
        multiple_create_memhook_int \datawidth, \width, \data, \regs
    .endif
.endm // multiple_create_memhook_int

// For each `data` in `regs`, this extracts the register name to use for the
// data argument. This generates the memhooks for 32-bit usize targets
.macro multiple_create_memhook32 datawidth, reg, regs:vararg
    multiple_create_memhook_int \datawidth, 4, \reg, eax, ecx, edx, ebx, esp, ebp, esi, edi, r8d, r9d, r10d, r11d, r12d, r13d, r14d, r15d

    // Continue creating memhooks until we're out of regs
    .ifnb \regs
        multiple_create_memhook32 \datawidth, \regs
    .endif
.endm

// For each `data` in `regs`, this extracts the register name to use for the
// data argument. This generates the memhooks for 64-bit usize targets
.macro multiple_create_memhook64 datawidth, reg, regs:vararg
    multiple_create_memhook_int \datawidth, 8, \reg, rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15

    // Continue creating memhooks until we're out of regs
    .ifnb \regs
        multiple_create_memhook64 \datawidth, \regs
    .endif
.endm

// Create all possible memhooks for 32-bit
multiple_create_memhook32 1, al, cl, dl, bl, spl, bpl, sil, dil, r8b, r9b, r10b, r11b, r12b, r13b, r14b, r15b
multiple_create_memhook32 2, ax, cx, dx, bx, sp, bp, si, di, r8w, r9w, r10w, r11w, r12w, r13w, r14w, r15w
multiple_create_memhook32 4, eax, ecx, edx, ebx, esp, ebp, esi, edi, r8d, r9d, r10d, r11d, r12d, r13d, r14d, r15d
multiple_create_memhook32 8, rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15

// Create all possible memhooks for 64-bit
multiple_create_memhook64 1, al, cl, dl, bl, spl, bpl, sil, dil, r8b, r9b, r10b, r11b, r12b, r13b, r14b, r15b
multiple_create_memhook64 2, ax, cx, dx, bx, sp, bp, si, di, r8w, r9w, r10w, r11w, r12w, r13w, r14w, r15w
multiple_create_memhook64 4, eax, ecx, edx, ebx, esp, ebp, esi, edi, r8d, r9d, r10d, r11d, r12d, r13d, r14d, r15d
multiple_create_memhook64 8, rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15

"#,
    // Some magic values we use in our assembly for find-and-replace
    REPLACE_WITH_FLUSH = const REPLACE_WITH_FLUSH,
    REPLACE_WITH_PC    = const REPLACE_WITH_PC,

    REPLACE_WITH_REGHOOK_SIZE   = const REPLACE_WITH_REGHOOK_SIZE,
    REPLACE_WITH_REGHOOK_OFFSET = const REPLACE_WITH_REGHOOK_OFFSET,
);

// Create the 32-bit Cannoli implementation
create_bitness!(
    u32, Cannoli32, init_cannoli32, lift_instruction32, jit_entry32,
    jit_exit32, cannoli_flush_buffer32, lift_memop32,
    cannoli_mmap32, cannoli_munmap32
);

// Create the 64-bit Cannoli implementation
create_bitness!(
    u64, Cannoli64, init_cannoli64, lift_instruction64, jit_entry64,
    jit_exit64, cannoli_flush_buffer64, lift_memop64,
    cannoli_mmap64, cannoli_munmap64
);

