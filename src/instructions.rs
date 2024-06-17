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
pub fn process_instruction(cpu: &mut CPU, ram: &mut Vec<u8>, instruction: Instruction, operand: Vec<u8>) -> String {
    match instruction {
        Instruction::NOP => {
            cpu.pc += 1;

            return String::from("NOP 0x00");
        },
        Instruction::LDBCNN => {
            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::BC, value);

            cpu.pc += 3;

            return String::from("LD BC, NN 0x01");
        },
        Instruction::INCB => {
            cpu.b += 1;
            cpu.pc += 1;

            return String::from("INC B 0x04");
        },
        Instruction::DECB => {
            if cpu.b == 0 {
                cpu.f = 0b10000010;
            } else {
                cpu.b -= 1;
            }

            cpu.pc += 1;

            return String::from("DEC B 0x05");
        },
        Instruction::LDBN => {
            cpu.b = operand[0];
            cpu.pc += 2;

            return String::from("LD B, N 0x06");
        },
        Instruction::LDDENN => {
            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::DE, value);

            cpu.pc += 3;

            return String::from("LD DE, NN 0x11");
        },
        Instruction::LDHLNN => {
            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::HL, value);

            cpu.pc += 3;

            return String::from("LD HL, NN 0x21");
        },
        Instruction::LDNNHL => {
            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);

            ram[value as usize] = cpu.l;
            ram[(value + 1) as usize] = cpu.h;

            cpu.pc += 3;

            return String::from("LD (NN), HL 0x22");
        },
        Instruction::INCHL => {
            cpu.set_pair(RegisterPair::HL, cpu.get_pair(RegisterPair::HL) + 1);
            cpu.pc += 1;

            return String::from("INC HL 0x23");
        },
        Instruction::JRZD => {
            if cpu.f == 0b01000010 {
                cpu.pc += operand[0] as u16 + 2;
            } else {
                cpu.pc += 2;
            }

            return String::from("JR Z, D 0x28");
        },
        Instruction::LDSPNN => {    
            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.sp = value;

            cpu.pc += 3;

            return String::from("LD SP, NN 0x31");
        },
        Instruction::LDHLN => {
            ram[cpu.get_pair(RegisterPair::HL) as usize] = operand[0];

            cpu.pc += 2;

            return String::from("LD (HL), N 0x36");
        },
        Instruction::LDAN => {
            cpu.a = operand[0];    
            cpu.pc += 2;

            return String::from("LD A, N 0x3E");
        },
        Instruction::LDBA => {
            cpu.b = cpu.a;
            cpu.pc += 1;

            return String::from("LD B, A 0x06");
        },
        Instruction::HALT => {
            cpu.pc += 1;

            loop { }; //Wait for interrupt (infinite loop)
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

            return String::from("CP B 0xB8");
        },
        Instruction::JPNN => {
            let address: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.pc = address;

            return String::from("JP NN 0xC3");
        },
        Instruction::ADDAN => {
            cpu.a += operand[0];
            cpu.pc += 2;

            return String::from("ADD A, N 0xC6");
        },
        Instruction::RET => {
            let address: u16 = ((ram[(cpu.sp + 1) as usize] as u16) << 8) | (ram[cpu.sp as usize] as u16);
            cpu.pc = address;
        
            cpu.sp += 2;
        
            return String::from("RET 0xC9");
        },
        Instruction::CALLNN => {
            cpu.pc += 3;
        
            ram[(cpu.sp - 1) as usize] = (cpu.pc >> 8) as u8;
            ram[(cpu.sp - 2) as usize] = cpu.pc as u8;
        
            cpu.sp -= 2;
        
            let address: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.pc = address;
        
            return String::from("CALL NN 0xCD");
        },
        Instruction::OUTNA => {
            //Write A to port N
            cpu.pc += 2;

            return String::from("OUT N, A 0xD3");
        },
        Instruction::SUBN => {
            if cpu.a < operand[0] {
                cpu.f = 0b10000010; //SN Flag
            } else {
                cpu.a -= operand[0];
            }

            cpu.pc += 2;

            return String::from("SUB A, N 0xD6");
        },
        Instruction::DI => {
            //Prevent maskable interrupts from triggering
            cpu.pc += 1;

            return String::from("DI 0xF3");
        },

        #[allow(unreachable_patterns)]
        _ => panic!("Instruction not implemented."),
    }
}