use crate::{file, stackforth::StackForth};
use std::collections::HashMap;

pub struct ForthInterpreter {
    words: HashMap<String, Vec<String>>,
    stack: StackForth,
}

impl Default for ForthInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl ForthInterpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            words: HashMap::new(),
            stack: StackForth::new(),
        };

        interpreter
            .words
            .insert("+".to_string(), vec!["+".to_string()]);
        interpreter
            .words
            .insert("-".to_string(), vec!["-".to_string()]);
        interpreter
            .words
            .insert("*".to_string(), vec!["*".to_string()]);
        interpreter
            .words
            .insert("/".to_string(), vec!["/".to_string()]);
        interpreter
            .words
            .insert("CR".to_string(), vec!["CR".to_string()]);
        interpreter
            .words
            .insert(".".to_string(), vec![".".to_string()]);
        interpreter
            .words
            .insert("DUP".to_string(), vec!["DUP".to_string()]);
        interpreter
            .words
            .insert("DROP".to_string(), vec!["DROP".to_string()]);
        interpreter
            .words
            .insert("SWAP".to_string(), vec!["SWAP".to_string()]);
        interpreter
            .words
            .insert("OVER".to_string(), vec!["OVER".to_string()]);
        interpreter
            .words
            .insert("ROT".to_string(), vec!["ROT".to_string()]);
        interpreter
            .words
            .insert("EMIT".to_string(), vec!["EMIT".to_string()]);
        interpreter
            .words
            .insert("AND".to_string(), vec!["AND".to_string()]);
        interpreter
            .words
            .insert("OR".to_string(), vec!["OR".to_string()]);
        interpreter
            .words
            .insert("NOT".to_string(), vec!["NOT".to_string()]);
        interpreter
            .words
            .insert("=".to_string(), vec!["=".to_string()]);
        interpreter
            .words
            .insert(">".to_string(), vec![">".to_string()]);
        interpreter
            .words
            .insert("<".to_string(), vec!["<".to_string()]);

        interpreter
    }

    pub fn empty_forth(&mut self, file_name: &str) {
        while let Some(value) = self.stack.pop() {
            if let Err(e) = file::write_to_file(file_name, &value.to_string()) {
                eprintln!("Error al escribir en el archivo: {}", e);
            }
        }
    }

    pub fn execute(&mut self, line: Vec<String>) {
        let mut iter = line.into_iter();
        let mut executing = true;
        let mut skip_else = false;

        while let Some(token) = iter.next() {
            match token.as_str() {
                ":" => {
                    if let Some(word_name) = iter.next() {
                        let mut definition = Vec::new();
                        for next_token in iter.by_ref() {
                            if next_token == ";" {
                                break;
                            } else {
                                definition.push(next_token);
                            }
                        }
                        self.words.insert(word_name, definition);
                    }
                }
                "IF" => {
                    if let Some(value) = self.stack.pop() {
                        executing = value != 0; // Si es 0, salteamos hasta ELSE o THEN
                        skip_else = false;
                    } else {
                        println!("Error: IF sin valor en el stack");
                        executing = false;
                    }
                }
                "ELSE" => {
                    if !skip_else {
                        executing = !executing; // Cambia la ejecución entre la rama verdadera y falsa
                    }
                }
                "THEN" => {
                    executing = true; // Restablece la ejecución normal
                }
                _ => {
                    if executing {
                        self.execute_word(&token);
                    }
                }
            }
        }
    }

    fn execute_word(&mut self, token: &str) {
        if let Some(key) = self.words.get(token) {
            let mut executing = true;
            for component in key {
                match component.as_str() {
                    "IF" => {
                        if let Some(value) = self.stack.pop() {
                            executing = value != 0;
                        } else {
                            println!("Error: IF sin valor en el stack");
                            executing = false;
                        }
                    }
                    "ELSE" => {
                        executing = !executing;
                    }
                    "THEN" => {
                        executing = true;
                    }
                    _ if executing => match component.to_uppercase().as_str() {
                        "+" => self.stack.add(),
                        "-" => self.stack.subtract(),
                        "*" => self.stack.multiply(),
                        "/" => self.stack.divide(),
                        "CR" => println!(),
                        "." => {
                            if let Some(value) = self.stack.pop() {
                                println!("{}", value);
                            } else {
                                println!("Error: Couldnt print valid value");
                            }
                        }
                        "DUP" => self.stack.dup(),
                        "DROP" => self.stack.drop_top(),
                        "SWAP" => self.stack.swap(),
                        "OVER" => self.stack.over(),
                        "ROT" => self.stack.rot(),
                        "EMIT" => {
                            if let Some(value) = self.stack.pop() {
                                print!("{}", value as u8 as char);
                            } else {
                                println!("Error: Could not emit character");
                            }
                        }
                        "AND" => self.stack.and(),
                        "OR" => self.stack.or(),
                        "NOT" => self.stack.not(),
                        "=" => self.stack.equal(),
                        ">" => self.stack.greater_than(),
                        "<" => self.stack.less_than(),
                        _ => {
                            if let Ok(num) = component.parse::<i16>() {
                                self.stack.push(num);
                            } else {
                                eprintln!("Error: '{}' is not a valid number", token);
                            }
                        }
                    },
                    _ => {}
                }
            }
        } else if let Ok(num) = token.parse::<i16>() {
            self.stack.push(num);
        } else {
            eprintln!("Error: '{}' is not a valid number", token);
        }
    }
}
