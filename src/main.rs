mod file;
mod forthinterpreter;
mod parse;
mod stackforth;

use forthinterpreter::ForthInterpreter;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No file as argument");
        return;
    }

    let resultado = file::read_file(&args[1]);

    match resultado {
        Ok(lines) => {
            let mut forth_interpreter = ForthInterpreter::new();
            for line in lines {
                let parsed = parse::parse(line);
                forth_interpreter.execute(parsed);
            }
            forth_interpreter.empty_forth("stack.fth");
        }
        Err(e) => eprintln!("Error while opening file: {}", e),
    }
}
