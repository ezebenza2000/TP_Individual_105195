use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct IfCommand;

impl Executable for IfCommand {
    fn execute(&mut self, _stack: &mut StackForth) -> Result<(), StackError> {
        Ok(())
    }
}
