use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct ElseCommand;

impl Executable for ElseCommand {
    fn execute(&mut self, _stack: &mut StackForth) -> Result<(), StackError> {
        Ok(())
    }
}
