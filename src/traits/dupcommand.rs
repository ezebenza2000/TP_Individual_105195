use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct DupCommand;

impl Executable for DupCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.dup()?;
        Ok(())
    }
}
