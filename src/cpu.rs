use crate::instructions::{match_instruction, process_instruction};
use inline_colorization::*;

#[derive(Default, Clone)]
pub struct Processor {
    pub a: u8, pub f: u8,
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    pub i: u8, pub r: u8,
    pub ix: u16, 
    pub iy: u16,
    pub sp: u16,
    pub pc: u16,
    pub halted: bool,
}

pub enum RegisterPair {
    BC, DE, HL,
}

pub enum AddressMode {
    None,
    Register,
    Extended,
    Immediate,
    ImmediateExtended,
}

impl Processor {
    pub fn get_pair(&self, pair: RegisterPair) -> u16 {
        match pair {
            RegisterPair::BC => ((self.b as u16) << 8) | (self.c as u16),
            RegisterPair::DE => ((self.d as u16) << 8) | (self.e as u16),
            RegisterPair::HL => ((self.h as u16) << 8) | (self.l as u16),
        }
    }

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

    pub fn set_flag(&mut self, position: usize, value: bool) {
        if value {
            self.f |= 1 << position;
        } else {
            self.f &= !(1 << position);
        }
    }

    pub fn get_flag(&self, position: usize) -> bool {
        (self.f & (1 << position)) != 0
    }

    pub fn run(mut self, mut ram: Vec<u8>) {
        loop {
            if self.halted { continue; }

            let instr: u8 = ram[self.pc as usize];
            let mut operand: Vec<u8> = Vec::new();
    
            let (instruction, address_mode) = match_instruction(instr);
            match address_mode {
                AddressMode::Immediate => { 
                    operand.push(ram[self.pc as usize + 1]);
                },
                AddressMode::ImmediateExtended | AddressMode::Extended => {
                    operand.push(ram[self.pc as usize + 1]);
                    operand.push(ram[self.pc as usize + 2]);
                },
                _ => { }
            }
            
            let cpu_cloned: Processor = self.clone();
            let result: String = process_instruction(&mut self, &mut ram, instruction, operand).0;

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
            println!("{color_white} PC: {:#06X} > {:#06X}", cpu_cloned.pc, self.pc);

            println!("{color_reset}{style_reset}\n");
        }
    }
}