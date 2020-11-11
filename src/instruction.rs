use crate::operand::Operand;
use crate::vm::VmState;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Halt,
    Out(Operand),
    NoOp,
}

impl Instruction {
    pub fn execute(&self, state: &mut VmState) -> bool {
        match self {
            Self::Halt => false,
            Self::Out(Operand::Literal(c)) => {
                print!("{}", *c as u8 as char);
                true
            }
            Self::NoOp => true,
            _ => false,
        }
    }
}
