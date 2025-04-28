use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct DropCommand;

impl Executable for DropCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.drop()?;
        Ok(())
    }
}
