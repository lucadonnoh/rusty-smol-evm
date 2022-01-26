pub mod memory;
pub mod stack;

use memory::Memory;
use stack::Stack;

pub struct ExecutionContext {
    code: Vec<u8>,
    stack: Stack,
    memory: Memory,
    pc: usize,
    stopped: bool
}

impl ExecutionContext {
    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn read_code(&mut self, num_bytes: usize) -> u128 {
        let code = u128::from_be_bytes(self.code[self.pc..self.pc + num_bytes].try_into().unwrap());
        self.pc += num_bytes;
        code
    }
}

impl Default for ExecutionContext {
    fn default() -> Self {
        ExecutionContext {
            code: Vec::new(),
            stack: Stack::default(),
            memory: Memory::default(),
            pc: 0,
            stopped: false
        }
    }
}