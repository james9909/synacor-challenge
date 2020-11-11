use crate::operand::Operand;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Halt,
    Out(Operand),
    NoOp,
}

impl Instruction {
    pub fn execute(&self, stack: &mut Vec<u16>, registers: &mut [u16; 8]) -> bool {
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
