use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct SwapCommand;

impl Executable for SwapCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.swap()?;
        Ok(())
    }
}
