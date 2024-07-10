use clap::{Arg, Command};

use zin::cpu::processor::Processor;
use zin::io::handler::IoHandler;
use zin::memory::Memory;

fn main() {
    let matches = Command::new("zin")
        .about("A z80 emulator.")
        .arg(
            Arg::new("file")
                .help("The file to process.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("start")
                .help("The start address of the program.")
                .required(false)
                .index(2),
        )
        .get_matches();

    let cpu: Processor = Default::default();

    //Get the file to process and load it to memory
    let file = matches.get_one::<String>("file").unwrap();
    let start_arg = matches.get_one::<String>("start");
    
    let mut start: u16 = 0x0000;
    if !start_arg.is_none() {
        if start_arg.unwrap().starts_with("0x") {
            start = u16::from_str_radix(&start_arg.unwrap()[2..], 16).unwrap();
        } else {
            start = start_arg.unwrap().parse::<u16>().unwrap();
        }
    }

    let mut memory: Memory = Memory::new();
    memory.load_file(file.as_str(), start);

    let io: IoHandler = IoHandler::new();
    
    //Run the program
    cpu.run(memory, io, start);
}