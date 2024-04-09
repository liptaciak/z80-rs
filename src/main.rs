mod cpu;
mod instructions;
mod interp;

use std::env;
use std::fs;

use cpu::{CPU, run};
use instructions::{AddressMode, Instruction, process};
use interp::match_instruction;

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