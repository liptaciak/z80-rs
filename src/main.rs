mod cpu;
mod instructions;

use std::fs;

use cpu::{CPU, RegisterPair, AddressMode};
use instructions::{match_instruction, process};

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("z80")
        .about("Z80 Emulator.")
        .arg(
            Arg::new("file")
                .help("The file to process.")
                .required(true)
                .index(1),
        )
        .get_matches();

    let cpu: CPU = Default::default();

    let file = matches.get_one::<String>("file").unwrap();
    let program: Vec<u8> = fs::read(file)
        .expect("Not able to read file.");
    
    let mut ram: Vec<u8> = Vec::with_capacity(0x10000);
    for opcode in program.iter().copied() {
        if ram.len() < 0x10000 {
            ram.push(opcode);
        }
    }

    cpu.run(ram);
}