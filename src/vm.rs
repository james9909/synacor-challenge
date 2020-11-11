use crate::error::VmError;
use crate::instruction::Instruction;
use crate::opcode::Opcode;
use crate::operand::{Literal, Operand, Register};

use std::convert::TryFrom;

const NUM_REGISTERS: usize = 8;

pub struct VmState {
    pub(crate) pc: u16,
    pub(crate) registers: [u16; NUM_REGISTERS],
    pub(crate) stack: Vec<u16>,
}

impl VmState {
    pub fn read_value(&self, operand: &Operand) -> Literal {
        match operand {
            Operand::Literal(l) => *l,
            Operand::Register(r) => self.registers[*r as usize],
        }
    }

    pub fn set_register(&mut self, register: Register, value: Literal) {
        self.registers[register as usize] = value
    }
}

pub struct Vm<'a> {
    code: &'a [u16],
    running: bool,
    state: VmState,
}

impl<'a> Vm<'a> {
    pub fn new(code: &'a [u16]) -> Vm<'a> {
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

    fn read_u16(&mut self) -> u16 {
        let val = self.code[self.state.pc as usize];
        self.state.pc += 1;
        val
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

    fn expect_register(&mut self) -> Result<Register, VmError> {
        match self.parse_operand()? {
            Operand::Register(r) => Ok(r),
            op @ _ => Err(VmError::UnexpectedOperand(op)),
        }
    }

    fn read_instruction(&mut self) -> Result<Instruction, VmError> {
        let instruction = match Opcode::try_from(self.read_u16())? {
            Opcode::Halt => Instruction::Halt,
            Opcode::Set => Instruction::Set(self.expect_register()?, self.parse_operand()?),
            Opcode::Eq => Instruction::Eq(
                self.expect_register()?,
                self.parse_operand()?,
                self.parse_operand()?,
            ),
            Opcode::Jmp => Instruction::Jmp(self.parse_operand()?),
            Opcode::Jt => Instruction::Jt(self.parse_operand()?, self.parse_operand()?),
            Opcode::Jf => Instruction::Jf(self.parse_operand()?, self.parse_operand()?),
            Opcode::Add => Instruction::Add(
                self.expect_register()?,
                self.parse_operand()?,
                self.parse_operand()?,
            ),
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
        let program = [0];
        let mut vm = Vm::new(&program);
        vm.step();
        assert!(!vm.running);
    }
}
