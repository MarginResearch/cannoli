/// syscall numbers for x86 host
#[repr(u32)]
#[derive(Debug, PartialEq)]
pub enum HostSyscallNum {
    Read = 0,
    Getrandom = 318,
    NotHandled = 0xffffffff,
}

impl From<u32> for HostSyscallNum {
    fn from(val: u32) -> Self {
        match val {
            0   => HostSyscallNum::Read,
            318 => HostSyscallNum::Getrandom,
            _   => HostSyscallNum::NotHandled,
        }
    }
}

/// syscall numbers for target architecture
#[repr(u32)]
#[derive(Debug)]
pub enum TargetSyscallNum {
    Read = 0,
    Getrandom = 318,
    NotHandled = 0xffffffff,
}

impl From<u32> for TargetSyscallNum {
    fn from(val: u32) -> Self {
        match val {
            _      => TargetSyscallNum::NotHandled,
        }
    }
}

impl Into<u32> for TargetSyscallNum {
    fn into(self) -> u32 {
        match self {
            _ => 0xffffffff,
        }
    }
}

pub fn get_args(_regs: &[u8]) -> [u32; 4] {
    panic!("No arch specified at compilation or not implemented!");
}

pub fn get_return_address(_regs: &[u8]) -> u32 {
    panic!("No arch specified at compilation or not implemented!");
}

pub fn get_return_value(_regs: &[u8]) -> u32 {
    panic!("No arch specified at compilation or not implemented!");
}

pub fn get_stack_pointer(_regs: &[u8]) -> u32 {
    panic!("No arch specified at compilation or not implemented!");
}

