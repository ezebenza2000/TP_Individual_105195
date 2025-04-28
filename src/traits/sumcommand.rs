use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct SumCommand;

impl Executable for SumCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.add()?;
        Ok(())
    }
}
