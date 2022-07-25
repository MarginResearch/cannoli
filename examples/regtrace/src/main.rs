//! An example user of Cannoli which collects register traces

#![feature(array_chunks)]

use cannoli::{Cannoli, create_cannoli};

/// The structure we implement [`Cannoli`] for!
struct Coverage;

/// Context shared between threads
struct Context;

impl Cannoli for Coverage {
    /// The type emit in the serialized trace
    type Trace = ();

    /// Context, the shared, immutable context shared between all threads doing
    /// processing. We stuff our symbol table here.
    type Context = Context;

    /// Load the symbol table
    fn init(_ci: &cannoli::ClientInfo) -> (Self, Self::Context) {
        (Self, Context)
    }

    fn regs(_ctxt: &Context, pc: u64, regs: &[u8]) -> Option<()> {
        let mut parsed = [0; 32];
        for (ii, chunk) in regs.array_chunks::<4>().enumerate() {
            parsed[ii] = u32::from_le_bytes(*chunk);
        }

        println!("Regs {pc:#x} {parsed:#x?}");
        None
    }
}

fn main() {
    create_cannoli::<Coverage>(4).unwrap();
}

