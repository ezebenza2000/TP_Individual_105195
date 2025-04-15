mod commandinterpreter;
mod commands;
mod errors;
mod file;
mod forthinterpreter;
mod parse;
mod stackforth;
mod traits;

use crate::forthinterpreter::ForthInterpreter;
use std::env;

/// FUNCTION: main()
/// Reads the first command-line argument as the input file path.
/// Parses the contents of the file into individual lines.
/// For each parsed line, invokes `ForthInterpreter` to execute the corresponding command line.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No file as argument");
        return;
    }

    let input_file = &args[1];
    let output_file = if args.len() >= 3 {
        &args[2]
    } else {
        "stack.fth"
    };

    let mut interpreter = ForthInterpreter::new();

    match file::read_file(input_file) {
        Ok(lines) => {
            for line in lines {
                let parsed = parse::parse(line);
                commandinterpreter::interpret_commands(&mut interpreter, parsed);
            }
            interpreter.empty_forth(output_file);
        }
        Err(e) => eprintln!("Error while opening file: {}", e),
    }
}
