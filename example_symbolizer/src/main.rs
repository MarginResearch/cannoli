//! An example user of Cannoli which symbolizes a trace

use cannoli::{Cannoli, create_cannoli};

/// The structure we implement symbolizer for!
struct Symbolizer;

impl Cannoli for Symbolizer {
    /// The type emit in the serialized trace
    ///
    /// In our case, this is `(symbol, offset)`
    type Trace = (&'static str, u64);

    /// Context, the shared, immutable context shared between all threads doing
    /// processing. We stuff our symbol table here.
    type Context = Vec<(u64, &'static str)>;

    /// Load the symbol table
    fn init(_: u64) -> (Self, Self::Context) {
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

        (Self, symbols)
    }

    /// Convert PCs into symbol + offset in parallel
    fn exec(ctxt: &Self::Context, pc: u64) -> Option<Self::Trace> {
        // Find the symbol
        match ctxt.binary_search_by_key(&pc, |x| x.0) {
            Ok(pos) => {
                // Direct symbol match
                Some((ctxt[pos].1, 0))
            }
            Err(pos) => {
                // Found location after symbol, find the nearest symbol below
                if let Some((addr, sym)) = ctxt.get(pos.wrapping_sub(1)) {
                    // Got symbol below
                    Some((sym, pc - addr))
                } else {
                    // No symbols below this address, just emit the PC
                    Some(("<unknown>", pc))
                }
            }
        }
    }

    fn trace(&mut self, _ctxt: &Self::Context, trace: &[Self::Trace]) {
        for (sym, off) in trace {
            println!("{}+{:#x}", sym, off);
        }
    }
}

fn main() {
    create_cannoli::<Symbolizer>(10).unwrap();
}

