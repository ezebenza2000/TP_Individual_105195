use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct DivideCommand;

impl Executable for DivideCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.divide()?;
        Ok(())
    }
}
