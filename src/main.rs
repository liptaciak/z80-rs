mod instructions;
mod cpu;

use cpu::CPU;
use instructions::{Instructions, process};

fn main() {
    #[allow(unused)]
    let mut cpu: CPU = CPU { c: 5, ..Default::default() };

    let instruction: Instructions = Instructions::Ldbc; //Moves 5 which is in C register to B register which is 0

    process(cpu, instruction);
}