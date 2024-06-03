use crate::{match_instruction, process};
#[derive(Default)]
pub struct CPU {
    pub a: u8, pub f: u8,
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    pub i: u8, pub r: u8,
    pub ix: u16, 
    pub iy: u16,
    pub sp: u16,
    pub pc: u16,
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

impl CPU {
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

    pub fn run(mut self, ram: Vec<u8>) {
        loop {
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
    
            process(&mut self, instruction, operand);

            if self.pc > u16::MAX { self.pc = 0x0000; }
        }
    }
}