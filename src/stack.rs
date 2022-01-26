use ethnum::u256;
pub struct Stack {
    stack: Vec<u256>,
    max_depth: u16
}

impl Stack {
    pub fn push(&mut self, value: u256) {
        if self.stack.len() + 1 > self.max_depth as usize {
            panic!("Stack overflow");
        }

        self.stack.push(value);
    }

    pub fn pop(&mut self) -> u256 {
        if self.stack.len() == 0 {
            panic!("Stack underflow");
        }

        self.stack.pop().unwrap()
    }
}

impl Default for Stack {
    fn default() -> Self {
        Stack {
            stack: vec![],
            max_depth: 1024
        }
    }
}