use std::env;

use synacor::vm::Vm;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file> ", &args[0]);
        return Ok(());
    }

    let filename = &args[1];
    let mut code = synacor::read_program(filename)?;
    let mut vm = Vm::new(&mut code);

    if let Err(e) = vm.run() {
        println!("Program crashed: {}", e);
    }
    Ok(())
}
