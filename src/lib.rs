pub mod cpu;
pub mod instructions;

//Tests
#[cfg(test)]
mod tests {
    use crate::cpu::*;
    use crate::instructions::*;

    #[test]
    fn ldhln() {
        let mut cpu: Processor = Default::default();
        cpu.set_pair(RegisterPair::HL, 0x0000);

        let mut ram: Vec<u8> = Vec::with_capacity(0x10000);
        ram.push(0x36);

        unsafe { ram.set_len(0x10000); }

        let ram = process_instruction(&mut cpu, &mut ram, Instruction::LDHLN, vec![0x12]).2;
        assert_eq!(ram[cpu.get_pair(RegisterPair::HL) as usize], 0x12);
    }
}