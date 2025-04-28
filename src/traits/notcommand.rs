use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct NotCommand;

impl Executable for NotCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.not()?;
        Ok(())
    }
}
