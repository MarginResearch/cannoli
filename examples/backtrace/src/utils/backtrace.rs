use std::sync::Mutex;
use serde::ser::{Serialize, Serializer, SerializeStruct};
pub use crate::utils::utils::SymOff;

pub type Backtrace = Vec<(&'static str, u64)>;

pub type ReturnStack = Vec<u64>;

pub struct BacktraceState {
    pub backtrace: Backtrace,
    pub return_stack: ReturnStack,
    pub branch_flag: bool,
}

impl Default for BacktraceState {
    fn default() -> Self {
        Self {
            backtrace: Vec::new(),
            return_stack: Vec::new(),
            branch_flag: false,
        }
    }
}

pub struct PidBacktraceCtx(pub Mutex<BacktraceState>);

impl PidBacktraceCtx {
    pub fn push_backtrace(&self, symbol: &'static str, addr: u64) {
        let backtrace_state: &mut BacktraceState = &mut self.0.lock().unwrap();
        backtrace_state    
            .backtrace
            .push((
                symbol,
                addr
            ));
    }
    pub fn pop_backtrace(&self) -> Option<(&'static str, u64)> {
        let backtrace_state = &mut self.0.lock().unwrap();
        backtrace_state
            .backtrace.pop()
    }

    pub fn push_return_stack(&self, ra: u64) {
        let backtrace_state = &mut self.0.lock().unwrap();
        // push link register to the return stack
        backtrace_state
            .return_stack
            .push(ra);
    }

    pub fn pop_return_stack(&self) -> Option<u64> {
        let backtrace_state = &mut self.0.lock().unwrap();
        backtrace_state
            .return_stack.pop()
    }

    pub fn is_return(&self, pc: u64) -> bool {
        let backtrace_state = &mut self.0.lock().unwrap();
        let last: Option<&u64> =
            backtrace_state.return_stack.last();
        match last {
            Some(&addr) => {
                // println!("Checking return at pc: {:08x}, ra: {:08x}", pc, addr);
                if addr == pc {
                    // println!("returning true");
                    return true
                } else {
                    return false
                }
            },
            None => false,
        }
    }
    
    pub fn is_branch(&self) -> bool {
        self.0.lock().unwrap().branch_flag
    }

    pub fn set_branch_flag(&self) {
        self.0.lock().unwrap().branch_flag = true;
    }

    pub fn unset_branch_flag(&self) {
        self.0.lock().unwrap().branch_flag = false;
    }

    pub fn log_event(&self) {
        let data = serde_json::to_string_pretty(&self)
                    .unwrap() + "\n";
        println!("{}", data);
    }
}

impl Serialize for PidBacktraceCtx {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where S: Serializer,
    {
        let backtrace_state: &mut BacktraceState = &mut self.0.lock().unwrap();
        let mut state = serializer.serialize_struct("Backtrace", 1)?;
        let _ = state.serialize_field("backtrace", &backtrace_state.backtrace);
        state.end()
    }
}

pub struct TidContext {
    // Lookup from an address to a symbol, stored in sorted order
    pub symbols: Vec<(u64, &'static str)>,
}

impl TidContext {
    // Attempt to resolve a symbol into a symbol and an offset
    pub fn resolve(&self, addr: u64) -> SymOff {
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
