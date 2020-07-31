#![allow(non_snake_case)]
struct RegFile {
    Index: u64,
    Random: u64,
    EntryLo0: u64,
    EntryLo1: u64,
    Context: u64,
    PageMask: u64,
    Wired: u64,
    BadVAddr: u64,
    Count: u64,
    EntryHi: u64,
    Compare: u64,
    Status: u64,
    Cause: u64,
    EPC: u64,
    PRId: u64,
    Config: u64,
    LLAddr: u64,
    WatchLo: u64,
    WatchHi: u64,
    XContext: u64,
    ParityError: u64,
    CacheError: u64,
    TagLo: u64,
    TagHi: u64,
    ErrorEPC: u64,
}

struct CP0 {
    registers: [u64; 32]
}

impl CP0 {
    
}
