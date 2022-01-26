mod execution_context;

use ethnum::u256;
use execution_context::stack::Stack;
use execution_context::memory::Memory;

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


