use cannoli::{Cannoli, create_cannoli};

struct Symbolizer;

impl Cannoli for Symbolizer {
    type Trace = (&'static str, u64);

    type Context = Vec<(u64, &'static str)>;

    fn init(_: u64) -> (Self, Self::Context) {
        let mut symbols = Vec::new();
        symbols.push((0x100000000, "apples"));
        (Self, symbols)
    }

    fn exec(ctxt: &Self::Context, pc: u64) -> Option<Self::Trace> {
        match ctxt.binary_search_by_key(&pc, |x| x.0) {
            Ok(pos) => {
                // Direct symbol match
                Some((ctxt[pos].1, 0))
            }
            Err(pos) => {
                // Position is where to insert to maintain sorted order, thus
                // pos - 1 is the index where the nearest symbol to this
                // address is
                if let Some((addr, sym)) = ctxt.get(pos.wrapping_sub(1)) {
                    Some((sym, pc - addr))
                } else {
                    // Unknown symbol
                    Some(("<unknown>", pc))
                }
            }
        }
    }

    fn read(_ctxt: &Self::Context, _pc: u64, _addr: u64, _val: u64)
            -> Option<Self::Trace> {
        None
    }

    fn write(_ctxt: &Self::Context, _pc: u64, _addr: u64, _val: u64)
            -> Option<Self::Trace> {
        None
    }

    fn trace(&mut self, _ctxt: &Self::Context, trace: &[Self::Trace]) {
    }
}

fn main() {
    create_cannoli::<Symbolizer>(4).unwrap();
}

