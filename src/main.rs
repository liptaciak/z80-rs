use std::fs;
use clap::{Arg, Command};

use zin::cpu::Processor;

fn main() {
    let matches = Command::new("zin")
        .about("A z80 emulator.")
        .arg(
            Arg::new("file")
                .help("The file to process.")
                .required(true)
                .index(1),
        )
        .get_matches();

    let cpu: Processor = Default::default();
    
    //Get the file to process
    let file = matches.get_one::<String>("file").unwrap();
    let program: Vec<u8> = fs::read(file)
        .expect("Not able to read file.");
    
    //Load the program into memory
    let mut ram: Vec<u8> = Vec::with_capacity(0x10000);
    for opcode in program.iter().copied() {
        if ram.len() < 0x10000 {
            ram.push(opcode);
        }
    }
    
    unsafe { ram.set_len(0x10000); }

    cpu.run(ram);
}