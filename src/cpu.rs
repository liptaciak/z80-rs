use crate::{AddressMode, Instruction, match_instruction, process};

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

pub fn run(mut cpu: CPU, ram: Vec<u8>) {
    #[allow(unused_assignments)]
    let mut operand: Option<u8> = None;

    for (index, i) in ram.iter().enumerate() {
        let x: usize = cpu.pc as usize;

        if index != x { continue; }
        println!("PC: {}", x);

        let (instruction, address_mode): (Instruction, AddressMode) = match_instruction(i);
        match address_mode {
            AddressMode::Immediate => { 
                operand = Some(ram[x + 1]);
                cpu.pc += 1;
            },
            _ => { }
        }

        process(&mut cpu, instruction, operand);
        cpu.pc += 1;
    }
}