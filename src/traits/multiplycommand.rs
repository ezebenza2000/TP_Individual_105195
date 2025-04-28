use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct MultiplyCommand;

impl Executable for MultiplyCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.multiply()?;
        Ok(())
    }
}
