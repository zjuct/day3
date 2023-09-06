use std::cell::RefCell;

#[derive(Debug)]
struct SimpleStack<T> {
  stack: RefCell<Vec<T>>
}

impl<T> SimpleStack<T> {
  fn new() -> Self {
    Self {
      stack: RefCell::new(Vec::new())
    }
  }

  fn push(&self, value: T) {
    self.stack.borrow_mut().push(value);
  }

  fn pop(&self) -> Option<T> {
    self.stack.borrow_mut().pop()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn stack_test() {
    let stack = SimpleStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));

    stack.push(4);

    assert_eq!(stack.pop(), Some(4));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
  }
}
