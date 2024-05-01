//! An example user of Cannoli which collects register traces

#![feature(array_chunks)]

use std::sync::Arc;
use cannoli::Cannoli;

/// The structure we implement [`Cannoli`] for!
struct Coverage;

/// Context shared between threads
struct Context;

impl Cannoli for Coverage {
    /// The type emit in the serialized trace
    type Trace = ();

    /// Context, the shared, immutable context shared between all threads doing
    /// processing. We stuff our symbol table here.
    type TidContext = Context;
    type PidContext = ();
    
    fn init_pid(_: &cannoli::ClientInfo) -> Arc<Self::PidContext> {
        Arc::new(())
    }

    /// Load the symbol table
    fn init_tid(_pid: &Self::PidContext,
                _ci: &cannoli::ClientInfo) -> (Self, Self::TidContext) {
        (Self, Context)
    }

    fn regs(_pid: &Self::PidContext, _tid: &Self::TidContext,
            pc: u64, regs: &[u8], _trace: &mut Vec<Self::Trace>) {
        let mut parsed = [0; 32];
        for (ii, chunk) in regs.array_chunks::<4>().enumerate() {
            parsed[ii] = u32::from_le_bytes(*chunk);
        }

        println!("Regs {pc:#x} {parsed:#x?}");
    }
}

fn main() {
    cannoli::run::<Coverage>(4).unwrap();
}

