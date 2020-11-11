use crate::error::VmError;
use crate::operand::{Literal, Operand, Register};
use crate::vm::VmState;

const MODULO: Literal = 32768;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Halt,
    Set(Register, Operand),
    Push(Operand),
    Pop(Register),
    Eq(Register, Operand, Operand),
    Jmp(Operand),
    Jt(Operand, Operand),
    Jf(Operand, Operand),
    Add(Register, Operand, Operand),
    Out(Operand),
    NoOp,
}

impl Instruction {
    pub fn execute(&self, state: &mut VmState) -> Result<bool, VmError> {
        match self {
            Self::Halt => return Ok(false),
            Self::Set(dest, source) => {
                state.set_register(*dest, state.read_value(source));
            }
            Self::Push(a) => {
                state.push(state.read_value(a));
            }
            Self::Pop(a) => {
                let v = state.pop()?;
                state.set_register(*a, v);
            }
            Self::Eq(dest, a, b) => {
                let a = state.read_value(a);
                let b = state.read_value(b);
                state.set_register(*dest, (a == b) as u16);
            }
            Self::Jmp(dest) => {
                state.pc = state.read_value(dest);
            }
            Self::Jt(a, b) => {
                let a = state.read_value(a);
                let b = state.read_value(b);
                if a != 0u16 {
                    state.pc = b;
                }
            }
            Self::Jf(a, b) => {
                let a = state.read_value(a);
                let b = state.read_value(b);
                if a == 0u16 {
                    state.pc = b;
                }
            }
            Self::Add(dest, a, b) => {
                let sum = state.read_value(a) + state.read_value(b) % MODULO;
                state.set_register(*dest, sum);
            }
            Self::Out(op) => {
                let chr = state.read_value(op) as u8;
                print!("{}", chr as char);
            }
            Self::NoOp => {}
        };
        Ok(true)
    }
}
