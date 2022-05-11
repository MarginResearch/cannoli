//! These are the deep internals for Cannoli and handle direct callbacks
//! between QEMU, QEMU's JIT, and Rust
//!
//! Edit this _very carefully_

use std::cell::RefCell;
use std::mem::{ManuallyDrop, size_of, size_of_val};
use mempipe::{SendPipe, ChunkWriter};

/// Chunk size to use when streaming data over IPC
const CHUNK_SIZE: usize = 16 * 1024;

/// Number of chunks to use with IPC
const NUM_BUFFERS: usize = 16;

// Pull in the FFI bindings we generated
include!(concat!(env!("OUT_DIR"), "/ffi_bindings.rs"));

struct HookState {
    /// Pipe to use to send data out of QEMU to the processing process
    pipe: SendPipe<CHUNK_SIZE, NUM_BUFFERS>,

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
        Self {
            active_buffer: None,
            pipe: SendPipe::create("scoop")
                .expect("Failed to create pipe for QemuHook"),
        }
    }
}

thread_local! {
    /// The thread-local QEMU hook state
    static HOOK_STATE: RefCell<HookState> = RefCell::new(HookState::default());
}

macro_rules! create_bitness {
    (
        $tusize:ty, $cannoli:tt, $init:ident, $lift:ident, $entry:ident,
        $exit:ident, $hook:ident, $hook_end:ident, $flush:ident
    ) => {

/// Called by QEMU to initialize this library, we also return version
/// information to QEMU so it knows that we're using the right version
#[no_mangle]
extern fn $init() -> &'static $cannoli {
    /// Bindings information for QEMU
    const BINDINGS: $cannoli = $cannoli {
        version:          CANNOLI_VERSION,
        lift_instruction: Some($lift),
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

/// Called _directly_ from the JIT without preserving any registers. We have
/// to preserve all registers in the JIT, this is not a standard extern FFI!
///
/// This is normal assembly, not shellcode, and won't be copied or moved around
/// so we can actually use symbols in it! Yay!
#[no_mangle]
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
}

/// Magic value to replace with the address of the respective `flush_buffer`
/// function
const REPLACE_WITH_FLUSH: u64 = 0xa03e2cd1b94c78fd;

/// Magic value to replace with the current instructions PC
const REPLACE_WITH_PC: u64 = 0xcc5fe07bf3cfe384;

std::arch::global_asm!(r#"
.extern cannoli_flush_buffer32

// This code is injected _directly_ into QEMUs JIT, we have to make sure we
// don't touch _any_ registers without preserving them
cannoli_hook32:
    // r12 - Pointer to trace buffer
    // r13 - Pointer to end of trace buffer
   
    // Check if we're at the end of the trace buffer, there's always 1
    // target long worth of space, so `jb` is fine here
    cmp r12, r13
    jb  2f

    // We're out of space! This happens "rarely", only when the buffer is full,
    // so we can do much more complex work here. We can also save and restore
    // some registers.
    //
    // We directly call into our Rust to reduce the icache pollution and to get
    // some code sharing for the much more complex flushing operation
    mov  r14, {REPLACE_WITH_FLUSH}
    call r14

2:
    mov dword ptr [r12], {REPLACE_WITH_PC}
    add r12, 4

cannoli_hook32_end:
"#,
    REPLACE_WITH_FLUSH = const REPLACE_WITH_FLUSH,
    REPLACE_WITH_PC    = const REPLACE_WITH_PC,
);

create_bitness!(
    u32, Cannoli32, init_cannoli32, lift_instruction32, jit_entry32,
    jit_exit32, cannoli_hook32, cannoli_hook32_end, cannoli_flush_buffer32
);

//create_bitness!(u64, Cannoli64, init_cannoli64, lift_instruction64, jit_entry64, jit_exit64);

