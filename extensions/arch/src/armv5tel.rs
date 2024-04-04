const REG_WIDTH: usize = 32;
const REG_SZ: usize = REG_WIDTH / 8;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum ArmReg {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    SP,
    LR,
    PC,
}

impl From<usize> for ArmReg {
    fn from(val: usize) -> Self {
        match val {
            0  => Self::R0,
            1  => Self::R1,
            2  => Self::R2,
            3  => Self::R3,
            4  => Self::R4,
            5  => Self::R5,
            6  => Self::R6,
            7  => Self::R7,
            8  => Self::R8,
            9  => Self::R9,
            10 => Self::R10,
            11 => Self::R11,
            12 => Self::R12,
            13 => Self::SP,
            14 => Self::LR,
            15 => Self::PC,
            _ => panic!("Invalid ARM register index {}", val),
        }
    }
}

/// syscall numbers for target architecture
#[repr(u32)]
#[derive(Debug)]
pub enum TargetSyscallNum {
    Read = 3,
    Getrandom = 384,
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

#[inline]
fn get_arm_reg_le(regs: &[u8], register: ArmReg) -> u32 {
    let i = register as usize * REG_SZ;
    u32::from_le_bytes(regs[i..i+REG_SZ].try_into().unwrap())
}

/// Get the standard arm argument registers r0..r3
pub fn get_args(regs: &[u8]) -> [u32; 4] {
    let r0 = get_arm_reg_le(regs, ArmReg::R0);
    let r1 = get_arm_reg_le(regs, ArmReg::R1);
    let r2 = get_arm_reg_le(regs, ArmReg::R2);
    let r3 = get_arm_reg_le(regs, ArmReg::R3);
    [r0, r1, r2, r3]
}

/// Gets the return address from the link register
#[inline]
pub fn get_return_address(regs: &[u8]) -> u32 {
    get_arm_reg_le(regs, ArmReg::LR)
}

/// Get the return register r0
#[inline]
pub fn get_return_value(regs: &[u8]) -> u32 {
    get_arm_reg_le(regs, ArmReg::R0)
}

/// Get the stack pointer SP
#[inline]
pub fn get_stack_pointer(regs: &[u8]) -> u32 {
    get_arm_reg_le(regs, ArmReg::SP)
}

