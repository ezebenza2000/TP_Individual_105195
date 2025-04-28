use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct SubtractCommand;

impl Executable for SubtractCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.subtract()?;
        Ok(())
    }
}
