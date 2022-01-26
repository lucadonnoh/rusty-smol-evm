use ethnum::u256;

pub struct Memory {
    memory: Vec<u256>
}

impl Memory {
    pub fn store(&mut self, offset: usize, value: u256) {
        if offset >= self.memory.len() {
            self.memory.resize(offset + 1, u256::default());
        }

        self.memory[offset] = value;
    }

    pub fn load(&self, offset: usize) -> u256 {
        if offset > self.memory.len() {
            u256::default()
        } else {
            self.memory[offset]
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            memory: vec![]
        }
    }
}