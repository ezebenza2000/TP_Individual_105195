use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct OrCommand;

impl Executable for OrCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.or()?;
        Ok(())
    }
}
