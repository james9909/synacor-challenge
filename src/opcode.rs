use crate::error::VmError;

use std::convert::TryFrom;

pub enum Opcode {
    Halt,
    Out,
    NoOp,
}

impl TryFrom<u16> for Opcode {
    type Error = VmError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Halt),
            19 => Ok(Self::Out),
            21 => Ok(Self::NoOp),
            _ => Err(VmError::InvalidOpcode(value)),
        }
    }
}
