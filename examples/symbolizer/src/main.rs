//! An example user of Cannoli which symbolizes a trace

use cannoli::{Cannoli, create_cannoli};

/// An original pointer address, and then a resolved symbol + offset for that
/// address
struct SymOff {
    /// The "raw", original address
    addr: u64,

    /// Symbol
    symbol: &'static str,

    /// Offset from the base of the symbol
    offset: u64,
}

impl std::fmt::Display for SymOff {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.offset != 0 {
            write!(f, "{:#018x} ({}+{:#x})",
                self.addr, self.symbol, self.offset)
        } else {
            write!(f, "{:#018x} ({})", self.addr, self.symbol)
        }
    }
}

enum Operation {
    Exec  { pc: SymOff },
    Read  { pc: SymOff, addr: SymOff, val: u64, sz: u8 },
    Write { pc: SymOff, addr: SymOff, val: u64, sz: u8 },
}

/// The structure we implement [`Cannoli`] for!
struct Symbolizer;

/// Context shared between threads
struct Context {
    /// Lookup from an address to a symbol, stored in sorted order
    symbols: Vec<(u64, &'static str)>,
}

impl Context {
    /// Attempt to resolve a symbol into a symbol and an offset
    fn resolve(&self, addr: u64) -> SymOff {
        // Get access to the symbols
        let symbols = &self.symbols;

        // Find the symbol
        match symbols.binary_search_by_key(&addr, |x| x.0) {
            Ok(pos) => {
                // Direct symbol match
                SymOff {
                    addr,
                    symbol: symbols[pos].1,
                    offset: 0
                }
            }
            Err(pos) => {
                // Found location after symbol, find the nearest symbol below
                if let Some((sa, sym)) = pos.checked_sub(1)
                        .and_then(|x| symbols.get(x)) {
                    // Got symbol below
                    SymOff {
                        addr,
                        symbol: sym,
                        offset: addr - sa
                    }
                } else {
                    // No symbols below this address, just emit the PC
                    SymOff {
                        addr,
                        symbol: "<unknown>",
                        offset: addr,
                    }
                }
            }
        }
    }
}

impl Cannoli for Symbolizer {
    /// The type emit in the serialized trace
    type Trace = Operation;

    /// Context, the shared, immutable context shared between all threads doing
    /// processing. We stuff our symbol table here.
    type Context = Context;

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

        (Self, Context { symbols })
    }

    /// Convert PCs into symbol + offset in parallel
    fn exec(ctxt: &Self::Context, pc: u64) -> Option<Self::Trace> {
        Some(Operation::Exec { pc: ctxt.resolve(pc) })
    }

    /// Symbolize reads
    fn read(ctxt: &Self::Context, pc: u64, addr: u64, val: u64, sz: u8)
            -> Option<Self::Trace> {
        Some(Operation::Read {
            pc:   ctxt.resolve(pc),
            addr: ctxt.resolve(addr),
            val, sz,
        })
    }

    /// Symbolize writes
    fn write(ctxt: &Self::Context, pc: u64, addr: u64, val: u64, sz: u8)
            -> Option<Self::Trace> {
        Some(Operation::Write {
            pc:   ctxt.resolve(pc),
            addr: ctxt.resolve(addr),
            val, sz,
        })
    }

    /// Print the trace we processed!
    fn trace(&mut self, _ctxt: &Self::Context, trace: &[Self::Trace]) {
        for op in trace {
            match op {
                Operation::Exec { pc } => {
                    println!("\x1b[0;34mEXEC\x1b[0m   @ {pc}");
                }
                Operation::Read { pc, addr, val, sz } => {
                    println!("\x1b[0;32mREAD{sz}\x1b[0m  @ {pc} | \
                        {addr} ={val:#x}");
                }
                Operation::Write { pc, addr, val, sz } => {
                    println!("\x1b[0;31mWRITE{sz}\x1b[0m @ {pc} | \
                        {addr} ={val:#x}");
                }
            }
        }
    }
}

fn main() {
    create_cannoli::<Symbolizer>(2).unwrap();
}

