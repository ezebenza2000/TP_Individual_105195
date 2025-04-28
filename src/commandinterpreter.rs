use crate::forthinterpreter::ForthInterpreter;
use crate::parse::parse_conditional_blocks;

pub struct CommandLine {
    iter: std::vec::IntoIter<String>,
}

impl CommandLine {
    pub fn new(commands: Vec<String>) -> Self {
        Self {
            iter: commands.into_iter(),
        }
    }

    pub fn next_token(&mut self) -> Option<String> {
        self.iter.next()
    }

    //PRE: Receive a string
    //POST: Iter until string is found
    pub fn skip_until(&mut self, end_string: &str) {
        while let Some(token) = self.next_token() {
            if token == end_string {
                break;
            }
        }
    }

    //PRE: Receive an end_string
    //POST: Return a vector with every string of the iteration until end_string is found
    pub fn collect_until(&mut self, end_string: &str) -> Vec<String> {
        let mut result = Vec::new();
        while let Some(token) = self.next_token() {
            if token == end_string {
                break;
            }
            result.push(token);
        }
        result
    }
}

//Runs the line's stream
pub fn interpret_commands(interpreter: &mut ForthInterpreter, commands: Vec<String>) {
    let mut stream = CommandLine::new(commands);
    while let Some(token) = stream.next_token() {
        //println!("token: {}",token);
        match_token(interpreter, token, &mut stream);
    }
}

fn match_token(interpreter: &mut ForthInterpreter, token: String, stream: &mut CommandLine) {
    match token.as_str() {
        ":" => word_definition(interpreter, stream),
        ".\"" => print_command(stream),
        _ => execute_token(interpreter, token, stream),
    }
}

fn word_definition(interpreter: &mut ForthInterpreter, stream: &mut CommandLine) {
    if let Some(word_name) = stream.next_token() {
        if word_name.parse::<i16>().is_ok() {
            println!("invalid-word");
            stream.skip_until(";");
            return;
        }

        let definition = stream.collect_until(";");
        interpreter.new_word(&word_name, &definition);
    } else {
        println!("invalid-word");
    }
}

fn execute_token(interpreter: &mut ForthInterpreter, token: String, stream: &mut CommandLine) {
    if interpreter.is_a_fundamental_word(&token) {
        handle_fundamental_word(interpreter, token, stream);
    } else if interpreter.is_a_word_definition(&token){

    } else if interpreter.is_a_word(&token) {
        handle_word(interpreter, token);
    } else if let Ok(num) = token.parse::<i16>() {
        if let Err(e) = interpreter.stack.push(num) {
            println!("{}", e);
        }
    } else {
        println!("?");
    }
}

fn handle_fundamental_word(
    interpreter: &mut ForthInterpreter,
    token: String,
    stream: &mut CommandLine,
) {
    match token.as_str() {
        "IF" => {
            let (if_block, else_block) = parse_conditional_blocks(&mut stream.iter);
            // If we are here it means that we encountered a conditional evaluation.
            // So we have a tuple of (if_bock , else_block) so it depends of the condition
            // wich one will be executed.
            match interpreter.stack.pop() {
                Ok(condition) => {
                    if condition != 0 {
                        interpret_commands(interpreter, if_block);
                    } else if let Some(else_block) = else_block {
                        interpret_commands(interpreter, else_block);
                    }
                }
                Err(e) => print!("{}", e),
            }
        }
        _ => {
            interpreter.execute_fundamental_word(&token);
        }
    }
}

fn handle_word(interpreter: &mut ForthInterpreter, token: String) {
    let word = vec![token.to_string()];
    if let Some(sequence) = interpreter.expand_sequence(&word) {
        interpret_commands(interpreter, sequence);
    }
}

fn print_command(stream: &mut CommandLine) {
    let output = stream.next_token();

    if let Some(value) = output {
        print!("{}", value);
    }
}
