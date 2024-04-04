const REG_WIDTH: usize = 64;
const REG_SZ: usize = REG_WIDTH / 8;

enum Aarch64Reg {
    X0 = 0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    LR,
    SP,
}

impl From<usize> for Aarch64Reg {
    fn from(val: usize) -> Self {
        match val {
            0  => Self::X0,
            1  => Self::X1,
            2  => Self::X2,
            3  => Self::X3,
            4  => Self::X4,
            5  => Self::X5,
            6  => Self::X6,
            7  => Self::X7,
            8  => Self::X8,
            9  => Self::X9,
            10 => Self::X10,
            11 => Self::X11,
            12 => Self::X12,
            13 => Self::X13,
            14 => Self::X14,
            15 => Self::X15,
            16 => Self::X16,
            17 => Self::X17,
            18 => Self::X18,
            19 => Self::X19,
            20 => Self::X20,
            21 => Self::X21,
            22 => Self::X22,
            23 => Self::X23,
            24 => Self::X24,
            25 => Self::X25,
            26 => Self::X26,
            27 => Self::X27,
            28 => Self::X28,
            29 => Self::X29,
            30 => Self::LR,
            31 => Self::SP,
            _ => panic!("Invalid AARCH64 register index {}", val),
        }
    }
}

/// syscall numbers for target architecture
#[repr(u32)]
#[derive(Debug)]
pub enum TargetSyscallNum {
    Read = 63,
    Getrandom = 278,
    NotHandled = 0xffffffff,
}

impl From<u32> for TargetSyscallNum {
    fn from(val: u32) -> Self {
        match val {
            63     => TargetSyscallNum::Read,
            278    => TargetSyscallNum::Getrandom,
            _      => TargetSyscallNum::NotHandled,
        }
    }
}

impl Into<u32> for TargetSyscallNum {
    fn into(self) -> u32 {
        match self {
            TargetSyscallNum::Read     => 63,
            TargetSyscallNum::Getrandom => 278,
            _ => 0xffffffff,
        }
    }
}

#[inline]
fn get_aarch64_reg_le(regs: &[u8], register: Aarch64Reg) -> u64 {
    u64::from_le_bytes(*(regs.array_chunks().nth(register as usize).unwrap()))
}

/// Get the standard arm argument registers x0..x7
#[inline]
pub fn get_args(regs: &[u8]) -> [u64; 8] {
    let x0 = get_aarch64_reg_le(regs, Aarch64Reg::X0);
    let x1 = get_aarch64_reg_le(regs, Aarch64Reg::X1);
    let x2 = get_aarch64_reg_le(regs, Aarch64Reg::X2);
    let x3 = get_aarch64_reg_le(regs, Aarch64Reg::X3);
    let x4 = get_aarch64_reg_le(regs, Aarch64Reg::X4);
    let x5 = get_aarch64_reg_le(regs, Aarch64Reg::X5);
    let x6 = get_aarch64_reg_le(regs, Aarch64Reg::X6);
    let x7 = get_aarch64_reg_le(regs, Aarch64Reg::X7);
    [x0, x1, x2, x3, x4, x5, x6, x7]
}

/// Gets the return address from the link register
#[inline]
pub fn get_return_address(regs: &[u8]) -> u64 {
    get_aarch64_reg_le(regs, Aarch64Reg::LR)
}

/// Get the return register r0
#[inline]
pub fn get_return_value(regs: &[u8]) -> u64 {
    get_aarch64_reg_le(regs, Aarch64Reg::X0)
}

/// Get the stack pointer SP
#[inline]
pub fn get_stack_pointer(regs: &[u8]) -> u64 {
    get_aarch64_reg_le(regs, Aarch64Reg::SP)
}
