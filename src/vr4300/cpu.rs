#![allow(non_snake_case)]
use std::sync::Arc;
use std::sync::Mutex;
struct RegFile {
    GPR: [u64; 32],
    FPR: [u64; 32],
    PC: u64,
    HI: u64,
    LO: u64,
    LLBit: u8,
    FCR0: u32,
    FCR31: u32,
}
use crate::vr4300::pipeline::Pipeline;
pub enum Mode {
    User,
    Supervisor,
    Kernel,
}

pub enum BitMode {
    ThirtyTwo,
    SixtyFour,
}


pub struct CPURegs {
    gpr: [u64; 32],
    fpr: [f64; 32],
    pc: u64,
    hi: u64,
    lo: u64,
    ll_bit: bool,
    fcr0: u32,
    fcr31: u32,
}

pub struct CPU {
    reg_file: RegFile,
    pipeline: Pipeline,
    mode: Mode,
    bit_mode: BitMode,
}

impl CPU {}


