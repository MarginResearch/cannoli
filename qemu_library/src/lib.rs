//! A small shared library which connects to small stubs patched into QEMU
//! and provides real-time streaming of things like PCs, register state, and
//! memory accesses

#![feature(asm_const, maybe_uninit_slice, inline_const, naked_functions)]
#![feature(asm_sym)]

mod cannoli_memops;

use std::io::Write;
use std::net::TcpStream;
use std::mem::{ManuallyDrop, size_of, size_of_val};
use std::cell::RefCell;
use mempipe::{SendPipe, ChunkWriter};

/// Chunk size to use when streaming data over IPC
const CHUNK_SIZE: usize = 22 * 1024;

/// Number of chunks to use with IPC
const NUM_BUFFERS: usize = 8;

// Pull in the FFI bindings we generated
include!(concat!(env!("OUT_DIR"), "/ffi_bindings.rs"));

/// Opcodes we send over the IPC
#[repr(C)]
enum Opcode {
    /// A `u32` follows, which is the PC which started execution
    Exec32,

    /// A `u64` follows, which is the PC which started execution
    Exec64,

    Read8_32,
    Read16_32,
    Read32_32,
    Write8_32,
    Write16_32,
    Write32_32,

    Read8_64,
    Read16_64,
    Read32_64,
    Read64_64,
    Write8_64,
    Write16_64,
    Write32_64,
    Write64_64,
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
        let pipe = SendPipe::create().expect("Failed to create pipe");

        // Connect to the server
        let mut server = TcpStream::connect("127.0.0.1:11458")
            .expect("Failed to connect to Cannoli server");

        // Send the uid to the server
        server.write_all(&pipe.uid().to_le_bytes())
            .expect("Failed to send UID to server");

        Self {
            active_buffer: None,
            _server: server,
            pipe,
        }
    }
}

thread_local! {
    /// The thread-local QEMU hook state
    static HOOK_STATE: RefCell<HookState> = RefCell::new(HookState::default());
}

/// Gross macro we use to generate both the 32-bit and 64-bit versions of code
/// for handling QEMU targets of different bitnesses. Unfortunately we kind of
/// have to do this as we don't want the user to have to build different
/// binaries for different workloads.
///
/// Effectively, this macro allows us to specify all the correct symbols and
/// types needed to generate the implementations for a given bitness
macro_rules! create_bitness {
    (
        $tusize:ty, $cannoli:tt, $init:ident, $lift:ident, $entry:ident,
        $exit:ident, $hook:ident, $hook_end:ident, $flush:ident, $memop:ident,
    ) => {

/// Called by QEMU to initialize this library, we also return version
/// information to QEMU so it knows that we're using the right version
#[no_mangle]
extern fn $init() -> &'static $cannoli {
    /// Bindings information for QEMU
    const BINDINGS: $cannoli = $cannoli {
        version:          CANNOLI_VERSION,
        lift_instruction: Some($lift),
        lift_memop:       Some($memop),
        jit_entry:        Some($entry),
        jit_exit:         Some($exit),
    };

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
    // Get the start and end address of the shellcode
    let (start, end) = (
        core::ptr::addr_of!($hook)     as usize,
        core::ptr::addr_of!($hook_end) as usize,
    );

    // Get a slice to the shellcode
    let shellcode = core::slice::from_raw_parts(
        start as *const u8, end - start);

    // Make sure the shellcode will fit in the temporary buffer we got from
    // QEMU
    assert!(shellcode.len() <= buf_size,
        "Cannoli: Shellcode too large for QEMU buffer");

    // Initialize the buffer and remove the `MaybeUninit`
    buf.copy_from_nonoverlapping(shellcode.as_ptr(), shellcode.len());

    // Create access to buffer
    let tmp = std::slice::from_raw_parts_mut(buf as *mut u8, shellcode.len());

    // Find the location we want to patch with the opcode
    let patch = shellcode.windows(5)
        .position(|x| x == b"\x41\xc6\x04\x24\x69")
        .expect("Failed to find patch location");
    let opcode = &mut tmp[patch + 4];

    // Determine the opcode based on this instruction type
    *opcode = if size_of::<$tusize>() == 4 {
        Opcode::Exec32 as u8
    } else {
        Opcode::Exec64 as u8
    };

    // Find the location we want to patch with PC
    let patch = shellcode.windows(size_of::<$tusize>())
        .position(|x| x == (REPLACE_WITH_PC as $tusize).to_le_bytes())
        .expect("Failed to find patch location");
    tmp[patch..][..size_of::<$tusize>()].copy_from_slice(&pc.to_le_bytes());

    // Find the location we want to patch with the flush buffer function
    let patch = shellcode.windows(size_of_val(&REPLACE_WITH_FLUSH))
        .position(|x| x == REPLACE_WITH_FLUSH.to_le_bytes())
        .expect("Failed to find patch location");
    tmp[patch..][..size_of_val(&REPLACE_WITH_FLUSH)]
        .copy_from_slice(&($flush as usize).to_le_bytes());

    shellcode.len()
}

/// Invoked from QEMU when entering the JIT. This provides an opportunity for
/// us to introduce some register state to the JIT.
///
/// `out_regs` points to an array of 3 `size_t`s which should be filled with
/// the values for `r12`, `r13`, and `r14`, respectively
#[no_mangle]
unsafe extern fn $entry(out_regs: *mut usize) {
    // Invoke Rust-level JIT entry hook
    HOOK_STATE.with(|hook| {
        // Get mutable access to the hook state
        let mut hook = hook.borrow_mut();

        // Allocate a new buffer in our pipe
        let mut buffer = hook.pipe.alloc_buffer();

        // Inform the hook that we entered the JIT, requesting
        // the values we should put in registers `r12`, `r13`, and
        // `r14` upon JIT entry
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
    // Invoke Rust-level JIT exit hook
    HOOK_STATE.with(|hook| {
        // Get mutable access to the hook state
        let mut hook = hook.borrow_mut();

        // Get the active buffer
        let ab = hook.active_buffer.take().expect("JIT active buffer missing");

        // We allow dropping of the buffer now
        let mut ab = ManuallyDrop::into_inner(ab);
        let to_send = r12 - ab.get_raw() as usize;
        ab.send_raw(to_send);
    });
}

unsafe extern fn $memop(is_write: i32, data_reg: usize, addr_reg: usize,
        memop: i32, buf: *mut u8, buf_size: usize) -> usize {
    // Get the start and end of the shellcode
    let (start, end) = cannoli_memops::MEMHOOK_TABLE[0][data_reg][addr_reg];

    // Convert the addresses to `usize`s
    let (start, end) =
        (start as *const u8 as usize, end as *const u8 as usize);

    // Get a slice to the shellcode
    let shellcode = core::slice::from_raw_parts(
        start as *const u8, end - start);

    // Make sure the shellcode will fit in the temporary buffer we got from
    // QEMU
    assert!(shellcode.len() <= buf_size,
        "Cannoli: Shellcode too large for QEMU buffer");

    // Initialize the buffer and remove the `MaybeUninit`
    buf.copy_from_nonoverlapping(shellcode.as_ptr(), shellcode.len());

    // Create access to buffer
    let tmp = std::slice::from_raw_parts_mut(buf as *mut u8, shellcode.len());

    // Find the location we want to patch with the opcode
    let patch = shellcode.windows(5)
        .position(|x| x == b"\x41\xc6\x04\x24\x69")
        .expect("Failed to find patch location");
    let opcode = &mut tmp[patch + 4];

    // QEMU memops:
    const MO_8:  i32 = 0;
    const MO_16: i32 = 1;
    const MO_32: i32 = 2;
    const MO_64: i32 = 3;

    // Get the opcode given the request
    *opcode = match (is_write != 0, memop, size_of::<$tusize>()) {
        (false, MO_8,  4) => Opcode::Read8_32   as u8,
        (false, MO_16, 4) => Opcode::Read16_32  as u8,
        (false, MO_32, 4) => Opcode::Read32_32  as u8,
        (true,  MO_8,  4) => Opcode::Write8_32  as u8,
        (true,  MO_16, 4) => Opcode::Write16_32 as u8,
        (true,  MO_32, 4) => Opcode::Write32_32 as u8,
        (false, MO_8,  8) => Opcode::Read8_64   as u8,
        (false, MO_16, 8) => Opcode::Read16_64  as u8,
        (false, MO_32, 8) => Opcode::Read32_64  as u8,
        (false, MO_64, 8) => Opcode::Read64_64  as u8,
        (true,  MO_8,  8) => Opcode::Write8_64  as u8,
        (true,  MO_16, 8) => Opcode::Write16_64 as u8,
        (true,  MO_32, 8) => Opcode::Write32_64 as u8,
        (true,  MO_64, 8) => Opcode::Write64_64 as u8,
        _ => panic!("Cannoli: Unexpected memory combination"),
    };

    // Find the location we want to patch with the flush buffer function
    let patch = shellcode.windows(size_of_val(&REPLACE_WITH_FLUSH))
        .position(|x| x == REPLACE_WITH_FLUSH.to_le_bytes())
        .expect("Failed to find patch location");
    tmp[patch..][..size_of_val(&REPLACE_WITH_FLUSH)]
        .copy_from_slice(&($flush as usize).to_le_bytes());

    shellcode.len()
}

/// Called _directly_ from the JIT without preserving any registers. We have
/// to preserve all registers in the JIT, this is not a standard extern FFI!
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
        mov r14, [rsp + 0x10]

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
}} // macro_rules!

extern {
    static cannoli_hook32:     u8;
    static cannoli_hook32_end: u8;
    static cannoli_hook64:     u8;
    static cannoli_hook64_end: u8;
}

/// Magic value to replace with the address of the respective `flush_buffer`
/// function
const REPLACE_WITH_FLUSH: u64 = 0xa03e2cd1b94c78fd;

/// Magic value to replace with the current instructions PC
const REPLACE_WITH_PC: u64 = 0xcc5fe07bf3cfe384;

std::arch::global_asm!(r#"

// ============================================================================

// This code is injected _directly_ into QEMUs JIT, we have to make sure we
// don't touch _any_ registers without preserving them
.global cannoli_hook32
cannoli_hook32:
    // r12 - Pointer to trace buffer
    // r13 - Pointer to end of trace buffer
    // r14 - Scratch

    // Allocate room in the buffer
    lea r14, [r12 + 5]

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
    // Opcode
    mov byte ptr [r12], 0x69

    // PC
    mov dword ptr [r12 + 1], {REPLACE_WITH_PC}

    // Advance buffer
    add r12, 5

.global cannoli_hook32_end
cannoli_hook32_end:

// ============================================================================

// This code is injected _directly_ into QEMUs JIT, we have to make sure we
// don't touch _any_ registers without preserving them
.global cannoli_hook64
cannoli_hook64:
    // r12 - Pointer to trace buffer
    // r13 - Pointer to end of trace buffer
    // r14 - Scratch

    // Allocate room in the buffer
    lea r14, [r12 + 9]

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
    // Opcode
    mov byte ptr [r12], 0x69

    // PC
    mov r14, {REPLACE_WITH_PC}
    mov qword ptr [r12 + 1], r14

    // Advance buffer
    add r12, 9

.global cannoli_hook64_end
cannoli_hook64_end:

// ============================================================================

.macro create_memhook width, data, addr
.global cannoli_memhook_\data\()_\addr
cannoli_memhook_\data\()_\addr:
    // r12 - Pointer to trace buffer
    // r13 - Pointer to end of trace buffer
    // r14 - For reads, this always holds the address, for writes, it's scratch

    // Allocate room in the buffer
    lea r12, [r12 + (\width * 2 + 1)]
    cmp r12, r13
    lea r12, [r12 - (\width * 2 + 1)]
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
    // Opcode
    mov byte ptr [r12], 0x69

    // Address and data
    mov [r12 +          1], \addr
    mov [r12 + \width + 1], \data

    // Advance buffer
    add r12, \width * 2 + 1

.global cannoli_memhook_\data\()_\addr\()_end
cannoli_memhook_\data\()_\addr\()_end:
.endm

.macro multiple_create_memhook_int width, data, addr, regs:vararg
    // Create the memhook
    create_memhook \width, \data, \addr

    // Continue creating memhooks until we're out of regs
    .ifnb \regs
        multiple_create_memhook_int \width, \data, \regs
    .endif
.endm

.macro multiple_create_memhook32 reg, regs:vararg
    multiple_create_memhook_int 4, \reg, eax, ecx, edx, ebx, esp, ebp, esi, edi, r8d, r9d, r10d, r11d, r12d, r13d, r14d, r15d

    // Continue creating memhooks until we're out of regs
    .ifnb \regs
        multiple_create_memhook32 \regs
    .endif
.endm

.macro multiple_create_memhook64 reg, regs:vararg
    multiple_create_memhook_int 8, \reg, rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15

    // Continue creating memhooks until we're out of regs
    .ifnb \regs
        multiple_create_memhook64 \regs
    .endif
.endm

multiple_create_memhook32 eax, ecx, edx, ebx, esp, ebp, esi, edi, r8d, r9d, r10d, r11d, r12d, r13d, r14d, r15d
multiple_create_memhook64 rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15

"#,
    REPLACE_WITH_FLUSH = const REPLACE_WITH_FLUSH,
    REPLACE_WITH_PC    = const REPLACE_WITH_PC,
);

create_bitness!(
    u32, Cannoli32, init_cannoli32, lift_instruction32, jit_entry32,
    jit_exit32, cannoli_hook32, cannoli_hook32_end, cannoli_flush_buffer32,
    lift_memop32,
);

create_bitness!(
    u64, Cannoli64, init_cannoli64, lift_instruction64, jit_entry64,
    jit_exit64, cannoli_hook64, cannoli_hook64_end, cannoli_flush_buffer64,
    lift_memop64,
);

