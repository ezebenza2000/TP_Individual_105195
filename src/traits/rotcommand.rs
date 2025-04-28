use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct RotCommand;

impl Executable for RotCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.rot()?;
        Ok(())
    }
}
