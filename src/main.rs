use clap::{Arg, Command};

use zin::cpu::processor::Processor;
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
            Arg::new("org")
                .help("The origin address.")
                .required(false)
                .index(2),
        )
        .get_matches();

    let cpu: Processor = Default::default();

    //Get the file to process and load it to memory
    let file = matches.get_one::<String>("file").unwrap();
    let org_arg = matches.get_one::<String>("org");
    
    let mut org: u16 = 0x0000;
    if !org_arg.is_none() {
        if org_arg.unwrap().starts_with("0x") {
            org = u16::from_str_radix(&org_arg.unwrap()[2..], 16).unwrap();
        } else {
            org = org_arg.unwrap().parse::<u16>().unwrap();
        }
    }

    let mut memory: Memory = Memory::new();
    memory.load_file(file.as_str(), org);
    
    //Run the program
    cpu.run(memory, org);
}