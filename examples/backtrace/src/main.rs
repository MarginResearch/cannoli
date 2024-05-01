//! An example user of Cannoli which collects register traces

#![feature(array_chunks)]

use std::sync::{Arc, Mutex};
use cannoli::Cannoli;

use crate::utils::backtrace::{
    BacktraceState,
    PidBacktraceCtx,
    TidContext,
};

mod utils;
pub use crate::utils::SymOff;

enum Trace {
    Branch {
        branch: bool,
        pc: SymOff,
        ra: u64,
    },
}

struct BacktraceExample;

impl Cannoli for BacktraceExample {
    type Trace = Trace;
    type TidContext = TidContext;
    type PidContext = PidBacktraceCtx;

    fn init_pid(_c: &cannoli::ClientInfo) -> Arc<Self::PidContext> {
        Arc::new(
            PidBacktraceCtx(Mutex::new(BacktraceState::default()))
        )
    }

    /// Load the symbol table
    fn init_tid(_pid: &Self::PidContext,
            _ci: &cannoli::ClientInfo) -> (Self, Self::TidContext) {
        // Symbols
        let mut symbols = Vec::new();

        // Load the symbol file up and leak it so all the lifetimes are static
        let data = std::fs::read_to_string("symbols.txt").unwrap();
        let data = Box::leak(data.into_boxed_str());

        // Parse each line into an address and symbol
        for line in data.lines() {
            let chunk = line.splitn(3, ' ').collect::<Vec<_>>();

            let addr = u64::from_str_radix(chunk[0], 16).unwrap();
            let sym  = chunk[2];
            symbols.push((addr, sym));
        }

        // Sort the symbols by address
        symbols.sort_by_key(|x| x.0);
        (Self, TidContext {
            symbols,
        })
    }

    fn branch(_pid: &Self::PidContext, tid: &Self::TidContext,
            pc: u64, branch: bool, regs: &[u8], trace: &mut Vec<Self::Trace>) {

        // Get return instruction pointer, architecture specific
        let i = 31 * 4; // $31 in mips * 4 bytes per register
        // Get a slice at the register's index -> u32
        let ra = u32::from_le_bytes(regs[i..i+4].try_into().unwrap());

        // Resolve the symbol at the PC
        let pc = tid.resolve(pc);

        // Push the inst event
        trace.push(Trace::Branch {
            pc,
            branch,
            ra: ra as u64,
        });
    }

    fn trace(&mut self, pid: &Self::PidContext, tid: &Self::TidContext,
             trace: &[Self::Trace]) {
        for ent in trace {
            match ent {
                Trace::Branch { branch, pc, ra } => {
                   // handle backtrace operations if flag is set
                    if pid.is_branch() {
                        // check if this instruction is the entry to a func
                        let sym = tid.resolve(pc.addr());
                        if sym.is_entry() {
                           // push symbol to backtrace stack
                            pid.push_backtrace(sym.symbol(),pc.addr());

                            // push link register to the return stack
                            pid.push_return_stack(*ra);
                            // log new backtrace state
                            pid.log_event();
                        }
                        // else check if this instruction matches the expected
                        // return address (last val pushed to the ret stack)
                        // need to check in loop to handle branch w/o link
                        loop {
                            if pid.is_return(pc.addr()) {
                                // pop two stacks
                                pid.pop_backtrace();
                                pid.pop_return_stack();

                                // log new backtrace state
                                pid.log_event();
                            }
                            else { break }
                        }

                        // clear flag
                        pid.unset_branch_flag();
                    }
                    // if current instruction is a branch, set branch flag
                    // to analyze next instruction address
                    if *branch {
                        pid.set_branch_flag();
                    }
                },
            }
        }
    }
}

fn main() {
    cannoli::run::<BacktraceExample>(4).unwrap();
}
