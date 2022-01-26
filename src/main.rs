pub mod stack;

use ethnum::u256;
use stack::Stack;

fn main() {

    let mut stack = Stack::default();
    let val = u256::new(1);

    stack.push(val);

    assert_eq!(stack.pop(), val);
}


