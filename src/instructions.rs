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
    LDNNHL,
    INCHL,
    JRZD,
    LDSPNN,
    LDHLN,
    LDBC,
    LDAN,
    LDBA,
    HALT,
    CPB,
    JPNN,
    ADDAN,
    RET,
    CALLNN,
    OUTNA,
    SUBN,
    DI,
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
        0x22 => {( Instruction::LDNNHL, AddressMode::Extended )},
        0x23 => {( Instruction::INCHL, AddressMode::Register )},
        0x28 => {( Instruction::JRZD, AddressMode::Immediate )},
        0x31 => {( Instruction::LDSPNN, AddressMode::Extended )},
        0x36 => {( Instruction::LDHLN, AddressMode::Immediate )},
        0x3E => {( Instruction::LDAN, AddressMode::Immediate )},
        0x47 => {( Instruction::LDBA, AddressMode::Register )},
        0x76 => {( Instruction::HALT, AddressMode::None )},
        0xB8 => {( Instruction::CPB, AddressMode::Register )},
        0xC3 => {( Instruction::JPNN, AddressMode::Extended )},
        0xC6 => {( Instruction::ADDAN, AddressMode::Immediate )},
        0xC9 => {( Instruction::RET, AddressMode::None )},
        0xCD => {( Instruction::CALLNN, AddressMode::Extended )},
        0xD3 => {( Instruction::OUTNA, AddressMode::Immediate )},
        0xD6 => {( Instruction::SUBN, AddressMode::Immediate )},
        0xF3 => {( Instruction::DI, AddressMode::None )},
        _ => panic!("Instruction {:#X} not supported.", instr),
    }
}

//TODO: Add flags.
pub fn process(cpu: &mut CPU, ram: &mut Vec<u8>, instruction: Instruction, operand: Vec<u8>) {
    match instruction {
        Instruction::NOP => {
            print!("PC: {} | ", cpu.pc);
            println!("NOP | 0x00");

            cpu.pc += 1;
        },
        Instruction::LDBCNN => {
            print!("PC: {} | ", cpu.pc);

            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::BC, value);

            cpu.pc += 3;

            println!("LD BC, NN | 0x01 | NN: {0} -> BC: {1}", value, cpu.get_pair(RegisterPair::BC));
        },
        Instruction::INCB => {
            print!("PC: {} | ", cpu.pc);

            cpu.b += 1;
            cpu.pc += 1;

            println!("INC B | 0x04 | B: {0}", cpu.b);
        },
        Instruction::DECB => {
            if cpu.b == 0 {
                cpu.f = 0b10000010;
            } else {
                print!("PC: {} | ", cpu.pc);

                cpu.b -= 1;

                println!("DEC B | 0x05 | B: {0}", cpu.b);
            }

            cpu.pc += 1;
        },
        Instruction::LDBN => {
            print!("PC: {} | ", cpu.pc);

            cpu.b = operand[0];
            cpu.pc += 2;

            println!("LD B, N | 0x06 | N: {0} -> B: {1}", operand[0], cpu.b);
        },
        Instruction::LDDENN => {
            print!("PC: {} | ", cpu.pc);

            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::DE, value);

            cpu.pc += 3;

            println!("LD DE, NN | 0x11 | NN: {0} -> DE: {1}", value, cpu.get_pair(RegisterPair::DE));
        },
        Instruction::LDHLNN => {
            print!("PC: {} | ", cpu.pc);

            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::HL, value);

            cpu.pc += 3;

            println!("LD HL, NN | 0x21 | NN: {0} -> HL: {1}", value, cpu.get_pair(RegisterPair::HL));
        },
        Instruction::LDNNHL => {
            print!("PC: {} | ", cpu.pc);

            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);

            ram[value as usize] = cpu.l;
            ram[(value + 1) as usize] = cpu.h;

            let result: u16 = ((ram[value as usize] as u16) << 8) | (ram[(value + 1) as usize] as u16);

            cpu.pc += 3;

            println!("LD (NN), HL | 0x22 | HL: {0} -> NN: {1}", cpu.get_pair(RegisterPair::HL), result)
        },
        Instruction::INCHL => {
            print!("PC: {} | ", cpu.pc);

            cpu.set_pair(RegisterPair::HL, cpu.get_pair(RegisterPair::HL) + 1);
            cpu.pc += 1;

            println!("INC HL | 0x23 | HL: {0}", cpu.get_pair(RegisterPair::HL));
        },
        Instruction::JRZD => {
            if cpu.f == 0b01000010 {
                print!("PC: {} | ", cpu.pc);
                cpu.pc += operand[0] as u16 + 2;

                println!("JR Z, D | 0x28 | D: {0} -> PC: {1}", operand[0], cpu.pc);
            } else {
                cpu.pc += 2;
            }
        },
        Instruction::LDSPNN => {
            print!("PC: {} | ", cpu.pc);
            
            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.sp = value;

            cpu.pc += 3;

            println!("LD SP, NN | 0x31 | NN: {0} -> SP: {1}", value, cpu.sp);
        },
        Instruction::LDHLN => {
            print!("PC: {} | ", cpu.pc);

            ram[cpu.get_pair(RegisterPair::HL) as usize] = operand[0];

            cpu.pc += 2;

            println!("LD (HL), N | 0x36 | N: {0} -> (HL): {1}", operand[0], cpu.get_pair(RegisterPair::HL))
        },
        Instruction::LDAN => {
            print!("PC: {} | ", cpu.pc);

            cpu.a = operand[0];    
            cpu.pc += 2;

            println!("LD A, N | 0x3E | N: {0} -> A: {1}", operand[0], cpu.a);
        },
        Instruction::LDBA => {
            print!("PC: {} | ", cpu.pc);

            cpu.b = cpu.a;
            cpu.pc += 1;

            println!("LD B, A | 0x06 | A: {0} -> B: {1}", cpu.a, cpu.b);
        },
        Instruction::HALT => {
            print!("PC: {} | ", cpu.pc);
            println!("HALT | 0x76");

            cpu.pc += 1;

            loop { }; //Wait for interrupt (infinite loop)
        },
        Instruction::CPB => {
            print!("PC: {} | ", cpu.pc);

            if cpu.b > cpu.a {
                cpu.f = 0b10000010; //SN Flag
            } else {
                let value: u8 = cpu.a - cpu.b;

                if value == 0 {
                    cpu.f = 0b01000010; //ZN Flag
                }
            }

            cpu.pc += 1;

            println!("CP B | 0xB8 | B: {0} - A: {1} -> F: {2}", cpu.b, cpu.a, cpu.f);
        },
        Instruction::JPNN => {
            print!("PC: {} | ", cpu.pc);

            let address: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.pc = address;

            println!("JP NN | 0xC3 | NN: {0} -> PC: {1}", address, cpu.pc);
        },
        Instruction::ADDAN => {
            print!("PC: {} | ", cpu.pc);
            println!("ADD A, N | 0xC6 | A: {0} + N: {1}", cpu.a, operand[0]);

            cpu.a += operand[0];
            cpu.pc += 2;
        },
        Instruction::RET => {
            print!("PC: {} | ", cpu.pc);
        
            let address: u16 = ((ram[(cpu.sp + 1) as usize] as u16) << 8) | (ram[cpu.sp as usize] as u16);
            cpu.pc = address;
        
            cpu.sp += 2;
        
            println!("RET | 0xC9 | PC: {0}", address);
        },
        Instruction::CALLNN => {
            print!("PC: {} | ", cpu.pc);
        
            cpu.pc += 3;
        
            ram[(cpu.sp - 1) as usize] = (cpu.pc >> 8) as u8;
            ram[(cpu.sp - 2) as usize] = cpu.pc as u8;
        
            cpu.sp -= 2;
        
            let address: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.pc = address;
        
            println!("CALL NN | 0xCD | NN: {0} -> PC: {1}", address, cpu.pc);
        },
        Instruction::OUTNA => {
            print!("PC: {} | ", cpu.pc);
            println!("OUT N, A | 0xD3 | A: {0} -> N: {1}", cpu.a, operand[0]);

            //Write A to port N
            cpu.pc += 2;
        },
        Instruction::SUBN => {
            if cpu.a < operand[0] {
                cpu.f = 0b10000010; //SN Flag
            } else {
                print!("PC: {} | ", cpu.pc);
                println!("SUB A, N | 0xD6 | A: {0} - N: {1}", cpu.a, operand[0]);
                
                cpu.a -= operand[0];
            }

            cpu.pc += 2;
        },
        Instruction::DI => {
            print!("PC: {} | ", cpu.pc);
            println!("DI | 0xF3"); //Prevent maskable interrupts from triggering.

            cpu.pc += 1;
        },

        #[allow(unreachable_patterns)]
        _ => panic!("Instruction not implemented."),
    }
}