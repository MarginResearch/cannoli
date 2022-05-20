//! A small shared library which connects to small stubs patched into QEMU
//! and provides real-time streaming of things like PCs, register state, and
//! memory accesses

#![feature(asm_const, maybe_uninit_slice, inline_const, naked_functions)]
#![feature(asm_sym)]

mod cannoli_memops;
mod cannoli_internals;

/// Called before an instruction is lifted in QEMU. If this function returns
/// `true`, then the instrumentation is added and this PC will generate logs
/// in the traces.
///
/// This may be called from multiple threads
fn hook_inst(_pc: u64) -> bool {
    true
}

/// Called when a memory access is being lifted in QEMU. Returning `true` will
/// cause the memory access to generate events in the trace buffer.
///
/// This may be called from multiple threads
fn hook_mem(_pc: u64, _write: bool, _size: usize) -> bool {
    true
}

