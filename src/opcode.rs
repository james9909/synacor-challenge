use crate::error::VmError;

use std::convert::TryFrom;

pub enum Opcode {
    Halt,
    Set,
    Push,
    Pop,
    Eq,
    Gt,
    Jmp,
    Jt,
    Jf,
    Add,
    Mult,
    Mod,
    And,
    Or,
    Not,
    Rmem,
    Wmem,
    Call,
    Out,
    NoOp,
}

impl TryFrom<u16> for Opcode {
    type Error = VmError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Halt),
            1 => Ok(Self::Set),
            2 => Ok(Self::Push),
            3 => Ok(Self::Pop),
            4 => Ok(Self::Eq),
            5 => Ok(Self::Gt),
            6 => Ok(Self::Jmp),
            7 => Ok(Self::Jt),
            8 => Ok(Self::Jf),
            9 => Ok(Self::Add),
            10 => Ok(Self::Mult),
            11 => Ok(Self::Mod),
            12 => Ok(Self::And),
            13 => Ok(Self::Or),
            14 => Ok(Self::Not),
            15 => Ok(Self::Rmem),
            16 => Ok(Self::Wmem),
            17 => Ok(Self::Call),
            19 => Ok(Self::Out),
            21 => Ok(Self::NoOp),
            _ => Err(VmError::InvalidOpcode(value)),
        }
    }
}
