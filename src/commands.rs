use crate::errors::StackError;
use crate::stackforth::StackForth;
use crate::traits::Executable;

pub struct SumCommand;

impl Executable for SumCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.add()?;
        Ok(())
    }
}

pub struct SubtractCommand;

impl Executable for SubtractCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.subtract()?;
        Ok(())
    }
}

pub struct MultiplyCommand;

impl Executable for MultiplyCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.multiply()?;
        Ok(())
    }
}

pub struct DivideCommand;

impl Executable for DivideCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.divide()?;
        Ok(())
    }
}

pub struct CrCommand;

impl Executable for CrCommand {
    fn execute(&self, _stack: &mut StackForth) -> Result<(), StackError> {
        println!();
        Ok(())
    }
}

pub struct DotCommand;

impl Executable for DotCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        if let Ok(value) = stack.pop() {
            print!(" {}", value);
        } else {
            println!("Error: Couldn't print valid value");
        }
        Ok(())
    }
}

pub struct DupCommand;

impl Executable for DupCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.dup()?;
        Ok(())
    }
}

pub struct DropCommand;

impl Executable for DropCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.drop()?;
        Ok(())
    }
}

pub struct SwapCommand;

impl Executable for SwapCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.swap()?;
        Ok(())
    }
}

pub struct OverCommand;

impl Executable for OverCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.over()?;
        Ok(())
    }
}

pub struct RotCommand;

impl Executable for RotCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.rot()?;
        Ok(())
    }
}

pub struct EmitCommand;

impl Executable for EmitCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        if let Ok(value) = stack.pop() {
            print!(" {}", value as u8 as char);
        } else {
            println!("Error: Couldn't emit valid value");
        }
        Ok(())
    }
}

pub struct AndCommand;

impl Executable for AndCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.and()?;
        Ok(())
    }
}

pub struct OrCommand;

impl Executable for OrCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.or()?;
        Ok(())
    }
}

pub struct NotCommand;

impl Executable for NotCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.not()?;
        Ok(())
    }
}

pub struct EqualCommand;

impl Executable for EqualCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.equal()?;
        Ok(())
    }
}

pub struct GreaterCommand;

impl Executable for GreaterCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.greater_than()?;
        Ok(())
    }
}

pub struct LessCommand;

impl Executable for LessCommand {
    fn execute(&self, stack: &mut StackForth) -> Result<(), StackError> {
        stack.less_than()?;
        Ok(())
    }
}

pub struct IfCommand;

impl Executable for IfCommand {
    fn execute(&self, _stack: &mut StackForth) -> Result<(), StackError> {
        Ok(())
    }
}

pub struct ElseCommand;

impl Executable for ElseCommand {
    fn execute(&self, _stack: &mut StackForth) -> Result<(), StackError> {
        Ok(())
    }
}

pub struct ThenCommand;

impl Executable for ThenCommand {
    fn execute(&self, _stack: &mut StackForth) -> Result<(), StackError> {
        Ok(())
    }
}
