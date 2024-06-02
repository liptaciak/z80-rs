use crate::{CPU, AddressMode, RegisterPair};

#[allow(dead_code)]
pub enum Instruction {
    NOP,
    INCB,
    HALT,
    CPB,
    LDBCNN,
    LDBB, LDBC,
    LDAN, LDBN, LDCN, LDDN, LDEN, LDHN, LDLN,
    JPNN, JRZD,
}

pub fn match_instruction(i: u8) -> (Instruction, AddressMode) {
    match i {
        0x00 => {( Instruction::NOP, AddressMode::None )},
        0x01 => {( Instruction::LDBCNN, AddressMode::ImmediateExtended )},
        0x04 => {( Instruction::INCB, AddressMode::Register )},
        0x06 => {( Instruction::LDBN, AddressMode::Immediate )},
        0x0E => {( Instruction::LDCN, AddressMode::Immediate )},
        0x28 => {( Instruction::JRZD, AddressMode::Immediate )},
        0x29 => {( Instruction::LDBC, AddressMode::Register )},
        0x3E => {( Instruction::LDAN, AddressMode::Immediate )},
        0x76 => {( Instruction::HALT, AddressMode::None )},
        0xB8 => {( Instruction::CPB, AddressMode::Register )},
        0xC3 => {( Instruction::JPNN, AddressMode::Extended )},
        _ => panic!("Instruction {} not supported.", i),
    }
}

pub fn process(cpu: &mut CPU, instruction: Instruction, operand: Vec<u8>) {
    match instruction {
        Instruction::NOP => {
            cpu.pc += 1;

            println!("NOP | 0x00\n");
        },
        Instruction::LDBC => {
            cpu.b = cpu.c;

            cpu.pc += 1;
        
            println!("LD B, C | 0x29 | C: {0} -> B: {1}\n", cpu.c, cpu.b);
        },
        Instruction::LDAN => {
            cpu.a = operand[0];
            
            cpu.pc += 2;

            println!("LD A, N | 0x3E | N: {0} -> A: {1}\n", operand[0], cpu.a);
        },
        Instruction::LDBN => {
            cpu.b = operand[0];

            cpu.pc += 2;

            println!("LD B, N | 0x06 | N: {0} -> B: {1}\n", operand[0], cpu.b);
        },
        Instruction::LDCN => {
            cpu.c = operand[0];

            cpu.pc += 3;

            println!("LD C, N | 0xOE | N: {0} -> C: {1}\n", operand[0], cpu.c);
        },
        Instruction::LDBCNN => {
            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::BC, value);

            cpu.pc += 3;

            println!("LD BC, NN | 0x01 | NN: {0} -> BC: {1}\n", value, cpu.get_pair(RegisterPair::BC));
        },
        Instruction::HALT => {
            println!("HALT | 0x76\n");

            cpu.pc += 1;

            loop { }; //Wait for interrupt (infinite loop)
        },
        Instruction::JPNN => {
            let address: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.pc = address;

            println!("JP NN | 0xC3 | NN: {0} -> PC: {1}\n", address, cpu.pc)
        },
        Instruction::CPB => {
            if cpu.b > cpu.a {
                cpu.f = 0b10000010; //SN Flag
            } else {
                let value: u8 = cpu.a - cpu.b;

                if value == 0 {
                    cpu.f = 0b01000010; //ZN Flag
                }
            }

            cpu.pc += 1;

            println!("CP B | 0xB8 | B: {0} - A: {1} -> F: {2}\n", cpu.b, cpu.a, cpu.f)
        },
        Instruction::JRZD => {
            if cpu.f == 0b01000010 {
                cpu.pc += operand[0] as u16 + 2;

                println!("JR Z, D | 0x28 | D: {0} -> PC: {1}\n", operand[0], cpu.pc)
            } else {
                cpu.pc += 2;
            }
        },
        Instruction::INCB => {
            cpu.b += 1;

            cpu.pc += 1;

            println!("INC B | 0x04 | B: {0}\n", cpu.b)
        },
        _ => panic!("Instruction not implemented."),
    }
}