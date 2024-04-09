use crate::{Instruction, AddressMode};

pub fn match_instruction(i: &u8) -> (Instruction, AddressMode) {
    match i {
        0 => {( Instruction::Nop, AddressMode::Implied )},
        40 => {( Instruction::Ldbb, AddressMode::Register )},
        41 => {( Instruction::Ldbc, AddressMode::Register )},
        62 => {( Instruction::Ldan, AddressMode::Immediate )},
        06 => {( Instruction::Ldbn, AddressMode::Immediate )},
        14 => {( Instruction::Ldcn, AddressMode::Immediate )},
        _ => panic!("Instruction {} not supported.", i),
    }
}