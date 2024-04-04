use jitter::HookType;

#[no_mangle]
fn hook_inst(_pc: u64, _branch: bool) -> HookType {
    HookType::Branch
}

// no memory tracing needed unless analyzing x86 binaries, which push the
// return address to the stack
#[no_mangle]
fn hook_mem(_pc: u64, _write: bool, _size: usize) -> bool {
    false
}

#[no_mangle]
fn hook_user_syscall() -> bool {
    false
}
