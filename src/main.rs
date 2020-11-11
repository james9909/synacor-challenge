use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

mod error;
mod instruction;
mod opcode;
mod operand;
mod vm;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file> ", &args[0]);
        return Ok(());
    }

    let filename = &args[1];
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // Convert the program so we're operating only on u16 values
    let mut code: Vec<u16> = buffer
        .chunks_exact(2)
        .into_iter()
        .map(|b| u16::from_ne_bytes([b[0], b[1]]))
        .collect();
    let mut vm = vm::Vm::new(&mut code);

    if let Err(e) = vm.run() {
        println!("Program crashed: {}", e);
    }
    Ok(())
}
