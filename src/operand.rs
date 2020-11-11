use crate::error::VmError;

use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub enum Operand {
    Literal(u16),
    Register(u8),
}

impl TryFrom<u16> for Operand {
    type Error = VmError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0..=32767 => Ok(Operand::Literal(value)),
            32768..=32775 => Ok(Operand::Register((value - 32768) as u8)),
            32776..=65535 => Err(VmError::InvalidOperand(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u16() {
        // Literals
        assert_eq!(Operand::try_from(0).unwrap(), Operand::Literal(0));
        assert_eq!(Operand::try_from(32767).unwrap(), Operand::Literal(32767));

        // Registers
        assert_eq!(Operand::try_from(32768).unwrap(), Operand::Register(0));
        assert_eq!(Operand::try_from(32775).unwrap(), Operand::Register(7));

        // Invalid
        assert!(Operand::try_from(32776).is_err());
        assert!(Operand::try_from(65535).is_err());
    }
}
