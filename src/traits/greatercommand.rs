use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct GreaterCommand;

impl Executable for GreaterCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.greater_than()?;
        Ok(())
    }
}
