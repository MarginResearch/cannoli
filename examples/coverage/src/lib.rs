use jitter::HookType;

/// Called before an instruction is lifted in QEMU.
///
/// The `HookType` dictates the type of hook used for the instruction, and may
/// be `Never`, `Always`, and `Once`
///
/// This may be called from multiple threads
#[no_mangle]
fn hook_inst(_pc: u64) -> HookType {
    // We only care about binary hit vs not hit so we are okay with oneshot
    // coverage
    HookType::Once
}

/// Called when a memory access is being lifted in QEMU. Returning `true` will
/// cause the memory access to generate events in the trace buffer.
///
/// This may be called from multiple threads
#[no_mangle]
fn hook_mem(_pc: u64, _write: bool, _size: usize) -> bool {
    // Don't care about memory for coverage stuff
    false
}

