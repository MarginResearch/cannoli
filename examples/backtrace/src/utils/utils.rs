// An original pointer address, and then a resolved symbol + offset for that    
// address                                                                      
#[derive(Clone, Copy)]                                                          
pub struct SymOff {                                                                 
    // The "raw", original address                                              
    pub addr: u64,                                                                  
                                                                                
    // Symbol                                                                   
    pub symbol: &'static str,                                                       
                                                                                
    // Offset from the base of the symbol                                       
    pub offset: u64,                                                                
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

impl SymOff {
    pub fn addr(self) -> u64 {
        self.addr
    }
    pub fn symbol(self) -> &'static str {
        self.symbol
    }
    pub fn offset(self) -> u64 {
        self.offset
    }
    pub fn is_entry(&self) -> bool {
        self.offset == 0 && self.symbol != "<unknown>"
    }
}
