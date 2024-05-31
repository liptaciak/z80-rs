use crate::{Instruction, match_instruction, process};

//Macro for making all struct fields public.
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub_struct!(CPU {
    a: u8, f: u8,
    b: u8, c: u8,
    d: u8, e: u8,
    h: u8, l: u8,
    i: u8, r: u8,
    ix: u16, 
    iy: u16,
    sp: u16,
    pc: u16,
});

#[allow(dead_code)]
pub enum RegisterPair {
    AF,
    BC,
    DE,
    HL,
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            a: 0, f: 0,
            b: 0, c: 0,
            d: 0, e: 0,
            h: 0, l: 0,
            i: 0, r: 0,
            ix: 0,
            iy: 0,
            sp: 0,
            pc: 0,
        }
    }
}

#[allow(dead_code)]
pub enum AddressMode {
    Implied,
    Register,
    Immediate,
    ImmediateExtended,
}

impl CPU {
    pub fn get_pair(&self, pair: RegisterPair) -> u16 {
        match pair {
            RegisterPair::AF => ((self.a as u16) << 8) | (self.f as u16),
            RegisterPair::BC => ((self.b as u16) << 8) | (self.c as u16),
            RegisterPair::DE => ((self.d as u16) << 8) | (self.e as u16),
            RegisterPair::HL => ((self.h as u16) << 8) | (self.l as u16),
        }
    }

    pub fn set_pair(&mut self, pair: RegisterPair, value: u16) {
        match pair {
            RegisterPair::AF => {
                self.a = (value >> 8) as u8;
                self.f = value as u8;
            },
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
}

pub fn run(mut cpu: CPU, ram: Vec<u8>) {
    for (index, i) in ram.iter().enumerate() {
        let mut operand: Vec<u8> = Vec::new();
        let x: usize = cpu.pc as usize;

        if index != x { continue; }
        println!("PC: {}", x);

        let (instruction, address_mode): (Instruction, AddressMode) = match_instruction(i);
        match address_mode {
            AddressMode::Immediate => { 
                operand.push(ram[x + 1]);
                cpu.pc += 1;
            },
            AddressMode::ImmediateExtended => {
                operand.push(ram[x + 1]);
                operand.push(ram[x + 2]);
                cpu.pc += 2;
            },
            _ => { }
        }

        process(&mut cpu, instruction, operand);
        cpu.pc += 1;
    }
}