use crate::operand::{Literal, Operand};
use crate::vm::VmState;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Halt,
    Out(Operand),
    NoOp,
    Jmp(Literal),
}

impl Instruction {
    pub fn execute(&self, state: &mut VmState) -> bool {
        match self {
            Self::Halt => false,
            Self::Out(value) => {
                let chr = match value {
                    Operand::Literal(c) => *c,
                    Operand::Register(r) => state.registers[*r as usize],
                } as u8;
                print!("{}", chr as char);
                true
            }
            Self::Jmp(destination) => {
                // Multiply by 2 because values are 16 bits
                state.pc = *destination * 2;
                true
            }
            Self::NoOp => true,
        }
    }
}
