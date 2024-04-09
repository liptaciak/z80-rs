use crate::CPU;

#[allow(dead_code)]
pub enum AddressMode {
    Implied,
    Register,
    Immediate,
}

#[allow(dead_code)]
pub enum Instruction {
    Nop,
    Ldbb,
    Ldbc,
    Ldan,
    Ldbn,
    Ldcn,
    Lddn,
    Lden,
    Ldhn,
    Ldln,
}

pub fn process(cpu: &mut CPU, instruction: Instruction, operand: Option<u8>) {
    match instruction {
        Instruction::Nop => {
            println!("NOP 00\n");
        },
        Instruction::Ldbb => {
            cpu.b = cpu.b;

            println!("LDBB 28 | B: {0} -> B: {1}\n", cpu.b, cpu.b);
        },
        Instruction::Ldbc => {
            cpu.b = cpu.c;
        
            println!("LDBC 29 | C: {0} -> B: {1}\n", cpu.c, cpu.b);
        },
        Instruction::Ldan => {
            cpu.a = operand.unwrap();

            println!("LDAN 3E | N: {0} -> A: {1}\n", operand.unwrap(), cpu.a);
        },
        Instruction::Ldbn => {
            cpu.b = operand.unwrap();

            println!("LDBN 06 | N: {0} -> B: {1}\n", operand.unwrap(), cpu.b);
        },
        Instruction::Ldcn => {
            cpu.c = operand.unwrap();

            println!{"LDCN OE | N: {0} -> C: {1}\n", operand.unwrap(), cpu.c};
        },
        _ => panic!("Instruction not implemented."),
    }
}