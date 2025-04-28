mod commandinterpreter;
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
    let mut stack_size: usize = 128000;
    let mut output_file = "stack.fth";

    if args.len() > 2 {
        if let Ok(value) = parse::parse_stack_len(&args[2]) {
            stack_size = value;
        } else {
            eprintln!("Failed to parse stack size");
        }
    }

    //This helps for temporal stacks in tests.
    if args.len() > 3 {
        output_file = &args[3];
    }

    let mut interpreter = ForthInterpreter::new(stack_size);

    match file::read_file(input_file) {
        Ok(lines) => {
            let parsed = parse::parse(lines);
            /*            for parse in parsed{
                println!("parse:'{}´",parse);
            } */

            commandinterpreter::interpret_commands(&mut interpreter, parsed);
            interpreter.empty_forth(output_file);
        }
        Err(e) => eprintln!("Error while opening file: {}", e),
    }
}
