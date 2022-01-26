pub mod stack;
pub mod memory;

use ethnum::u256;
use stack::Stack;
use memory::Memory;

fn main() {

    let mut stack = Stack::default();
    let mut memory = Memory::default();

    let val = u256::new(1);
    let offset = 5;

    stack.push(val);
    memory.store(offset, val);

    assert_eq!(stack.pop(), val);
    assert_eq!(memory.load(offset), val);
}


