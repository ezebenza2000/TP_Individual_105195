pub struct StackForth {
    stack: Vec<i16>,
}

impl Default for StackForth {
    fn default() -> Self {
        Self::new()
    }
}

impl StackForth {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, value: i16) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<i16> {
        self.stack.pop()
    }

    pub fn add(&mut self) {
        if let (Some(b), Some(a)) = (self.pop(), self.pop()) {
            self.push(a + b);
        } else {
            eprintln!("Error: Not enough values in stack");
        }
    }

    pub fn subtract(&mut self) {
        if let (Some(b), Some(a)) = (self.pop(), self.pop()) {
            self.push(a - b);
        } else {
            eprintln!("Error: Not enough values in stack");
        }
    }

    pub fn multiply(&mut self) {
        if let (Some(b), Some(a)) = (self.pop(), self.pop()) {
            self.push(a * b);
        } else {
            eprintln!("Error: Not enough values in stack");
        }
    }

    pub fn divide(&mut self) {
        if let (Some(b), Some(a)) = (self.pop(), self.pop()) {
            if b == 0 {
                println!("Error: Divide by CERO");
                self.push(a);
                self.push(b);
            } else {
                self.push(a / b);
            }
        } else {
            eprintln!("Error: Not enough values in stack");
        }
    }

    pub fn dup(&mut self) {
        if let Some(&top) = self.stack.last() {
            self.stack.push(top);
        } else {
            eprintln!("Error: No values in stack for DUP");
        }
    }

    pub fn drop_top(&mut self) {
        if self.stack.pop().is_none() {
            println!("Error: Not enough values in stack for DROP");
        }
    }

    pub fn swap(&mut self) {
        if self.stack.len() < 2 {
            eprintln!("Error: Not enough values in stack for SWAP");
            return;
        }
        let len = self.stack.len();
        self.stack.swap(len - 1, len - 2);
    }

    pub fn over(&mut self) {
        if self.stack.len() < 2 {
            eprintln!("Error: Not enough values in stack for OVER");
            return;
        }
        let len = self.stack.len();
        let second_last = self.stack[len - 2];
        self.stack.push(second_last);
    }

    pub fn rot(&mut self) {
        if self.stack.len() < 3 {
            eprintln!("Error: Not enough values in stack for ROT");
            return;
        }
        let len = self.stack.len();
        self.stack.swap(len - 1, len - 2);
        self.stack.swap(len - 2, len - 3);
    }

    pub fn equal(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            self.push(if a == b { 1 } else { 0 });
        }
    }

    //second pop in stack is less than first in stack
    pub fn less_than(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            self.push(if b < a { 1 } else { 0 });
        }
    }

    //second pop in stack is garter than first in stack
    pub fn greater_than(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            self.push(if b > a { 1 } else { 0 });
        }
    }

    pub fn and(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            self.push(if a != 0 && b != 0 { 1 } else { 0 });
        }
    }

    pub fn or(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            self.push(if a != 0 || b != 0 { 1 } else { 0 });
        }
    }

    pub fn not(&mut self) {
        if let Some(a) = self.pop() {
            self.push(if a == 0 { 1 } else { 0 });
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut stack = StackForth::new();
        stack.push(42);
        assert_eq!(stack.pop(), Some(42));
    }

    #[test]
    fn test_add() {
        let mut stack = StackForth::new();
        stack.push(10);
        stack.push(5);
        stack.add();
        assert_eq!(stack.pop(), Some(15));
    }

    #[test]
    fn test_subtract() {
        let mut stack = StackForth::new();
        stack.push(10);
        stack.push(5);
        stack.subtract();
        assert_eq!(stack.pop(), Some(5));
    }

    #[test]
    fn test_multiply() {
        let mut stack = StackForth::new();
        stack.push(10);
        stack.push(5);
        stack.multiply();
        assert_eq!(stack.pop(), Some(50));
    }

    #[test]
    fn test_divide() {
        let mut stack = StackForth::new();
        stack.push(10);
        stack.push(5);
        stack.divide();
        assert_eq!(stack.pop(), Some(2));
    }

    #[test]
    fn test_divide_by_zero() {
        let mut stack = StackForth::new();
        stack.push(10);
        stack.push(0);
        stack.divide();
        assert_eq!(stack.pop(), Some(0));
    }

    #[test]
    fn test_dup() {
        let mut stack = StackForth::new();
        stack.push(42);
        stack.dup();
        assert_eq!(stack.pop(), Some(42));
        assert_eq!(stack.pop(), Some(42));
    }

    #[test]
    fn test_drop() {
        let mut stack = StackForth::new();
        stack.push(42);
        stack.drop_top();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_swap() {
        let mut stack = StackForth::new();
        stack.push(10);
        stack.push(20);
        stack.swap();
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), Some(20));
    }

    #[test]
    fn test_over() {
        let mut stack = StackForth::new();
        stack.push(10);
        stack.push(20);
        stack.over();
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
    }

    #[test]
    fn test_rot() {
        let mut stack = StackForth::new();
        stack.push(10);
        stack.push(20);
        stack.push(30);
        stack.rot();
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), Some(30));
    }

    #[test]
    fn test_equal() {
        let mut stack = StackForth::new();
        stack.push(10);
        stack.push(10);
        stack.equal();
        assert_eq!(stack.pop(), Some(1)); // 1 means true
    }

    #[test]
    fn test_not_equal() {
        let mut stack = StackForth::new();
        stack.push(10);
        stack.push(5);
        stack.equal();
        assert_eq!(stack.pop(), Some(0)); // 0 means false
    }

    #[test]
    fn test_less_than() {
        let mut stack = StackForth::new();
        stack.push(5);
        stack.push(10);
        stack.less_than();
        assert_eq!(stack.pop(), Some(1)); // 1 means true
    }

    #[test]
    fn test_not_less_than() {
        let mut stack = StackForth::new();
        stack.push(10);
        stack.push(5);
        stack.less_than();
        assert_eq!(stack.pop(), Some(0)); //0 means false
    }

    #[test]
    fn test_greater_than() {
        let mut stack = StackForth::new();
        stack.push(10);
        stack.push(5);
        stack.greater_than();
        assert_eq!(stack.pop(), Some(1)); // 1 means true
    }

    #[test]
    fn test_not_greater_than() {
        let mut stack = StackForth::new();
        stack.push(5);
        stack.push(10);
        stack.greater_than();
        assert_eq!(stack.pop(), Some(0)); // 0 significa falso
    }

    #[test]
    fn test_and() {
        let mut stack = StackForth::new();
        stack.push(1); // verdadero
        stack.push(1); // verdadero
        stack.and();
        assert_eq!(stack.pop(), Some(1));

        stack.push(1); // ture
        stack.push(0); // false
        stack.and();
        assert_eq!(stack.pop(), Some(0));

        stack.push(0); // false
        stack.push(0); // false
        stack.and();
        assert_eq!(stack.pop(), Some(0));
    }

    #[test]
    fn test_or() {
        let mut stack = StackForth::new();
        stack.push(1); // verdadero
        stack.push(1); // falso
        stack.or();
        assert_eq!(stack.pop(), Some(1));

        stack.push(1); // verdadero
        stack.push(0); // falso
        stack.or();
        assert_eq!(stack.pop(), Some(1));

        stack.push(0); // falso
        stack.push(0); // falso
        stack.or();
        assert_eq!(stack.pop(), Some(0));
    }

    #[test]
    fn test_not() {
        let mut stack = StackForth::new();
        stack.push(1);
        stack.not();
        assert_eq!(stack.pop(), Some(0));

        stack.push(0);
        stack.not();
        assert_eq!(stack.pop(), Some(1));
    }
}
