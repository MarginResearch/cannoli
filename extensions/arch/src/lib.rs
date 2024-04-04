#![feature(array_chunks)]

pub mod default;
pub use crate::default::HostSyscallNum;

// These are all Cannoli supported architectures. If we compile with one of
// these features, ignore the following definition. The following definition
// allows for compilation when there is NO specified arch.
#[cfg(not(any(
    feature = "aarch64",
    feature = "aarch64be",
    feature = "alpha",
    feature = "armv5teb",
    feature = "armv5tel",
    feature = "cris",
    feature = "hexagon",
    feature = "i386",
    feature = "i686",
    feature = "m68k",
    feature = "microblaze",
    feature = "mips",
    feature = "mips64",
    feature = "nios2",
    feature = "openrisc",
    feature = "parisc",
    feature = "ppc",
    feature = "ppc64",
    feature = "ppc64le",
    feature = "riscv32",
    feature = "riscv64",
    feature = "s390x",
    feature = "sh4",
    feature = "sparc",
    feature = "sparc64",
    feature = "x86_64",
    feature = "xtensa"
)))]
pub use crate::default::{
    TargetSyscallNum,
    get_args,
    get_return_address,
    get_return_value,
    get_stack_pointer,
};

#[cfg(feature = "mips")]
pub mod mipsel32;

#[cfg(feature = "mips")]
pub use crate::mipsel32::{
    TargetSyscallNum,
    get_args,
    get_return_address,
    get_return_value,
    get_stack_pointer,
};

#[cfg(feature = "armv5tel")]
pub mod armv5tel;

#[cfg(feature = "armv5tel")]
pub use crate::armv5tel::{
    TargetSyscallNum,
    get_args,
    get_return_address,
    get_return_value,
    get_stack_pointer,
};

#[cfg(feature = "aarch64")]
pub mod aarch64;

#[cfg(feature = "aarch64")]
pub use crate::aarch64::{
    TargetSyscallNum,
    get_args,
    get_return_address,
    get_return_value,
    get_stack_pointer,
};

impl From<TargetSyscallNum> for HostSyscallNum {
    fn from(val: TargetSyscallNum) -> Self {
        match val {
            TargetSyscallNum::Read       => HostSyscallNum::Read,
            TargetSyscallNum::Getrandom  => HostSyscallNum::Getrandom,
            TargetSyscallNum::NotHandled => HostSyscallNum::NotHandled,
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum SyscallParamType {
    Na = 0,
    Bool,
    Char,
    Int,
    RawPtr,
    Cstr,
}

// ** ensure numbering matches the ordering above ** //
impl From<u8> for SyscallParamType {
    fn from(val: u8) -> Self {
        match val {
            0 => SyscallParamType::Na,
            1 => SyscallParamType::Bool,
            2 => SyscallParamType::Char,
            3 => SyscallParamType::Int,
            4 => SyscallParamType::RawPtr,
            5 => SyscallParamType::Cstr,
            _ => panic!("error matching SyscallParamType for {}", val),
        }
    }
}


/// RegState is a generic struct that stores addresses and values retrieved
/// by architecture-specific functions for easy indexing and logging
#[derive(Debug)]
pub struct RegState {
    pub pc: u64,
    pub arg0: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub ret_addr: u64,
    pub ret_val: u64,
    pub sp: u64,
}

impl RegState {
    pub fn new(pc: u64, regs: &[u8]) -> RegState {
        let args = get_args(regs);
        RegState {
            pc: pc,
            arg0: args[0] as u64,
            arg1: args[1] as u64,
            arg2: args[2] as u64,
            arg3: args[3] as u64,
            ret_addr: get_return_address(regs) as u64,
            ret_val: get_return_value(regs) as u64,
            sp: get_stack_pointer(regs) as u64,
        }
    }
}

#[cfg(feature = "aarch64be")]
compile_error!("Arch aarch64be not currently supported");
#[cfg(feature = "alpha")]
compile_error!("Arch alpha not currently supported");
#[cfg(feature = "armv5teb")]
compile_error!("Arch armv5teb not currently supported");
#[cfg(feature = "cris")]
compile_error!("Arch cris not currently supported");
#[cfg(feature = "hexagon")]
compile_error!("Arch hexagon not currently supported");
#[cfg(feature = "i386")]
compile_error!("Arch i386 not currently supported");
#[cfg(feature = "i686")]
compile_error!("Arch i686 not currently supported");
#[cfg(feature = "m68k")]
compile_error!("Arch m68k not currently supported");
#[cfg(feature = "microblaze")]
compile_error!("Arch microblaze not currently supported");
#[cfg(feature = "mips64")]
compile_error!("Arch mips64 not currently supported");
#[cfg(feature = "nios2")]
compile_error!("Arch nios2 not currently supported");
#[cfg(feature = "openrisc")]
compile_error!("Arch openrisc not currently supported");
#[cfg(feature = "parisc")]
compile_error!("Arch parisc not currently supported");
#[cfg(feature = "ppc")]
compile_error!("Arch ppc not currently supported");
#[cfg(feature = "ppc64")]
compile_error!("Arch ppc64 not currently supported");
#[cfg(feature = "ppc64le")]
compile_error!("Arch ppc64le not currently supported");
#[cfg(feature = "riscv32")]
compile_error!("Arch riscv32 not currently supported");
#[cfg(feature = "riscv64")]
compile_error!("Arch riscv64 not currently supported");
#[cfg(feature = "s390x")]
compile_error!("Arch s390x not currently supported");
#[cfg(feature = "sh4")]
compile_error!("Arch sh4 not currently supported");
#[cfg(feature = "sparc")]
compile_error!("Arch sparc not currently supported");
#[cfg(feature = "sparc64")]
compile_error!("Arch sparc64 not currently supported");
#[cfg(feature = "x86_64")]
compile_error!("Arch x86_64 not currently supported");
#[cfg(feature = "xtensa")]
compile_error!("Arch xtensa not currently supported");


#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "mips")]
    #[test]
    fn mips_regs() {
        let regs: Vec<u8> = (0..(32*4)).collect();
        let regstate = RegState::new(0xdeadbeef, &regs);
        assert_eq!(regstate.ret_addr, 0x7f7e7d7c);
        assert_eq!(regstate.ret_val, 0x0b0a0908);
        assert_eq!(regstate.sp, 0x77767574);
    }

    #[cfg(feature = "aarch64")]
    #[test]
    fn aarch64_regs() {
        let regs: Vec<u8> = (0..=0xff).collect();
        let regstate = RegState::new(0xdeadbeef, &regs);
        assert_eq!(regstate.ret_addr, 0xf7f6f5f4f3f2f1f0);
        assert_eq!(regstate.ret_val,  0x0706050403020100);
        assert_eq!(regstate.sp,       0xfffefdfcfbfaf9f8);
    }

    #[cfg(feature = "armv5tel")]
    #[test]
    fn arm_regs() {
        let regs: Vec<u8> = (0..(32*4)).collect();
        let regstate = RegState::new(0xdeadbeef, &regs);
        assert_eq!(regstate.ret_addr, 0x3b3a3938);
        assert_eq!(regstate.ret_val, 0x03020100);
        assert_eq!(regstate.sp, 0x37363534);
    }
}
