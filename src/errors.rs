use std::fmt;

#[derive(Debug)]
pub enum StackError {
    StackUnderFlow,
    StackOverFlow,
    DivisionByZero,
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StackError::StackUnderFlow => write!(f, "stack-underflow"),
            StackError::StackOverFlow => write!(f, "stack-overflow"),
            StackError::DivisionByZero => write!(f, "division-by-zero"),
        }
    }
}
