use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct DotCommand {
    first_element: bool,
}

impl DotCommand {
    pub fn new() -> Self {
        DotCommand {
            first_element: true,
        }
    }
}

impl Executable for DotCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        if let Ok(value) = stack.pop() {
            if self.first_element {
                print!("{}", value);
                self.first_element = false;
            } else {
                print!(" {}", value);
            }
        } else {
            println!("Error: Couldn't print valid value");
        }
        Ok(())
    }
}
