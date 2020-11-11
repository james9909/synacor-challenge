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
    let mut vm = vm::Vm::new(&buffer);
    vm.run();
    Ok(())
}
