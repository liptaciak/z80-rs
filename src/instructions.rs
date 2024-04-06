use crate::CPU;

#[allow(dead_code)]
enum AddressMode {
    IMPLIED,
    ADDRESING,
}

#[allow(dead_code)]
pub struct Instruction {
    opcode: u8,
    address_mode: AddressMode,
}

pub const NOP: Instruction = Instruction { opcode: 00, address_mode: AddressMode::IMPLIED };
pub const LDBB: Instruction = Instruction { opcode: 40, address_mode: AddressMode::ADDRESING };
pub const LDBC: Instruction = Instruction { opcode: 41, address_mode: AddressMode::ADDRESING };

#[allow(dead_code)]
pub enum Instructions {
    Nop,
    Ldbb,
    Ldbc,
}

pub fn process(mut cpu: CPU, instructions: Instructions) {
    match instructions {
        Instructions::Nop{..} => {
            println!("NOP {0}", NOP.opcode)
        },
        Instructions::Ldbb{..} => {
            cpu.b = cpu.b;

            println!("LDBB {0} | B: {1} -> B: {2}", LDBB.opcode, cpu.b, cpu.b);
        },
        Instructions::Ldbc{..} => {
            cpu.b = cpu.c;
        
            println!("LDBC {0} | C: {1} -> B: {2}", LDBC.opcode, cpu.c, cpu.b);
        },
    }
}