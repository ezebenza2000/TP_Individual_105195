use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct ThenCommand;

impl Executable for ThenCommand {
    fn execute(&mut self, _stack: &mut StackForth) -> Result<(), StackError> {
        Ok(())
    }
}
