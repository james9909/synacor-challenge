use crate::operand::Operand;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum VmError {
    #[error("invalid opcode 0x{0:02x}")]
    InvalidOpcode(u16),
    #[error("invalid operand {0}")]
    InvalidOperand(u16),
    #[error("unexpected operand {0:?}")]
    UnexpectedOperand(Operand),
}
