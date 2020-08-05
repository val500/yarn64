use crate::vr4300::{cpu::BitMode, pipeline::{ExOut, LoadInput, LoadType}};

fn ex_add(rs_contents: i64, rt_contents: i64, rd: Option<u8>, bit_mode: BitMode) -> ExOut {
    ExOut {
        writeback: {
            let add = match (rs_contents as i32).checked_add(rt_contents as i32) {
                Some(ans) => ans,
                None => 0, // TODO: Integer overflow exception
            };
            Some(match bit_mode {
                BitMode::ThirtyTwo => add as u32 as u64, // zero extend
                BitMode::SixtyFour => add as u64,        // sign extend
            })
        },
        target_register: rd,
        ..Default::default()
    }
}

fn ex_addi(rs_contents: i64, immediate: i16, rd: Option<u8>, bit_mode: BitMode) -> ExOut {
    ExOut {
        writeback: {
            let add = match (rs_contents as i32).checked_add(immediate as i32) {
                Some(ans) => ans,
                None => 0, // TODO: Integer overflow exception
            };
            Some(match bit_mode {
                BitMode::ThirtyTwo => add as u32 as u64, // zero extend
                BitMode::SixtyFour => add as u64,        // sign extend
            })
        },
        target_register: rd,
        ..Default::default()
    }
}

fn ex_addiu(rs_contents: i64, immediate: i16, rd: Option<u8>, bit_mode: BitMode) -> ExOut {
    ExOut {
        writeback: {
            let add = (rs_contents as i32) + (immediate as i32);
            Some(match bit_mode {
                BitMode::ThirtyTwo => add as u32 as u64, // zero extend
                BitMode::SixtyFour => add as u64,        // sign extend
            })
        },
        target_register: rd,
        ..Default::default()
    }
}

fn ex_addu(rs_contents: u64, rt_contents: u64, rd: Option<u8>, bit_mode: BitMode) -> ExOut {
    ExOut {
        writeback: Some(match bit_mode {
            BitMode::ThirtyTwo => ((rs_contents as i32 + rt_contents as i32) as u32 as u64), // zero extend
            BitMode::SixtyFour => ((rs_contents as i32 + rt_contents as i32) as u64), // sign extend
        }),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_and(rs_contents: u64, rt_contents: u64, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some(rs_contents & rt_contents),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_andi(rs_contents: u64, immediate: u16, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some(rs_contents & (immediate as u64)),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_beq(rs_contents: u64, rt_contents: u64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch(rs_contents == rt_contents, pc, offset, bit_mode)
}

fn ex_beql(rs_contents: u64, rt_contents: u64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch_likely(rs_contents == rt_contents, pc, offset, bit_mode)
}

fn ex_bgez(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch(rs_contents >= 0, pc, offset, bit_mode)
}

fn ex_bgezal(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch_and_link(rs_contents >= 0, pc, offset, bit_mode)
}

fn ex_bgezall(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch_likely_and_link(rs_contents >= 0, pc, offset, bit_mode)
}

fn ex_bgezl(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch_likely(rs_contents >= 0, pc, offset, bit_mode)
}

fn ex_bgtz(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch(rs_contents > 0, pc, offset, bit_mode)
}

fn ex_bgtzl(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch_likely(rs_contents > 0, pc, offset, bit_mode)
}

fn ex_blez(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch(rs_contents <= 0, pc, offset, bit_mode)
}

fn ex_blezl(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch_likely(rs_contents <= 0, pc, offset, bit_mode)
}

fn ex_bltz(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch(rs_contents < 0, pc, offset, bit_mode)
}

fn ex_bltzal(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch_and_link(rs_contents < 0, pc, offset, bit_mode)
}

fn ex_bltzall(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch_likely_and_link(rs_contents < 0, pc, offset, bit_mode)
}

fn ex_bltzl(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch_likely(rs_contents < 0, pc, offset, bit_mode)
}

fn ex_bne(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch(rs_contents != 0, pc, offset, bit_mode)
}

fn ex_bnel(rs_contents: i64, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    branch_likely(rs_contents != 0, pc, offset, bit_mode)
}

fn ex_break() -> ExOut {
    ExOut {
        ..Default::default() // Throw breakpoint exception
    }
}

fn branch(condition: bool, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    ExOut {
        new_pc: if condition {
            Some(match bit_mode {
                BitMode::ThirtyTwo => ((pc as i32) + ((offset as i16 as i32) * 4)) as u32 as u64, // zero extend
                BitMode::SixtyFour => ((pc as i64) + ((offset as i16 as i64) * 4)) as u64, // sign extend
            })
        } else {
            None
        },

        ..Default::default()
    }
}

fn branch_likely(condition: bool, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    if condition {
        ExOut {
            new_pc: Some(match bit_mode {
                BitMode::ThirtyTwo => ((pc as i32) + ((offset as i16 as i32) * 4)) as u32 as u64,
                BitMode::SixtyFour => ((pc as i64) + ((offset as i16 as i64) * 4)) as u64,
            }),
            ..Default::default()
        }
    } else {
        ExOut {
            discard_next_instruction: true,
            ..Default::default()
        }
    }
}

fn branch_and_link(condition: bool, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    ExOut {
        new_pc: if condition {
            Some(match bit_mode {
                BitMode::ThirtyTwo => ((pc as i32) + ((offset as i16 as i32) * 4)) as u32 as u64,
                BitMode::SixtyFour => ((pc as i64) + ((offset as i16 as i64) * 4)) as u64,
            })
        } else {
            None
        },
        writeback: Some(pc + 8),
        target_register: Some(31),
        ..Default::default()
    }
}

fn branch_likely_and_link(condition: bool, pc: u64, offset: u16, bit_mode: BitMode) -> ExOut {
    if condition {
        ExOut {
            new_pc: Some(match bit_mode {
                BitMode::ThirtyTwo => ((pc as i32) + ((offset as i16 as i32) * 4)) as u32 as u64,
                BitMode::SixtyFour => ((pc as i64) + ((offset as i16 as i64) * 4)) as u64,
            }),
            writeback: Some(pc + 8),
            target_register: Some(31),
            ..Default::default()
        }
    } else {
        ExOut {
            discard_next_instruction: true,
            writeback: Some(pc + 8),
            target_register: Some(31),
            ..Default::default()
        }
    }
}

fn ex_dadd(rs_contents: i64, rt_contents: i64, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: match rs_contents.checked_add(rt_contents) {
            Some(ans) => Some(ans as u64),
            None => Some(0), //Integer Overflow Exception
        },
        target_register: rd,
        ..Default::default()
    }
}

fn ex_daddi(rs_contents: i64, immediate: i16, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: match rs_contents.checked_add(immediate as i64) {
            Some(ans) => Some(ans as u64),
            None => Some(0), //Integer Overflow Exception
        },
        target_register: rd,
        ..Default::default()
    }
}

fn ex_daddiu(rs_contents: i64, immediate: i16, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some((rs_contents + (immediate as i64)) as u64),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_daddu(rs_contents: i64, rt_contents: i64, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some((rs_contents + rt_contents) as u64),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_ddiv(rs_contents: u64, rt_contents: u64) -> ExOut {
    ExOut {
        hi_writeback: Some((rs_contents as i64 / rt_contents as i64) as u64),
        lo_writeback: Some((rs_contents as i64 % rt_contents as i64) as u64),
        ..Default::default()
    }
}

fn ex_ddivu(rs_contents: u64, rt_contents: u64) -> ExOut {
    ExOut {
        hi_writeback: Some(rs_contents / rt_contents),
        lo_writeback: Some(rs_contents % rt_contents),
        ..Default::default()
    }
}

fn ex_div(rs_contents: u64, rt_contents: u64, bit_mode: BitMode) -> ExOut {
    match bit_mode {
        BitMode::ThirtyTwo => ExOut {
            hi_writeback: Some((rs_contents as i32 / rt_contents as i32) as u32 as u64),
            lo_writeback: Some((rs_contents as i32 % rt_contents as i32) as u32 as u64),
            ..Default::default()
        },
        BitMode::SixtyFour => ExOut {
            hi_writeback: Some((rs_contents as i32 / rt_contents as i32) as u64),
            lo_writeback: Some((rs_contents as i32 % rt_contents as i32) as u64),
            ..Default::default()
        },
    }
}

fn ex_divu(rs_contents: u64, rt_contents: u64) -> ExOut {
    ExOut {
        hi_writeback: Some((rs_contents as u32 / rt_contents as u32) as u64),
        lo_writeback: Some((rs_contents as u32 % rt_contents as u32) as u64),
        ..Default::default()
    }
}

fn ex_dmult(rs_contents: u64, rt_contents: u64) -> ExOut {
    let mult = rs_contents as i64 as i128 * rt_contents as i64 as i128;
    ExOut {
        hi_writeback: Some((mult >> 64) as u64),
        lo_writeback: Some(mult as u64),
        ..Default::default()
    }
}

fn ex_dmultu(rs_contents: u64, rt_contents: u64) -> ExOut {
    let mult = rs_contents as u128 * rt_contents as u128;
    ExOut {
        hi_writeback: Some((mult >> 64) as u64),
        lo_writeback: Some(mult as u64),
        ..Default::default()
    }
}

fn ex_dsll(rt_contents: u64, sa: u8, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some(rt_contents << sa),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_dsllv(rs_contents: u64, rt_contents: u64, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some(rt_contents << (rs_contents & 63)),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_dsll32(rt_contents: u64, sa: u8, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some(rt_contents << (sa + 32)),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_dsra(rt_contents: u64, sa: u8, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some(((rt_contents as i64) >> sa) as u64),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_dsrav(rs_contents: u64, rt_contents: u64, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some(((rt_contents as i64) << (rs_contents & 63)) as u64),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_dsra32(rt_contents: u64, sa: u8, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some(((rt_contents as i64) >> (sa + 32)) as u64),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_dsrl(rt_contents: u64, sa: u8, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some(rt_contents >> sa),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_dsrlv(rs_contents: u64, rt_contents: u64, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some(rt_contents >> (rs_contents & 63)),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_dsrl32(rt_contents: u64, sa: u8, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some(rt_contents >> (sa + 32)),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_dsub(rs_contents: i64, rt_contents: i64, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: match rs_contents.checked_sub(rt_contents) {
            Some(ans) => Some(ans as u64),
            None => Some(0), //Integer Overflow Exception
        },
        target_register: rd,
        ..Default::default()
    }
}

fn ex_dsubu(rs_contents: u64, rt_contents: u64, rd: Option<u8>) -> ExOut {
    ExOut {
        writeback: Some(rs_contents - rt_contents),
        target_register: rd,
        ..Default::default()
    }
}

//TODO: ERET

fn ex_j(pc: u64, target: u32, bit_mode: BitMode) -> ExOut {
    ExOut {
        new_pc: Some(match bit_mode {
            BitMode::ThirtyTwo => (((pc as u32) & 0xF000) | (target << 2)) as u64,
            BitMode::SixtyFour => (pc & 0xFFFF0000) | ((target as u64) << 2),
        }),
        ..Default::default()
    }
}

fn ex_jal(pc: u64, target: u32, bit_mode: BitMode) -> ExOut {
    ExOut {
        new_pc: Some(match bit_mode {
            BitMode::ThirtyTwo => (((pc as u32) & 0xF000) | (target << 2)) as u64,
            BitMode::SixtyFour => (pc & 0xFFFF0000) | ((target as u64) << 2),
        }),
        writeback: Some(pc + 8),
        target_register: Some(31),
        ..Default::default()
    }
}

fn ex_jalr(rs_contents: u64, rd: Option<u8>, pc: u64) -> ExOut {
    ExOut {
        new_pc: Some(rs_contents),
        writeback: Some(pc + 8),
        target_register: rd,
        ..Default::default()
    }
}

fn ex_jar(rs_contents: u64) -> ExOut {
    ExOut {
        new_pc: Some(rs_contents),
        ..Default::default()
    }
}

fn ex_lb(base_contents: u64, offset: u16, rt: Option<u8>, bit_mode: BitMode) -> ExOut {
    ExOut {
        load_input: Some(LoadInput {
            vaddr_writeback: match bit_mode {
                BitMode::ThirtyTwo => ((base_contents as i32) + (offset as i16 as i32)) as u32 as u64,
                BitMode::SixtyFour => ((base_contents as i64) + (offset as i16 as i64)) as u64,
            },
            target_register: rt.unwrap(),
            load_type: LoadType::SignedByte,
        }),
        ..Default::default()
    }
}

fn ex_lbu(base_contents: u64, offset: u16, rt: Option<u8>, bit_mode: BitMode) -> ExOut {
    ExOut {
        load_input: Some(LoadInput {
            vaddr_writeback: match bit_mode {
                BitMode::ThirtyTwo => ((base_contents as i32) + (offset as i16 as i32)) as u32 as u64,
                BitMode::SixtyFour => ((base_contents as i64) + (offset as i16 as i64)) as u64,
            },
            target_register: rt.unwrap(),
            load_type: LoadType::UnsignedByte,
        }),
        ..Default::default()
    }
}

fn ex_ld(base_contents: u64, offset: u16, rt: Option<u8>, bit_mode: BitMode) -> ExOut {
    ExOut {
        load_input: Some(LoadInput {
            vaddr_writeback: match bit_mode {
                BitMode::ThirtyTwo => ((base_contents as i32) + (offset as i16 as i32)) as u32 as u64,
                BitMode::SixtyFour => ((base_contents as i64) + (offset as i16 as i64)) as u64,
            },
            target_register: rt.unwrap(),
            load_type: LoadType::UnsignedDoubleword,
        }),
        ..Default::default()
    }
}
