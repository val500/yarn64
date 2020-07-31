pub struct Pipeline {
    ic_out: IcOut,
    rf_out: RfOut,
    executor: Executor,
    dc_out: DcOut,
    wb_out: WbOut,
}

impl Pipeline {

}

struct Executor {
    ex_in: ExIn,
    ex_out: ExOut,
}

//TODO: Cache operations and coprocessor operations
impl Executor {
    fn execute(&self) {
        // Parse ex in and execute functions
    }

    
}


struct ExIn {
    opcode: u8, // 6 bits
    pc: u64,
    rs: Option<u8>, // 5 bits
    rt: Option<u8>, // 5 bits
    rd: Option<u8>, // 5 bits
    rs_contents: Option<u64>,
    rt_contents: Option<u64>,
    immediate: Option<u16>,
}

#[derive(Default)]
pub struct ExOut {
    pub writeback: Option<u64>,
    pub new_pc: Option<u64>,
    pub target_register: Option<u8>,
    pub discard_next_instruction: bool,
    pub hi_writeback: Option<u64>,
    pub lo_writeback: Option<u64>,
}

struct IcOut {}
struct RfOut {}
struct DcOut {}
struct WbOut {}
