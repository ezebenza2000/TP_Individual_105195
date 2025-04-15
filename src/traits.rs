use crate::errors::StackError;
use crate::stackforth::StackForth;

pub trait Executable {
    fn execute(&self, interpreter: &mut StackForth) -> Result<(), StackError>;
}
