use jitter::HookType;

/// Called before an instruction is lifted in QEMU.
///
/// The `HookType` dictates the type of hook used for the instruction, and may
/// be `Never`, `Always`, and `Once`
///
/// This may be called from multiple threads
#[no_mangle]
fn hook_inst(_pc: u64) -> HookType {
    HookType::Always
}

/// Called when a memory access is being lifted in QEMU. Returning `true` will
/// cause the memory access to generate events in the trace buffer.
///
/// This may be called from multiple threads
#[no_mangle]
fn hook_mem(_pc: u64, _write: bool, _size: usize) -> bool {
    true
}

