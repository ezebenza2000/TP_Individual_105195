use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct EmitCommand {
    first_element: bool,
}

impl EmitCommand {
    pub fn new() -> Self {
        EmitCommand {
            first_element: true,
        }
    }
}

impl Executable for EmitCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        if let Ok(value) = stack.pop() {
            if self.first_element {
                print!("{}", value as u8 as char);
                self.first_element = false;
            } else {
                print!(" {}", value as u8 as char);
            }
        } else {
            println!("Error: Couldn't emit valid value");
        }
        Ok(())
    }
}
