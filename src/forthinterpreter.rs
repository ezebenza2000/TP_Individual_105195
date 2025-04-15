use crate::commands::*;
use crate::file;
use crate::stackforth::StackForth;
use crate::traits::Executable;
use std::collections::HashMap;

/// Structures used
/// First hash ('fundamental_words') is used as not mutable hash.
/// Second has ('words') is used to define and redefine words.
///
/// When a new word is defined, its definition is stored in `words` as a sequence of fundamental operations.
/// This design allows words to be redefined freely, without affecting existing words that were previously
///   created using earlier definitions of those words.
/// If words_defined store other words_used it will change its meaning when words_used are redefine.
pub struct ForthInterpreter {
    pub fundamental_words: HashMap<String, Box<dyn Executable>>,
    pub words: HashMap<String, Vec<String>>,
    pub stack: StackForth,
}

impl ForthInterpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            fundamental_words: HashMap::new(),
            words: HashMap::new(),
            stack: StackForth::new(),
        };

        // Forth fundamental operations definitions (never change)
        interpreter
            .fundamental_words
            .insert("SUM".to_string(), Box::new(SumCommand));
        interpreter
            .fundamental_words
            .insert("SUBTRACT".to_string(), Box::new(SubtractCommand));
        interpreter
            .fundamental_words
            .insert("MULTIPLY".to_string(), Box::new(MultiplyCommand));
        interpreter
            .fundamental_words
            .insert("DIVIDE".to_string(), Box::new(DivideCommand));
        interpreter
            .fundamental_words
            .insert("CR".to_string(), Box::new(CrCommand));
        interpreter
            .fundamental_words
            .insert(".".to_string(), Box::new(DotCommand));
        interpreter
            .fundamental_words
            .insert("DUP".to_string(), Box::new(DupCommand));
        interpreter
            .fundamental_words
            .insert("DROP".to_string(), Box::new(DropCommand));
        interpreter
            .fundamental_words
            .insert("SWAP".to_string(), Box::new(SwapCommand));
        interpreter
            .fundamental_words
            .insert("OVER".to_string(), Box::new(OverCommand));
        interpreter
            .fundamental_words
            .insert("ROT".to_string(), Box::new(RotCommand));
        interpreter
            .fundamental_words
            .insert("EMIT".to_string(), Box::new(EmitCommand));
        interpreter
            .fundamental_words
            .insert("AND".to_string(), Box::new(AndCommand));
        interpreter
            .fundamental_words
            .insert("OR".to_string(), Box::new(OrCommand));
        interpreter
            .fundamental_words
            .insert("NOT".to_string(), Box::new(NotCommand));
        interpreter
            .fundamental_words
            .insert("=".to_string(), Box::new(EqualCommand));
        interpreter
            .fundamental_words
            .insert(">".to_string(), Box::new(GreaterCommand));
        interpreter
            .fundamental_words
            .insert("<".to_string(), Box::new(LessCommand));
        interpreter
            .fundamental_words
            .insert("IF".to_string(), Box::new(IfCommand));
        interpreter
            .fundamental_words
            .insert("ELSE".to_string(), Box::new(ElseCommand));
        interpreter
            .fundamental_words
            .insert("THEN".to_string(), Box::new(ThenCommand));

        // Forth words definitions (can change)
        interpreter
            .words
            .insert("+".to_string(), vec!["SUM".to_string()]);
        interpreter
            .words
            .insert("-".to_string(), vec!["SUBTRACT".to_string()]);
        interpreter
            .words
            .insert("*".to_string(), vec!["MULTIPLY".to_string()]);
        interpreter
            .words
            .insert("/".to_string(), vec!["DIVIDE".to_string()]);
        interpreter
            .words
            .insert("cr".to_string(), vec!["CR".to_string()]);
        interpreter
            .words
            .insert(".".to_string(), vec![".".to_string()]);
        interpreter
            .words
            .insert("dup".to_string(), vec!["DUP".to_string()]);
        interpreter
            .words
            .insert("drop".to_string(), vec!["DROP".to_string()]);
        interpreter
            .words
            .insert("swap".to_string(), vec!["SWAP".to_string()]);
        interpreter
            .words
            .insert("over".to_string(), vec!["OVER".to_string()]);
        interpreter
            .words
            .insert("rot".to_string(), vec!["ROT".to_string()]);
        interpreter
            .words
            .insert("emit".to_string(), vec!["EMIT".to_string()]);
        interpreter
            .words
            .insert("and".to_string(), vec!["AND".to_string()]);
        interpreter
            .words
            .insert("or".to_string(), vec!["OR".to_string()]);
        interpreter
            .words
            .insert("not".to_string(), vec!["NOT".to_string()]);
        interpreter
            .words
            .insert("if".to_string(), vec!["IF".to_string()]);
        interpreter
            .words
            .insert("else".to_string(), vec!["ELSE".to_string()]);
        interpreter
            .words
            .insert("then".to_string(), vec!["THEN".to_string()]);

        interpreter
    }

    /// Empties the stack by calling `StackForth` and writes the content to the specified file path.
    pub fn empty_forth(&mut self, file_name: &str) {
        if let Err(e) = file::write_to_file(file_name, &mut self.stack) {
            eprintln!("Error al escribir en el archivo: {}", e);
        }
    }

    pub fn is_a_word(&self, word: &str) -> bool {
        self.words.contains_key(word)
    }

    pub fn is_a_fundamental_word(&self, word: &str) -> bool {
        self.fundamental_words.contains_key(word)
    }

    ///PRE: Receive a vector of strings
    ///POST: Return a vector with each strings found in words hash value all together
    pub fn expand_sequence(&self, input_sequence: &Vec<String>) -> Option<Vec<String>> {
        let mut new_operation_sequence = Vec::new();

        for component in input_sequence {
            if let Some(sequence) = self.words.get(component) {
                for operation in sequence {
                    new_operation_sequence.push(operation.to_string());
                }
            } else {
                new_operation_sequence.push(component.to_string());
            }
        }

        Some(new_operation_sequence)
    }

    pub fn new_word(&mut self, word_name: &str, definition: &Vec<String>) {
        if let Some(sequence) = self.expand_sequence(definition) {
            self.words.insert(word_name.to_string(), sequence);
        }
    }

    pub fn execute_fundamental_word(&mut self, fundamental_word: &str) {
        if let Some(value) = self.fundamental_words.get(fundamental_word) {
            if let Err(e) = value.execute(&mut self.stack) {
                print!("{}", e);
            }
        } else {
            println!("Couldn't find word '{}'", fundamental_word);
        }
    }
}
