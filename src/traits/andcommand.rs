use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct AndCommand;

impl Executable for AndCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.and()?;
        Ok(())
    }
}
