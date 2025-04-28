use crate::traits::andcommand::AndCommand;
use crate::traits::crcommand::CrCommand;
use crate::traits::dividecommand::DivideCommand;
use crate::traits::dotcommand::DotCommand;
use crate::traits::dropcommand::DropCommand;
use crate::traits::dupcommand::DupCommand;
use crate::traits::elsecommand::ElseCommand;
use crate::traits::emitcommand::EmitCommand;
use crate::traits::equalcommand::EqualCommand;
use crate::traits::executable::Executable;
use crate::traits::greatercommand::GreaterCommand;
use crate::traits::ifcommand::IfCommand;
use crate::traits::lesscommand::LessCommand;
use crate::traits::multiplycommand::MultiplyCommand;
use crate::traits::notcommand::NotCommand;
use crate::traits::orcommand::OrCommand;
use crate::traits::overcommand::OverCommand;
use crate::traits::rotcommand::RotCommand;
use crate::traits::subtractcommand::SubtractCommand;
use crate::traits::sumcommand::SumCommand;
use crate::traits::swapcommand::SwapCommand;
use crate::traits::thencommand::ThenCommand;

use crate::file;
use crate::stackforth::StackForth;
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
    pub words_definition: HashMap<String, Vec<String>>,
    pub stack: StackForth,
    number_definitions: usize,
}

impl ForthInterpreter {
    pub fn new(stack_size: usize) -> Self {
        let mut interpreter = Self {
            fundamental_words: HashMap::new(),
            words: HashMap::new(),
            words_definition: HashMap::new(),
            stack: StackForth::new(stack_size),
            number_definitions: 0,
        };
        interpreter.register_fundamental_words();
        interpreter.register_words();

        interpreter
    }

    fn register_fundamental_words(&mut self) {
        // Forth fundamental operations definitions (never change)
        self.fundamental_words
            .insert("SUM".to_string(), Box::new(SumCommand));
        self.fundamental_words
            .insert("SUBTRACT".to_string(), Box::new(SubtractCommand));
        self.fundamental_words
            .insert("MULTIPLY".to_string(), Box::new(MultiplyCommand));
        self.fundamental_words
            .insert("DIVIDE".to_string(), Box::new(DivideCommand));
        self.fundamental_words
            .insert("CR".to_string(), Box::new(CrCommand));
        self.fundamental_words.insert(
            ".".to_string(),
            Box::new(DotCommand::new()) as Box<dyn Executable>,
        );
        self.fundamental_words
            .insert("DUP".to_string(), Box::new(DupCommand));
        self.fundamental_words
            .insert("DROP".to_string(), Box::new(DropCommand));
        self.fundamental_words
            .insert("SWAP".to_string(), Box::new(SwapCommand));
        self.fundamental_words
            .insert("OVER".to_string(), Box::new(OverCommand));
        self.fundamental_words
            .insert("ROT".to_string(), Box::new(RotCommand));
        self.fundamental_words.insert(
            "EMIT".to_string(),
            Box::new(EmitCommand::new()) as Box<dyn Executable>,
        );
        self.fundamental_words
            .insert("AND".to_string(), Box::new(AndCommand));
        self.fundamental_words
            .insert("OR".to_string(), Box::new(OrCommand));
        self.fundamental_words
            .insert("NOT".to_string(), Box::new(NotCommand));
        self.fundamental_words
            .insert("=".to_string(), Box::new(EqualCommand));
        self.fundamental_words
            .insert(">".to_string(), Box::new(GreaterCommand));
        self.fundamental_words
            .insert("<".to_string(), Box::new(LessCommand));
        self.fundamental_words
            .insert("if".to_string(), Box::new(IfCommand));
        self.fundamental_words
            .insert("else".to_string(), Box::new(ElseCommand));
        self.fundamental_words
            .insert("then".to_string(), Box::new(ThenCommand));
    }
    fn register_words(&mut self) {
        // Forth words definitions (can change)
        self.words.insert("+".to_string(), vec!["SUM".to_string()]);
        self.words
            .insert("-".to_string(), vec!["SUBTRACT".to_string()]);
        self.words
            .insert("*".to_string(), vec!["MULTIPLY".to_string()]);
        self.words
            .insert("/".to_string(), vec!["DIVIDE".to_string()]);
        self.words.insert("cr".to_string(), vec!["CR".to_string()]);
        self.words.insert(".".to_string(), vec![".".to_string()]);
        self.words
            .insert("dup".to_string(), vec!["DUP".to_string()]);
        self.words
            .insert("drop".to_string(), vec!["DROP".to_string()]);
        self.words
            .insert("swap".to_string(), vec!["SWAP".to_string()]);
        self.words
            .insert("over".to_string(), vec!["OVER".to_string()]);
        self.words
            .insert("rot".to_string(), vec!["ROT".to_string()]);
        self.words
            .insert("emit".to_string(), vec!["EMIT".to_string()]);
        self.words
            .insert("and".to_string(), vec!["AND".to_string()]);
        self.words.insert("or".to_string(), vec!["OR".to_string()]);
        self.words
            .insert("not".to_string(), vec!["NOT".to_string()]);
        self.words.insert("if".to_string(), vec!["if".to_string()]);
        self.words
            .insert("else".to_string(), vec!["else".to_string()]);
        self.words
            .insert("then".to_string(), vec!["then".to_string()]);
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
    pub fn is_a_word_definition(&self, word: &str) -> bool {
        self.words_definition.contains_key(word)
    }

    pub fn is_a_fundamental_word(&self, word: &str) -> bool {
        self.fundamental_words.contains_key(word)
    }

    pub fn expand_sequence_word_definition(&self, word: &str) -> Option<Vec<String>> {
        let mut new_operation_sequence = Vec::new();
        if let Some(sequence) = self.words_definition.get(word) {
            for operation in sequence {
                new_operation_sequence.push(operation.to_string());
            }
        }
        Some(new_operation_sequence)
    }

    pub fn expand_sequence_word(&self, word: &str) -> Option<Vec<String>> {
        let mut new_operation_sequence = Vec::new();
        if let Some(sequence) = self.words.get(word) {
            for operation in sequence {
                new_operation_sequence.push(operation.to_string());
            }
        }
        Some(new_operation_sequence)
    }

    fn restructure_definitions(&mut self, word_name: &str) {
        if let Some(word_value) = self.words.get(word_name) {
            for component in word_value {
                for definition in self.words_definition.values_mut() {
                    for item in definition.iter_mut() {
                        if *item == word_name {
                            *item = component.to_string();
                        }
                    }
                }
            }
        }
    }

    pub fn new_word(&mut self, word_name: &str, definition: &Vec<String>) {
        let mut new_sequence = Vec::new();

        if self.is_a_word(word_name) {
            self.restructure_definitions(word_name);
        }

        for component in definition {
            if component == word_name {
                if let Some(old_definition) = self.expand_sequence_word(word_name) {
                    new_sequence.extend(old_definition);
                }
            } else if self.is_a_word(component) {
                if let Some(expanded) = self.expand_sequence_word(component) {
                    new_sequence.extend(expanded);
                } else {
                    new_sequence.push(component.to_string());
                }
            } else {
                new_sequence.push(component.to_string());
            }
        }

        let def_name = format!("defname{}", self.number_definitions);
        self.words_definition
            .insert(def_name.to_string(), new_sequence);
        self.words
            .insert(word_name.to_string(), vec![def_name.to_string()]);
        self.number_definitions += 1;
    }

    pub fn execute_fundamental_word(&mut self, fundamental_word: &str) {
        if let Some(value) = self.fundamental_words.get_mut(fundamental_word) {
            if let Err(e) = value.execute(&mut self.stack) {
                print!("{}", e);
            }
        } else {
            println!("Couldn't find word '{}'", fundamental_word);
        }
    }
}
