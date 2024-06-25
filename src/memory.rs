use lazy_static::lazy_static;

use std::sync::RwLock;
use std::fs;

#[derive(Clone, Copy)]
pub struct Memory {}

const MEMORY_SIZE: usize = 0x10000;
lazy_static! { static ref MEMORY: RwLock<[u8; MEMORY_SIZE]> = RwLock::new([0; MEMORY_SIZE]); }

impl Memory {
    pub fn new() -> Self {
        Memory {}
    }

    pub fn load_file(&mut self, file: &str, org: u16) {
        let program: Vec<u8> = fs::read(file)
            .expect("Not able to read the file.");

        for (i, byte) in program.iter().enumerate() {
            self.write((org as usize) + i, *byte);
        }
    }

    pub fn read(self, address: usize) -> u8 {
        if address < MEMORY_SIZE {
            let memory = MEMORY.read().unwrap();
            memory[address]
        } else {
            panic!("Address out of bounds.")
        }
    }

    pub fn write(self, address: usize, value: u8) {
        if address < MEMORY_SIZE {
            let mut memory = MEMORY.write().unwrap();
            memory[address] = value;
        } else {
            panic!("Address out of bounds.")
        }
    }
}