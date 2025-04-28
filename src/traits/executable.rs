use crate::errors::StackError;
use crate::stackforth::StackForth;

pub trait Executable {
    fn execute(&mut self, interpreter: &mut StackForth) -> Result<(), StackError>;
}
