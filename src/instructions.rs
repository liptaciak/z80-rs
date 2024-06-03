use crate::{CPU, AddressMode, RegisterPair};

#[allow(dead_code)]
pub enum Instruction {
    NOP,
    LDBCNN,
    INCB,
    DECB,
    LDBN,
    LDCN,
    LDDENN,
    LDHLNN,
    JRZD,
    LDBC,
    LDAN,
    LDBA,
    HALT,
    CPB,
    JPNN,
    ADDAN,
    SUBN,
}

pub fn match_instruction(instr: u8) -> (Instruction, AddressMode) {
    match instr {
        0x00 => {( Instruction::NOP, AddressMode::None )},
        0x01 => {( Instruction::LDBCNN, AddressMode::ImmediateExtended )},
        0x04 => {( Instruction::INCB, AddressMode::Register )},
        0x05 => {( Instruction::DECB, AddressMode::Register )},
        0x06 => {( Instruction::LDBN, AddressMode::Immediate )},
        0x11 => {( Instruction::LDDENN, AddressMode::ImmediateExtended )},
        0x21 => {( Instruction::LDHLNN, AddressMode::ImmediateExtended )},
        0x28 => {( Instruction::JRZD, AddressMode::Immediate )},
        0x3E => {( Instruction::LDAN, AddressMode::Immediate )},
        0x47 => {( Instruction::LDBA, AddressMode::Register )},
        0x76 => {( Instruction::HALT, AddressMode::None )},
        0xB8 => {( Instruction::CPB, AddressMode::Register )},
        0xC3 => {( Instruction::JPNN, AddressMode::Extended )},
        0xC6 => {( Instruction::ADDAN, AddressMode::Immediate )},
        0xD6 => {( Instruction::SUBN, AddressMode::Immediate )},
        _ => panic!("Instruction {} not supported.", instr),
    }
}

//TODO: Add flags.
pub fn process(cpu: &mut CPU, instruction: Instruction, operand: Vec<u8>) {
    match instruction {
        Instruction::NOP => {
            println!("PC: {}", cpu.pc);
            println!("NOP | 0x00\n");

            cpu.pc += 1;
        },
        Instruction::LDBCNN => {
            println!("PC: {}", cpu.pc);

            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::BC, value);

            cpu.pc += 3;

            println!("LD BC, NN | 0x01 | NN: {0} -> BC: {1}\n", value, cpu.get_pair(RegisterPair::BC));
        },
        Instruction::INCB => {
            println!("PC: {}", cpu.pc);

            cpu.b += 1;
            cpu.pc += 1;

            println!("INC B | 0x04 | B: {0}\n", cpu.b);
        },
        Instruction::DECB => {
            if cpu.b == 0 {
                cpu.f = 0b10000010;
            } else {
                println!("PC: {}", cpu.pc);

                cpu.b -= 1;
                cpu.pc += 1;

                println!("DEC B | 0x05 | B: {0}\n", cpu.b);
            }
        },
        Instruction::LDBN => {
            println!("PC: {}", cpu.pc);

            cpu.b = operand[0];
            cpu.pc += 2;

            println!("LD B, N | 0x06 | N: {0} -> B: {1}\n", operand[0], cpu.b);
        },
        Instruction::LDDENN => {
            println!("PC: {}", cpu.pc);

            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::DE, value);

            cpu.pc += 3;

            println!("LD DE, NN | 0x11 | NN: {0} -> DE: {1}\n", value, cpu.get_pair(RegisterPair::DE));
        },
        Instruction::LDHLNN => {
            println!("PC: {}", cpu.pc);

            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::HL, value);

            cpu.pc += 3;

            println!("LD HL, NN | 0x21 | NN: {0} -> HL: {1}\n", value, cpu.get_pair(RegisterPair::HL));
        },
        Instruction::JRZD => {
            if cpu.f == 0b01000010 {
                println!("PC: {}", cpu.pc);
                cpu.pc += operand[0] as u16 + 2;

                println!("JR Z, D | 0x28 | D: {0} -> PC: {1}\n", operand[0], cpu.pc);
            } else {
                cpu.pc += 2;
            }
        },
        Instruction::LDAN => {
            println!("PC: {}", cpu.pc);

            cpu.a = operand[0];    
            cpu.pc += 2;

            println!("LD A, N | 0x3E | N: {0} -> A: {1}\n", operand[0], cpu.a);
        },
        Instruction::LDBA => {
            println!("PC: {}", cpu.pc);

            cpu.b = cpu.a;
            cpu.pc += 1;

            println!("LD B, A | 0x06 | A: {0} -> B: {1}\n", cpu.a, cpu.b);
        },
        Instruction::HALT => {
            println!("PC: {}", cpu.pc);
            println!("HALT | 0x76\n");

            cpu.pc += 1;

            loop { }; //Wait for interrupt (infinite loop)
        },
        Instruction::CPB => {
            println!("PC: {}", cpu.pc);

            if cpu.b > cpu.a {
                cpu.f = 0b10000010; //SN Flag
            } else {
                let value: u8 = cpu.a - cpu.b;

                if value == 0 {
                    cpu.f = 0b01000010; //ZN Flag
                }
            }

            cpu.pc += 1;

            println!("CP B | 0xB8 | B: {0} - A: {1} -> F: {2}\n", cpu.b, cpu.a, cpu.f);
        },
        Instruction::JPNN => {
            println!("PC: {}", cpu.pc);

            let address: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.pc = address;

            println!("JP NN | 0xC3 | NN: {0} -> PC: {1}\n", address, cpu.pc);
        },
        Instruction::ADDAN => {
            println!("PC: {}", cpu.pc);
            println!("ADD A, N | 0xC6 | A: {0} + N: {1}\n", cpu.a, operand[0]);

            cpu.a += operand[0];
            cpu.pc += 2;
        },
        Instruction::SUBN => {
            if cpu.a < operand[0] {
                cpu.f = 0b10000010; //SN Flag
            } else {
                println!("PC: {}", cpu.pc);
                println!("SUB A, N | 0xD6 | A: {0} - N: {1}\n", cpu.a, operand[0]);
                
                cpu.a -= operand[0];
            }

            cpu.pc += 2;
        },

        #[allow(unreachable_patterns)]
        _ => panic!("Instruction not implemented."),
    }
}