mod cpu;
mod instructions;

use std::env;
use std::fs;

use cpu::{CPU, AddressMode, run};
use instructions::{Instruction, match_instruction, process};

fn main() {
    let cpu: CPU = CPU { ..Default::default() };

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("File not specified.");
    }

    let ram: Vec<u8> = fs::read(&args[1])
        .expect("Not able to read file.");

    run(cpu, ram);
}