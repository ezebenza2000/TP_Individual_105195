use crate::errors::StackError;

pub struct StackForth {
    stack: Vec<i16>,
    max_stack_size: usize,
}

impl Default for StackForth {
    fn default() -> Self {
        Self::new()
    }
}

impl StackForth {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            max_stack_size: 18,
        }
    }

    pub fn push(&mut self, value: i16) -> Result<i16, StackError> {
        if self.stack.len() >= self.max_stack_size {
            Err(StackError::StackOverFlow)
        } else {
            self.stack.push(value);
            Ok(value)
        }
    }

    pub fn pop(&mut self) -> Result<i16, StackError> {
        self.stack.pop().ok_or(StackError::StackUnderFlow)
    }

    pub fn add(&mut self) -> Result<(), StackError> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a + b)?;
        Ok(())
    }

    pub fn subtract(&mut self) -> Result<(), StackError> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a - b)?;
        Ok(())
    }

    pub fn multiply(&mut self) -> Result<(), StackError> {
        let b = self.pop()?;
        let a = self.pop()?;
        self.push(a * b)?;
        Ok(())
    }

    pub fn divide(&mut self) -> Result<(), StackError> {
        let b = self.pop()?;
        let a = self.pop()?;
        if b == 0 {
            return Err(StackError::DivisionByZero);
        }
        self.push(a / b)?;
        Ok(())
    }

    pub fn dup(&mut self) -> Result<(), StackError> {
        let top = *self.stack.last().ok_or(StackError::StackUnderFlow)?;
        self.push(top)?;
        Ok(())
    }

    pub fn drop(&mut self) -> Result<(), StackError> {
        self.pop()?;
        Ok(())
    }

    pub fn swap(&mut self) -> Result<(), StackError> {
        let first = self.pop()?;
        let second = self.pop()?;

        self.push(first)?;
        self.push(second)?;

        Ok(())
    }

    pub fn over(&mut self) -> Result<(), StackError> {
        let first = self.pop()?;
        let second = self.pop()?;

        self.push(second)?;
        self.push(first)?;

        self.push(second)?;

        Ok(())
    }

    pub fn rot(&mut self) -> Result<(), StackError> {
        let third = self.pop()?;
        let second = self.pop()?;
        let first = self.pop()?;

        self.push(second)?;
        self.push(third)?;
        self.push(first)?;

        Ok(())
    }

    pub fn equal(&mut self) -> Result<(), StackError> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(if a == b { -1 } else { 0 })?;
        Ok(())
    }

    pub fn less_than(&mut self) -> Result<(), StackError> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(if b < a { -1 } else { 0 })?;
        Ok(())
    }

    pub fn greater_than(&mut self) -> Result<(), StackError> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(if b > a { -1 } else { 0 })?;
        Ok(())
    }

    pub fn and(&mut self) -> Result<(), StackError> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(if a != 0 && b != 0 { -1 } else { 0 })?;
        Ok(())
    }

    pub fn or(&mut self) -> Result<(), StackError> {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(if a != 0 || b != 0 { -1 } else { 0 })?;
        Ok(())
    }

    pub fn not(&mut self) -> Result<(), StackError> {
        let a = self.pop()?;
        self.push(if a == 0 { -1 } else { 0 })?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::errors::StackError;

    #[test]
    fn test_push_pop() {
        let mut stack = StackForth::new();
        stack.push(42).unwrap();
        assert_eq!(stack.pop().unwrap(), 42);
    }

    #[test]
    fn test_add() {
        let mut stack = StackForth::new();
        stack.push(10).unwrap();
        stack.push(5).unwrap();
        stack.add().unwrap();
        assert_eq!(stack.pop().unwrap(), 15);
    }

    #[test]
    fn test_subtract() {
        let mut stack = StackForth::new();
        stack.push(10).unwrap();
        stack.push(5).unwrap();
        stack.subtract().unwrap();
        assert_eq!(stack.pop().unwrap(), 5);
    }

    #[test]
    fn test_multiply() {
        let mut stack = StackForth::new();
        stack.push(10).unwrap();
        stack.push(5).unwrap();
        stack.multiply().unwrap();
        assert_eq!(stack.pop().unwrap(), 50);
    }

    #[test]
    fn test_divide() {
        let mut stack = StackForth::new();
        stack.push(10).unwrap();
        stack.push(5).unwrap();
        stack.divide().unwrap();
        assert_eq!(stack.pop().unwrap(), 2);
    }

    #[test]
    fn test_divide_by_zero() {
        let mut stack = StackForth::new();
        stack.push(10).unwrap();
        stack.push(0).unwrap();
        let result = stack.divide();
        assert!(matches!(result, Err(StackError::DivisionByZero)));
    }

    #[test]
    fn test_dup() {
        let mut stack = StackForth::new();
        stack.push(42).unwrap();
        stack.dup().unwrap();
        assert_eq!(stack.pop().unwrap(), 42);
        assert_eq!(stack.pop().unwrap(), 42);
    }

    #[test]
    fn test_drop() {
        let mut stack = StackForth::new();
        stack.push(42).unwrap();
        stack.drop().unwrap();
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_swap() {
        let mut stack = StackForth::new();
        stack.push(10).unwrap();
        stack.push(20).unwrap();
        stack.swap().unwrap();
        assert_eq!(stack.pop().unwrap(), 10);
        assert_eq!(stack.pop().unwrap(), 20);
    }

    #[test]
    fn test_over() {
        let mut stack = StackForth::new();
        stack.push(10).unwrap();
        stack.push(20).unwrap();
        stack.over().unwrap();
        assert_eq!(stack.pop().unwrap(), 10);
        assert_eq!(stack.pop().unwrap(), 20);
        assert_eq!(stack.pop().unwrap(), 10);
    }

    #[test]
    fn test_rot() {
        let mut stack = StackForth::new();
        stack.push(10).unwrap();
        stack.push(20).unwrap();
        stack.push(30).unwrap();
        stack.rot().unwrap();
        assert_eq!(stack.pop().unwrap(), 10);
        assert_eq!(stack.pop().unwrap(), 30);
        assert_eq!(stack.pop().unwrap(), 20);
    }

    #[test]
    fn test_equal() {
        let mut stack = StackForth::new();
        stack.push(10).unwrap();
        stack.push(10).unwrap();
        stack.equal().unwrap();
        assert_eq!(stack.pop().unwrap(), -1);
    }

    #[test]
    fn test_not_equal() {
        let mut stack = StackForth::new();
        stack.push(10).unwrap();
        stack.push(5).unwrap();
        stack.equal().unwrap();
        assert_eq!(stack.pop().unwrap(), 0);
    }

    #[test]
    fn test_less_than() {
        let mut stack = StackForth::new();
        stack.push(5).unwrap();
        stack.push(10).unwrap();
        stack.less_than().unwrap();
        assert_eq!(stack.pop().unwrap(), -1);
    }

    #[test]
    fn test_not_less_than() {
        let mut stack = StackForth::new();
        stack.push(10).unwrap();
        stack.push(5).unwrap();
        stack.less_than().unwrap();
        assert_eq!(stack.pop().unwrap(), 0);
    }

    #[test]
    fn test_greater_than() {
        let mut stack = StackForth::new();
        stack.push(10).unwrap();
        stack.push(5).unwrap();
        stack.greater_than().unwrap();
        assert_eq!(stack.pop().unwrap(), -1);
    }

    #[test]
    fn test_not_greater_than() {
        let mut stack = StackForth::new();
        stack.push(5).unwrap();
        stack.push(10).unwrap();
        stack.greater_than().unwrap();
        assert_eq!(stack.pop().unwrap(), 0);
    }

    #[test]
    fn test_and() {
        let mut stack = StackForth::new();
        stack.push(1).unwrap();
        stack.push(1).unwrap();
        stack.and().unwrap();
        assert_eq!(stack.pop().unwrap(), -1);

        stack.push(1).unwrap();
        stack.push(0).unwrap();
        stack.and().unwrap();
        assert_eq!(stack.pop().unwrap(), 0);

        stack.push(0).unwrap();
        stack.push(0).unwrap();
        stack.and().unwrap();
        assert_eq!(stack.pop().unwrap(), 0);
    }

    #[test]
    fn test_or() {
        let mut stack = StackForth::new();
        stack.push(1).unwrap();
        stack.push(1).unwrap();
        stack.or().unwrap();
        assert_eq!(stack.pop().unwrap(), -1);

        stack.push(1).unwrap();
        stack.push(0).unwrap();
        stack.or().unwrap();
        assert_eq!(stack.pop().unwrap(), -1);

        stack.push(0).unwrap();
        stack.push(0).unwrap();
        stack.or().unwrap();
        assert_eq!(stack.pop().unwrap(), 0);
    }

    #[test]
    fn test_not() {
        let mut stack = StackForth::new();
        stack.push(1).unwrap();
        stack.not().unwrap();
        assert_eq!(stack.pop().unwrap(), 0);

        stack.push(0).unwrap();
        stack.not().unwrap();
        assert_eq!(stack.pop().unwrap(), -1);
    }
}
