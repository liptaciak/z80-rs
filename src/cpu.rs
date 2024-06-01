use crate::{match_instruction, process};

//Macro for making all struct fields public.
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Default, Debug, Clone, PartialEq)]
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
    BC,
    DE,
    HL,
}

#[allow(dead_code)]
pub enum AddressMode {
    None,
    Implied,
    Register,
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
            let counter: usize = self.pc as usize;
            println!("PC: {}", counter);

            let i: u8 = ram[counter];
            let mut operand: Vec<u8> = Vec::new();
    
            let (instruction, address_mode) = match_instruction(i);
            match address_mode {
                AddressMode::Immediate => { 
                    operand.push(ram[counter + 1]);
                    self.pc += 1;
                },
                AddressMode::ImmediateExtended => {
                    operand.push(ram[counter + 1]);
                    operand.push(ram[counter + 2]);
                    self.pc += 2;
                },
                _ => { }
            }
    
            process(&mut self, instruction, operand);

            if counter > u16::MAX as usize { self.pc = 0x0000; }
        }
    }
}