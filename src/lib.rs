pub mod error;
pub mod instruction;
pub mod opcode;
pub mod operand;
pub mod vm;

use std::fs::File;
use std::io::Read;

pub fn read_program(filename: &str) -> Result<Vec<u16>, std::io::Error> {
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // Convert the input so we're operating only on u16 values
    Ok(buffer
        .chunks_exact(2)
        .into_iter()
        .map(|b| u16::from_ne_bytes([b[0], b[1]]))
        .collect())
}
