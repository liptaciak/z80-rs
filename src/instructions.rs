use crate::{CPU, AddressMode, RegisterPair};

#[allow(dead_code)]
pub enum Instruction {
    NOP,
    HALT,
    LDBCNN,
    LDBB, LDBC,
    LDAN, LDBN, LDCN, LDDN, LDEN, LDHN, LDLN,
    JPNN,
}

pub fn match_instruction(i: u8) -> (Instruction, AddressMode) {
    match i {
        0x00 => {( Instruction::NOP, AddressMode::None )},
        0x01 => {( Instruction::LDBCNN, AddressMode::ImmediateExtended )},
        0x28 => {( Instruction::LDBB, AddressMode::Register )},
        0x29 => {( Instruction::LDBC, AddressMode::Register )},
        0x3E => {( Instruction::LDAN, AddressMode::Immediate )},
        0x06 => {( Instruction::LDBN, AddressMode::Immediate )},
        0x0E => {( Instruction::LDCN, AddressMode::Immediate )},
        0x76 => {( Instruction::HALT, AddressMode::None )}
        0xC3 => {( Instruction::JPNN, AddressMode::ImmediateExtended )}
        _ => panic!("Instruction {} not supported.", i),
    }
}

pub fn process(cpu: &mut CPU, instruction: Instruction, operand: Vec<u8>) {
    match instruction {
        Instruction::NOP => {
            cpu.pc += 1;

            println!("NOP | 00\n");
        },
        Instruction::LDBB => {
            cpu.b = cpu.b;

            cpu.pc += 1;

            println!("LD B, B | 28 | B: {0} -> B: {1}\n", cpu.b, cpu.b);
        },
        Instruction::LDBC => {
            cpu.b = cpu.c;

            cpu.pc += 1;
        
            println!("LD B, C | 29 | C: {0} -> B: {1}\n", cpu.c, cpu.b);
        },
        Instruction::LDAN => {
            cpu.a = operand[0];
            
            cpu.pc += 1;

            println!("LD A, N | 3E | N: {0} -> A: {1}\n", operand[0], cpu.a);
        },
        Instruction::LDBN => {
            cpu.b = operand[0];

            cpu.pc += 1;

            println!("LD B, N | 06 | N: {0} -> B: {1}\n", operand[0], cpu.b);
        },
        Instruction::LDCN => {
            cpu.c = operand[0];

            cpu.pc += 1;

            println!("LD C, N | OE | N: {0} -> C: {1}\n", operand[0], cpu.c);
        },
        Instruction::LDBCNN => {
            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::BC, value);

            cpu.pc += 1;

            println!("LD BC, NN | 01 | NN: {0} -> BC: {1}\n", value, cpu.get_pair(RegisterPair::BC));
        },
        Instruction::HALT => {
            println!("HALT | 76\n");

            loop { }; //Wait for interrupt (infinite loop)
        },
        Instruction::JPNN => {
            let address: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.pc = address;

            println!("JP NN | C3 | NN: {0} -> PC: {1}\n", address, cpu.pc)
        },
        _ => panic!("Instruction not implemented."),
    }
}