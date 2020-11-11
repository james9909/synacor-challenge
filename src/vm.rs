use crate::error::VmError;
use crate::instruction::Instruction;
use crate::opcode::Opcode;
use crate::operand::{Literal, Operand};

use std::convert::TryFrom;

const NUM_REGISTERS: usize = 8;

pub struct VmState {
    pub(crate) pc: u16,
    pub(crate) registers: [u16; NUM_REGISTERS],
    pub(crate) stack: Vec<u16>,
}

pub struct Vm<'a> {
    code: &'a [u8],
    running: bool,
    state: VmState,
}

impl<'a> Vm<'a> {
    pub fn new(code: &'a [u8]) -> Vm<'a> {
        Vm {
            code,
            running: true,
            state: VmState {
                pc: 0,
                registers: [0; 8],
                stack: Vec::new(),
            },
        }
    }

    pub fn step(&mut self) {
        if !self.running {
            return;
        }
        match self.read_instruction() {
            Ok(i) => {
                self.running = i.execute(&mut self.state);
            }
            Err(e) => {
                println!("Failed to read instruction: {}", e);
                self.running = false;
            }
        };
        if self.state.pc as usize >= self.code.len() {
            self.running = false;
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.step()
        }
    }

    fn read_u8(&mut self) -> u8 {
        let val = self.code[self.state.pc as usize];
        self.state.pc += 1;
        val
    }

    fn read_u16(&mut self) -> u16 {
        (self.read_u8() as u16) | ((self.read_u8() as u16) << 8)
    }

    fn parse_operand(&mut self) -> Result<Operand, VmError> {
        Operand::try_from(self.read_u16())
    }

    fn expect_literal(&mut self) -> Result<Literal, VmError> {
        match self.parse_operand()? {
            Operand::Literal(l) => Ok(l),
            op @ _ => Err(VmError::UnexpectedOperand(op)),
        }
    }

    fn read_instruction(&mut self) -> Result<Instruction, VmError> {
        let instruction = match Opcode::try_from(self.read_u16())? {
            Opcode::Halt => Instruction::Halt,
            Opcode::Jmp => Instruction::Jmp(self.expect_literal()?),
            Opcode::Out => Instruction::Out(self.parse_operand()?),
            Opcode::NoOp => Instruction::NoOp,
        };
        Ok(instruction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_halt() {
        let program = [0, 0];
        let mut vm = Vm::new(&program);
        vm.step();
        assert!(!vm.running);
    }
}
