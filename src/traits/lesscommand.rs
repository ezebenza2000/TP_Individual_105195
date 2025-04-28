use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct LessCommand;

impl Executable for LessCommand {
    fn execute(&mut self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.less_than()?;
        Ok(())
    }
}
