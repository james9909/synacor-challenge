use crate::error::VmError;
use crate::instruction::Instruction;
use crate::opcode::Opcode;
use crate::operand::{Literal, Operand, Register};

use std::convert::TryFrom;

const NUM_REGISTERS: usize = 8;

struct StdinReader {
    buf: String,
    pos: usize,
}

impl StdinReader {
    pub fn new() -> Self {
        Self {
            buf: String::new(),
            pos: 0,
        }
    }

    pub fn read_char(&mut self) -> Result<u8, std::io::Error> {
        if self.pos == self.buf.len() {
            self.buf.clear();
            std::io::stdin().read_line(&mut self.buf)?;
            self.pos = 0;
        }
        let chr = self.buf.as_bytes()[self.pos];
        self.pos += 1;
        Ok(chr)
    }
}

pub struct VmState {
    memory: Vec<u16>,
    input_buffer: StdinReader,
    pc: u16,
    registers: [u16; NUM_REGISTERS],
    stack: Vec<u16>,
}

impl VmState {
    fn read_u16(&mut self) -> u16 {
        let val = self.memory[self.pc as usize];
        self.pc += 1;
        val
    }

    pub fn read(&self, address: u16) -> u16 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }

    pub fn read_value(&self, operand: &Operand) -> Literal {
        match operand {
            Operand::Literal(l) => *l,
            Operand::Register(r) => self.registers[*r as usize],
        }
    }

    pub fn set_register(&mut self, register: Register, value: Literal) {
        self.registers[register as usize] = value
    }

    pub fn get_register(&self, register: Register) -> Literal {
        self.registers[register as usize]
    }

    pub fn push(&mut self, value: Literal) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Literal, VmError> {
        self.stack.pop().ok_or(VmError::StackUnderflow)
    }

    pub fn read_char(&mut self) -> Result<u8, std::io::Error> {
        self.input_buffer.read_char()
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, new_pc: u16) {
        self.pc = new_pc;
    }
}

pub struct Vm {
    running: bool,
    state: VmState,
}

impl Vm {
    pub fn new(code: &[u16]) -> Vm {
        let mut memory = Vec::from(code);
        memory.resize(2usize.pow(15), 0);
        Vm {
            running: true,
            state: VmState {
                memory,
                input_buffer: StdinReader::new(),
                pc: 0,
                registers: [0; 8],
                stack: Vec::new(),
            },
        }
    }

    pub fn step(&mut self) -> Result<(), VmError> {
        if !self.running {
            return Ok(());
        }
        match self.read_instruction() {
            Ok(i) => {
                self.running = i.execute(&mut self.state)?;
            }
            Err(e) => {
                println!("Failed to read instruction: {}", e);
                self.running = false;
            }
        };
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        while self.running {
            self.step()?
        }
        Ok(())
    }

    fn parse_operand(&mut self) -> Result<Operand, VmError> {
        Operand::try_from(self.state.read_u16())
    }

    fn expect_register(&mut self) -> Result<Register, VmError> {
        match self.parse_operand()? {
            Operand::Register(r) => Ok(r),
            op @ _ => Err(VmError::UnexpectedOperand(op)),
        }
    }

    fn read_instruction(&mut self) -> Result<Instruction, VmError> {
        let instruction = match Opcode::try_from(self.state.read_u16())? {
            Opcode::Halt => Instruction::Halt,
            Opcode::Set => Instruction::Set(self.expect_register()?, self.parse_operand()?),
            Opcode::Push => Instruction::Push(self.parse_operand()?),
            Opcode::Pop => Instruction::Pop(self.expect_register()?),
            Opcode::Eq => Instruction::Eq(
                self.expect_register()?,
                self.parse_operand()?,
                self.parse_operand()?,
            ),
            Opcode::Gt => Instruction::Gt(
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
            Opcode::Mult => Instruction::Mult(
                self.expect_register()?,
                self.parse_operand()?,
                self.parse_operand()?,
            ),
            Opcode::Mod => Instruction::Mod(
                self.expect_register()?,
                self.parse_operand()?,
                self.parse_operand()?,
            ),
            Opcode::And => Instruction::And(
                self.expect_register()?,
                self.parse_operand()?,
                self.parse_operand()?,
            ),
            Opcode::Or => Instruction::Or(
                self.expect_register()?,
                self.parse_operand()?,
                self.parse_operand()?,
            ),
            Opcode::Not => Instruction::Not(self.expect_register()?, self.parse_operand()?),
            Opcode::Rmem => Instruction::Rmem(self.parse_operand()?, self.parse_operand()?),
            Opcode::Wmem => Instruction::Wmem(self.parse_operand()?, self.parse_operand()?),
            Opcode::Call => Instruction::Call(self.parse_operand()?),
            Opcode::Ret => Instruction::Ret,
            Opcode::Out => Instruction::Out(self.parse_operand()?),
            Opcode::In => Instruction::In(self.parse_operand()?),
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
        vm.step().expect("failed to step through program");
        assert!(!vm.running);
    }
}
