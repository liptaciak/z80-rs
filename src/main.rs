mod cpu;
mod instructions;

use std::env;
use std::fs;

use cpu::{CPU, RegisterPair, AddressMode};
use instructions::{match_instruction, process};

fn main() {
    let cpu: CPU = Default::default();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("File not specified.");
    }

    let program: Vec<u8> = fs::read(&args[1])
        .expect("Not able to read file.");

    let mut ram: Vec<u8> = Vec::with_capacity(0x10000);
    for opcode in program.iter().copied() {
        if ram.len() < 0x10000 {
            ram.push(opcode);
        }
    }

    cpu.run(ram);
}