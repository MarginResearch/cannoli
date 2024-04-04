use std::fmt;

const BITWIDTH: usize = 4;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum MipsReg {
    ZERO = 0, AT, V0, V1,
    A0, A1, A2, A3,
    T0, T1, T2, T3,
    T4, T5, T6, T7,
    S0, S1, S2, S3,
    S4, S5, S6, S7,
    T8, T9, K0, K1,
    GP, SP, FP, RA,
}

impl Into<usize> for MipsReg {
    fn into(self) -> usize {
        self as usize
    }
}

impl From<usize> for MipsReg {
    fn from(val: usize) -> Self {
        match val {
            0  => Self::ZERO,
            1  => Self::AT,
            2  => Self::V0,
            3  => Self::V1,
            4  => Self::A0,
            5  => Self::A1,
            6  => Self::A2,
            7  => Self::A3,
            8  => Self::T0,
            9  => Self::T1,
            10 => Self::T2,
            11 => Self::T3,
            12 => Self::T4,
            13 => Self::T5,
            14 => Self::T6,
            15 => Self::T7,
            16 => Self::S0,
            17 => Self::S1,
            18 => Self::S2,
            19 => Self::S3,
            20 => Self::S4,
            21 => Self::S5,
            22 => Self::S6,
            23 => Self::S7,
            24 => Self::T8,
            25 => Self::T9,
            26 => Self::K0,
            27 => Self::K1,
            28 => Self::GP,
            29 => Self::SP,
            30 => Self::FP,
            31 => Self::RA,
            _ => panic!("Invalid MIPS register index {}", val),
        }
    }
}

pub const REGS: [&str; 32] = [
    "$zero", "$at", "$v0", "$v1",
    "$a0",   "$a1", "$a2", "$a3",
    "$t0",   "$t1", "$t2", "$t3",
    "$t4",   "$t5", "$t6", "$t7",
    "$s0",   "$s1", "$s2", "$s3",
    "$s4",   "$s5", "$s6", "$s7",
    "$t8",   "$t9", "$k0", "$k1",
    "$gp",   "$sp", "$fp", "$ra",
];

impl fmt::Display for MipsReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", REGS[*self as usize])
    }
}

#[inline]
fn get_mips_reg_le(regs: &[u8], register: MipsReg) -> u32 {
    let i = register as usize * BITWIDTH;
    // Get a slice at the register's index -> u32
    u32::from_le_bytes(regs[i..i+4].try_into().unwrap())
}

/// Print out mips registers
#[allow(dead_code)]
pub fn regs_fmt_mipsel32(regs: &[u8]) {
    for i in 0..regs.len() / BITWIDTH {
        let x = get_mips_reg_le(regs, i.into());
        if (i + 1) % 4 == 0 {
            println!("\x1b[1;32m{:05}\x1b[0m {:08x}", REGS[i], x);
        } else {
            print!("\x1b[1;32m{:05}\x1b[0m {:08x} ", REGS[i], x);
        }
    }
}

/// data structures and functions for handling syscall parsing.
/// the format of syscall serialization is as follows:
///
/// bytes: u8      u32           u8            u32
/// .________._____________._____________._____________.
/// | opcode | syscall num | return type | return data |...
/// `--------`-------------`-------------`-------------`
///
/// bytes: u32       u8          ?           u8          ?
/// .___________.___________.___________.___________.___________.
/// | arg count | arg1 type | arg1 data | arg2 type | arg2 data |..
/// `-----------`-----------`-----------`-----------`-----------`
///
/// where "type" is the u8 that matches against a SyscallParamType enum and
/// "data" is dependent on the argument type. Simple data types like bools,
/// chars, and ints immediately follow the ret/arg type. Other more complex
/// types, such as RawPtr and Cstr, have multiple parameters in their
/// SyscallParam type, such as an address and the data at that address. Their
/// format is defined in the SyscallParam enum

/// syscall numbers for target architecture
/// see linux-user/mips/syscall_o32.tbl and add mips base (0xfa0) to each
#[derive(Debug)]
pub enum TargetSyscallNum {
    Read = 0xfa3,
    Getrandom = 0x1101,
    NotHandled = 0xffffffff,
}

impl From<u32> for TargetSyscallNum {
    fn from(val: u32) -> Self {
        match val {
            0xfa3  => TargetSyscallNum::Read,
            0x1101 => TargetSyscallNum::Getrandom,
            _      => TargetSyscallNum::NotHandled,
        }
    }
}

impl Into<u32> for TargetSyscallNum {
    fn into(self) -> u32 {
        match self {
            TargetSyscallNum::Read      => 0xfa3,
            TargetSyscallNum::Getrandom => 0x1101,
            _ => 0xffffffff,
        }
    }
}

/// Get the standard mips argument registers a0..a3
#[inline]
pub fn get_args(regs: &[u8]) -> [u32; 4] {
    let a0 = get_mips_reg_le(regs, MipsReg::A0);
    let a1 = get_mips_reg_le(regs, MipsReg::A1);
    let a2 = get_mips_reg_le(regs, MipsReg::A2);
    let a3 = get_mips_reg_le(regs, MipsReg::A3);
    [a0, a1, a2, a3]
}

/// Gets the return address from mips register RA
#[inline]
pub fn get_return_address(regs: &[u8]) -> u32 {
    get_mips_reg_le(regs, MipsReg::RA)
}

/// Get the return register v0
#[inline]
pub fn get_return_value(regs: &[u8]) -> u32 {
    get_mips_reg_le(regs, MipsReg::V0)
}

/// Get the stack pointer SP
#[inline]
pub fn get_stack_pointer(regs: &[u8]) -> u32 {
    get_mips_reg_le(regs, MipsReg::SP)
}

