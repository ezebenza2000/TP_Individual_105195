use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::executable::Executable;

pub struct CrCommand;

impl Executable for CrCommand {
    fn execute(&mut self, _stack: &mut StackForth) -> Result<(), StackError> {
        println!();
        Ok(())
    }
}
