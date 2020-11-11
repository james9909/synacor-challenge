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
    Gt(Register, Operand, Operand),
    Jmp(Operand),
    Jt(Operand, Operand),
    Jf(Operand, Operand),
    Add(Register, Operand, Operand),
    Mult(Register, Operand, Operand),
    Mod(Register, Operand, Operand),
    And(Register, Operand, Operand),
    Or(Register, Operand, Operand),
    Not(Register, Operand),
    Call(Operand),
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
            Self::Gt(dest, a, b) => {
                let a = state.read_value(a);
                let b = state.read_value(b);
                state.set_register(*dest, (a > b) as u16);
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
                let sum = (state.read_value(a) + state.read_value(b)) % MODULO;
                state.set_register(*dest, sum);
            }
            Self::Mult(dest, a, b) => {
                let sum =
                    (state.read_value(a) as usize * state.read_value(b) as usize) % MODULO as usize;
                state.set_register(*dest, sum as Literal);
            }
            Self::Mod(dest, a, b) => {
                let a = state.read_value(a);
                let b = state.read_value(b);
                state.set_register(*dest, a % b);
            }
            Self::And(dest, a, b) => {
                let and = state.read_value(a) & state.read_value(b);
                state.set_register(*dest, and);
            }
            Self::Or(dest, a, b) => {
                let or = state.read_value(a) | state.read_value(b);
                state.set_register(*dest, or);
            }
            Self::Not(dest, a) => {
                let a = state.read_value(a);
                // Only flip the last 15 bits using the mask 0b111111111111111
                state.set_register(*dest, a ^ 0x7fff);
            }
            Self::Call(a) => {
                state.push(state.pc);
                state.pc = state.read_value(a);
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
