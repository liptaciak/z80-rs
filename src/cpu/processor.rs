use crate::instruction::implementation::process_instruction;
use crate::instruction::set::{Instruction, AddressMode, RegisterPair, INSTRUCTIONS};

use crate::memory::Memory;
use crate::io::handler::IoHandler;

use inline_colorization::*;

///Define CPU fields
#[derive(Default, Clone)]
pub struct Processor {
    pub halted: bool,
    pub a: u8, pub f: u8,
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    pub i: u8, pub r: u8,
    pub ix: u16, 
    pub iy: u16,
    pub sp: u16,
    pub pc: u16,
    pub iff1: bool, 
    pub iff2: bool,
}

///CPU implementation
impl Processor {
    ///Get register pair value
    pub fn get_pair(&self, pair: RegisterPair) -> u16 {
        match pair {
            RegisterPair::BC => ((self.b as u16) << 8) | (self.c as u16),
            RegisterPair::DE => ((self.d as u16) << 8) | (self.e as u16),
            RegisterPair::HL => ((self.h as u16) << 8) | (self.l as u16),
        }
    }

    ///Set register pair value
    pub fn set_pair(&mut self, pair: RegisterPair, value: u16) {
        match pair {
            RegisterPair::BC => {
                self.b = (value >> 8) as u8;
                self.c = value as u8;
            },
            RegisterPair::DE => {
                self.d = (value >> 8) as u8;
                self.e = value as u8;
            },
            RegisterPair::HL => {
                self.h = (value >> 8) as u8;
                self.l = value as u8;
            },
        }
    }

    ///Set flag value
    pub fn set_flag(&mut self, position: usize, value: bool) {
        if value {
            self.f |= 1 << position;
        } else {
            self.f &= !(1 << position);
        }
    }

    ///Get flag value
    pub fn get_flag(&self, position: usize) -> bool {
        (self.f & (1 << position)) != 0
    }

    ///Run program from memory
    pub fn run(mut self, mut memory: Memory, mut io: IoHandler, org: u16) {
        self.pc = org;

        loop {
            let opcode: u8 = memory.read(self.pc);
            let mut operand: Vec<u8> = Vec::new();
    
            let (mut instruction, address_mode) = &INSTRUCTIONS[opcode as usize];
            match *address_mode {
                AddressMode::Immediate => { 
                    operand.push(memory.read(self.pc + 1));
                },
                AddressMode::ImmediateExtended | AddressMode::Extended => {
                    operand.push(memory.read(self.pc + 1));
                    operand.push(memory.read(self.pc + 2));
                },
                _ => { }
            }

            if self.halted { instruction = Instruction::NOP };
            
            let cpu_cloned: Processor = self.clone();
            let result: String = process_instruction(&mut self, &mut memory, &mut io, instruction, operand).0;

            //Check for interrupt
            //if self.iff1 {}

            if !cpu_cloned.halted {
                println!("{color_cyan} {}", result);

                print!("{color_white} A: {:#04X} > {:#04X} |", cpu_cloned.a, self.a);
                println!("{color_white} F: {:#010b} > {:#010b}", cpu_cloned.f, self.f);

                print!("{color_white} B: {:#04X} > {:#04X} |", cpu_cloned.b, self.b);
                println!("{color_white} C: {:#04X} > {:#04X}", cpu_cloned.c, self.c);

                print!("{color_white} D: {:#04X} > {:#04X} |", cpu_cloned.d, self.d);
                println!("{color_white} E: {:#04X} > {:#04X}", cpu_cloned.e, self.e);

                print!("{color_white} H: {:#04X} > {:#04X} |", cpu_cloned.h, self.h);
                println!("{color_white} L: {:#04X} > {:#04X}", cpu_cloned.l, self.l);

                print!("{color_white} I: {:#04X} > {:#04X} |", cpu_cloned.i, self.i);
                println!("{color_white} R: {:#04X} > {:#04X}", cpu_cloned.r, self.r);

                println!("{color_white} IX: {:#06X} > {:#06X}", cpu_cloned.ix, self.ix);
                println!("{color_white} IY: {:#06X} > {:#06X}\n", cpu_cloned.iy, self.iy);

                println!("{color_white} SP: {:#06X} > {:#06X}", cpu_cloned.sp, self.sp);
                println!("{color_white} PC: {:#06X} > {:#06X}\n", cpu_cloned.pc, self.pc);

                if self.halted { println!("\n{color_red} CPU HALTED"); }

                println!("{color_reset}{style_reset}");
            }
        }
    }
}