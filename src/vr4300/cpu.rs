#![allow(non_snake_case)]
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

pub enum RegSize {
    ThirtyTwo(u32),
    SixtyFour(u64),
}

pub struct Regs32 {
    gpr: [u32; 32],
    fpr: [f32; 32],
    pc: u32,
    hi: u32,
    lo: u32,
    ll_bit: bool,
    fcr0: u32,
    fcr31: u32,
}

pub struct Regs64 {
    gpr: [u64; 32],
    fpr: [f64; 32],
    pc: u64,
    hi: u64,
    lo: u64,
    ll_bit: bool,
    fcr0: u32,
    fcr31: u32,
}

pub enum CPURegs {
    ThirtyTwo(Regs32),
    SixtyFour(Regs64),
}


pub struct CPU {
    reg_file: CPURegs,
    pipeline: Pipeline,
    mode: Mode,
    bit_mode: BitMode,
}

impl CPU {}


