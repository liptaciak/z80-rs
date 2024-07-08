use crate::cpu::processor::Processor;
use crate::memory::Memory;

use crate::io::handler::IoHandler;
use crate::instruction::set::{Flag, Instruction, RegisterPair};

///Executes a single instruction.
pub fn process_instruction(cpu: &mut Processor, memory: &mut Memory, io: &mut IoHandler, instruction: Instruction, operand: Vec<u8>) -> (String, Processor, Memory) {
    let str: String;

    match instruction {
        Instruction::NOP => {
            if !cpu.halted { cpu.pc = cpu.pc.wrapping_add(1) };

            str = String::from("NOP 0x00");
        },
        Instruction::LDBCNN => {
            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::BC, value);

            cpu.pc = cpu.pc.wrapping_add(3);

            str = String::from("LD BC, NN 0x01");
        },
        Instruction::INCB => {
            cpu.set_flag(Flag::PV, false);
            if cpu.b == 0x7F { cpu.set_flag(Flag::PV, true); }

            cpu.set_flag(Flag::H, false);
            if (cpu.b & 0x0F) == 0b1111 { cpu.set_flag(Flag::H, true); }

            cpu.b = cpu.b.wrapping_add(1);
            cpu.pc = cpu.pc.wrapping_add(1);

            cpu.set_flag(Flag::Z, false);
            if cpu.b == 0x00 { cpu.set_flag(Flag::Z, true); }

            cpu.set_flag(Flag::S, false);
            if cpu.b > 0x7F { cpu.set_flag(Flag::S, true); }

            cpu.set_flag(Flag::N, false);

            str = String::from("INC B 0x04");
        },
        Instruction::DECB => {
            cpu.set_flag(Flag::PV, false);
            if cpu.b == 0x80 { cpu.set_flag(Flag::PV, true); }

            cpu.set_flag(Flag::H, false);
            if (cpu.b & 0x0F) == 0b0000 { cpu.set_flag(Flag::H, true); }

            cpu.b = cpu.b.wrapping_sub(1);
            cpu.pc = cpu.pc.wrapping_add(1);

            cpu.set_flag(Flag::S, false);
            if cpu.b > 0x7F { cpu.set_flag(Flag::S, true); }

            cpu.set_flag(Flag::Z, false);
            if cpu.b == 0x00 { cpu.set_flag(Flag::Z, true); }

            cpu.set_flag(Flag::N, true);

            str = String::from("DEC B 0x05");
        },
        Instruction::LDBN => {
            cpu.b = operand[0];
            cpu.pc = cpu.pc.wrapping_add(2);

            str = String::from("LD B, N 0x06");
        },
        Instruction::LDDENN => {
            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::DE, value);

            cpu.pc = cpu.pc.wrapping_add(3);

            str = String::from("LD DE, NN 0x11");
        },
        Instruction::LDHLNNM => {
            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.set_pair(RegisterPair::HL, value);

            cpu.pc = cpu.pc.wrapping_add(3);

            str = String::from("LD HL, NN 0x21");
        },
        Instruction::LDMNNHL => {
            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);

            memory.write(value, cpu.l);
            memory.write(value + 1, cpu.h);

            cpu.pc = cpu.pc.wrapping_add(3);

            str = String::from("LD (NN), HL 0x22");
        },
        Instruction::INCHL => {
            let result: u16 = cpu.get_pair(RegisterPair::HL).wrapping_add(1);
            cpu.set_pair(RegisterPair::HL, result);

            cpu.pc = cpu.pc.wrapping_add(1);

            str = String::from("INC HL 0x23");
        },
        Instruction::JRZD => {
            if cpu.get_flag(Flag::Z) {
                let result: u16 = operand[0] as u16 + 2;
                cpu.pc = cpu.pc.wrapping_add(result);
            } else {
                cpu.pc = cpu.pc.wrapping_add(2);
            }

            str = String::from("JR Z, D 0x28");
        },
        Instruction::LDSPNN => {    
            let value: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.sp = value;

            cpu.pc = cpu.pc.wrapping_add(3);

            str = String::from("LD SP, NN 0x31");
        },
        Instruction::LDMHLN => {
            memory.write(cpu.get_pair(RegisterPair::HL), operand[0]);

            cpu.pc = cpu.pc.wrapping_add(2);

            str = String::from("LD (HL), N 0x36");
        },
        Instruction::LDAN => {
            cpu.a = operand[0];    
            cpu.pc = cpu.pc.wrapping_add(2);

            str = String::from("LD A, N 0x3E");
        },
        Instruction::LDBA => {
            cpu.b = cpu.a;
            cpu.pc = cpu.pc.wrapping_add(1);

            str = String::from("LD B, A 0x06");
        },
        Instruction::HALT => {
            cpu.pc = cpu.pc.wrapping_add(1);
            cpu.halted = true;

            str = String::from("HALT 0x76");
        },
        Instruction::ADDAB => {
            cpu.set_flag(Flag::PV, false);
            if cpu.a <= 0x7F && cpu.b > (0x80 as u8).wrapping_sub(cpu.a) { cpu.set_flag(Flag::PV, true); }

            cpu.set_flag(Flag::H, false);
            if (cpu.a & 0x0F) == 0b1111 { cpu.set_flag(Flag::H, true); }

            let overflow: bool = cpu.a.overflowing_add(cpu.b).1;
            if overflow { cpu.set_flag(Flag::C, true); }

            cpu.a = cpu.a.wrapping_add(cpu.b);
            cpu.pc = cpu.pc.wrapping_add(2);

            cpu.set_flag(Flag::Z, false);
            if cpu.a == 0x00 { cpu.set_flag(Flag::Z, true); }

            cpu.set_flag(Flag::S, false);
            if cpu.a > 0x7F { cpu.set_flag(Flag::S, true); }

            cpu.set_flag(Flag::N, false);

            str = String::from("ADD A, B 0x80");
        },
        Instruction::ADCAC => {
            cpu.set_flag(Flag::PV, false);
            if cpu.a <= 0x7F && cpu.c > (0x80 as u8).wrapping_sub(cpu.a) { cpu.set_flag(Flag::PV, true); }

            cpu.set_flag(Flag::H, false);
            if (cpu.a & 0x0F) == 0b1111 { cpu.set_flag(Flag::H, true); }

            let overflow: bool = cpu.a.overflowing_add(cpu.c + cpu.get_flag(Flag::C) as u8).1;
            if overflow { cpu.set_flag(Flag::C, true); }

            cpu.a = cpu.a.wrapping_add(cpu.c + cpu.get_flag(Flag::C) as u8);
            cpu.pc = cpu.pc.wrapping_add(1);

            cpu.set_flag(Flag::Z, false);
            if cpu.a == 0x00 { cpu.set_flag(Flag::Z, true); }

            cpu.set_flag(Flag::S, false);
            if cpu.a > 0x7F { cpu.set_flag(Flag::S, true); }

            cpu.set_flag(Flag::N, false);

            str = String::from("ADC A, C 0x89");
        },
        Instruction::CPB => {
            cpu.set_flag(Flag::PV, false);
            if cpu.a >= 0x80 && cpu.b > cpu.a.wrapping_sub(0x7F) { cpu.set_flag(Flag::PV, true); }

            cpu.set_flag(Flag::H, false);
            if (cpu.a & 0x0F) == 0b0000 { cpu.set_flag(Flag::H, true); }

            if cpu.a < cpu.b { cpu.set_flag(Flag::C, true); }

            let result: u8 = cpu.a.wrapping_sub(cpu.b);

            cpu.set_flag(Flag::S, false);
            if result > 0x7F { cpu.set_flag(Flag::S, true); }

            cpu.set_flag(Flag::Z, false);
            if result == 0x00 { cpu.set_flag(Flag::Z, true); }

            cpu.pc = cpu.pc.wrapping_add(1);

            cpu.set_flag(Flag::N, true);

            str = String::from("CP B 0xB8");
        },
        Instruction::JPNN => {
            let address: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.pc = address;

            str = String::from("JP NN 0xC3");
        },
        Instruction::CALLNZNN => {
            cpu.pc = cpu.pc.wrapping_add(3);
        
            if !cpu.get_flag(Flag::Z) {
                memory.write(cpu.sp.wrapping_sub(1), (cpu.pc >> 8) as u8);
                memory.write(cpu.sp.wrapping_sub(2), cpu.pc as u8);
            
                cpu.sp = cpu.sp.wrapping_sub(2);
            
                let address: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
                cpu.pc = address;
            }

            str = String::from("CALL NZ, NN 0xC4");
        },
        Instruction::ADDAN => {
            cpu.set_flag(Flag::PV, false);
            if cpu.a <= 0x7F && operand[0] > (0x80 as u8).wrapping_sub(cpu.a) { cpu.set_flag(Flag::PV, true); }

            cpu.set_flag(Flag::H, false);
            if (cpu.a & 0x0F) == 0b1111 { cpu.set_flag(Flag::H, true); }

            let overflow: bool = cpu.a.overflowing_add(operand[0]).1;
            if overflow { cpu.set_flag(Flag::C, true); }

            cpu.a = cpu.a.wrapping_add(operand[0]);
            cpu.pc = cpu.pc.wrapping_add(2);

            cpu.set_flag(Flag::Z, false);
            if cpu.a == 0x00 { cpu.set_flag(Flag::Z, true); }

            cpu.set_flag(Flag::S, false);
            if cpu.a > 0x7F { cpu.set_flag(Flag::S, true); }

            cpu.set_flag(Flag::N, false);

            str = String::from("ADD A, N 0xC6");
        },
        Instruction::RET => {
            let address: u16 = ((memory.read(cpu.sp.wrapping_add(1)) as u16) << 8) |
            (memory.read(cpu.sp) as u16);
            cpu.pc = address;
        
            cpu.sp = cpu.sp.wrapping_add(2);
        
            str = String::from("RET 0xC9");
        },
        Instruction::JPZNN => {
            let address: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);

            if cpu.get_flag(Flag::Z) { 
                cpu.pc = address; 
            } else {
                cpu.pc = cpu.pc.wrapping_add(3);
            }

            str = String::from("JP Z, NN 0xCA");
        },
        Instruction::CALLNN => {
            cpu.pc = cpu.pc.wrapping_add(3);
        
            memory.write(cpu.sp.wrapping_sub(1), (cpu.pc >> 8) as u8);
            memory.write(cpu.sp.wrapping_sub(2), cpu.pc as u8);
        
            cpu.sp = cpu.sp.wrapping_sub(2);
        
            let address: u16 = ((operand[1] as u16) << 8) | (operand[0] as u16);
            cpu.pc = address;
        
            str = String::from("CALL NN 0xCD");
        },
        Instruction::OUTNA => {
            io.write(operand[0], cpu.a);
            cpu.pc = cpu.pc.wrapping_add(2);

            str = String::from("OUT N, A 0xD3");
        },
        Instruction::SUBN => {
            cpu.set_flag(Flag::PV, false);
            if cpu.a >= 0x80 && operand[0] > cpu.a.wrapping_sub(0x7F) { cpu.set_flag(Flag::PV, true); }

            cpu.set_flag(Flag::H, false);
            if (cpu.a & 0x0F) == 0b0000 { cpu.set_flag(Flag::H, true); }

            if cpu.a < operand[0] { cpu.set_flag(Flag::C, true); }

            cpu.a = cpu.a.wrapping_sub(operand[0]);
            cpu.pc = cpu.pc.wrapping_add(2);

            cpu.set_flag(Flag::S, false);
            if cpu.a > 0x7F { cpu.set_flag(Flag::S, true); }

            cpu.set_flag(Flag::Z, false);
            if cpu.a == 0x00 { cpu.set_flag(Flag::Z, true); }

            cpu.set_flag(Flag::N, true);

            str = String::from("SUB A, N 0xD6");
        },
        Instruction::INAN => {
            cpu.a = io.read(operand[0]);
            cpu.pc = cpu.pc.wrapping_add(2);

            str = String::from("IN A, N 0xDB");
        },
        Instruction::RETPO => {
            if !cpu.get_flag(Flag::PV) {
                let address: u16 = ((memory.read(cpu.sp.wrapping_add(1)) as u16) << 8) | 
                (memory.read(cpu.sp) as u16);
                cpu.pc = address;
            
                cpu.sp = cpu.sp.wrapping_add(2);
            } else {
                cpu.pc = cpu.pc.wrapping_add(1);
            }

            str = String::from("RET PO 0xE0");
        }
        Instruction::DI => {
            cpu.iff1 = false;
            cpu.iff2 = false;
            
            cpu.pc = cpu.pc.wrapping_add(1);

            str = String::from("DI 0xF3");
        },
        Instruction::EI => {
            cpu.iff1 = true;
            cpu.iff2 = true;

            cpu.pc = cpu.pc.wrapping_add(1);

            str = String::from("EI 0xFB");
        },
        Instruction::CPN => {
            cpu.set_flag(Flag::PV, false);
            if cpu.a >= 0x80 && operand[0] > cpu.a.wrapping_sub(0x7F) { cpu.set_flag(Flag::PV, true); }

            cpu.set_flag(Flag::H, false);
            if (cpu.a & 0x0F) == 0b0000 { cpu.set_flag(Flag::H, true); }

            if cpu.a < cpu.b { cpu.set_flag(Flag::C, true); }

            let result: u8 = cpu.a.wrapping_sub(operand[0]);

            cpu.set_flag(Flag::S, false);
            if result > 0x7F { cpu.set_flag(Flag::S, true); }

            cpu.set_flag(Flag::Z, false);
            if result == 0x00 { cpu.set_flag(Flag::Z, true); }

            cpu.pc = cpu.pc.wrapping_add(2);

            cpu.set_flag(Flag::N, true);

            str = String::from("CP N 0xFE");
        },
        _ => panic!("Instruction {:?} not implemented.", instruction),
    }

    (str, cpu.to_owned(), memory.to_owned())
}