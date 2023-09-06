use std::cell::RefCell;
use std::clone::Clone;
use std::ops::{Deref, Drop};
use std::ptr::NonNull;
use std::boxed::Box;

struct RcBox<T> {
  strong: RefCell<usize>,
  value: T,
}

impl<T> RcBox<T> {
  fn strong_count(&self) -> usize {
    *self.strong.borrow()
  }
  fn set_strong(&mut self, s: usize) {
    *self.strong.get_mut() = s;
  }
}

pub struct MyRc<T> {
  ptr: RefCell<NonNull<RcBox<T>>>
}

impl<T> MyRc<T> {
  pub fn new(value: T) -> MyRc<T> {
    MyRc {
      ptr: RefCell::new(NonNull::new(Box::into_raw(Box::new(
        RcBox {
          strong: RefCell::new(1),
          value
        }
      ))).unwrap())
    }
  }

  pub fn strong_count(&self) -> usize {
    unsafe {
      self.ptr.borrow().as_ref().strong_count()
    }
  }
}

impl<T> Clone for MyRc<T> {
  fn clone(&self) -> Self {
    unsafe {
      let s = self.ptr.borrow().as_ref().strong_count();
      self.ptr.borrow_mut().as_mut().set_strong(s + 1);
    }
    Self {
      ptr: self.ptr.clone()
    }
  }
}

impl<T> Deref for MyRc<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    unsafe {
      &self.ptr.borrow().as_ref().value
    }
  }
}

impl<T> Drop for MyRc<T> {
  fn drop(&mut self) {
    unsafe {
      let s = self.ptr.borrow().as_ref().strong_count();
      if s == 1 {
        let _value = Box::from_raw(self.ptr.borrow().as_ptr());
      } else {
        self.ptr.borrow_mut().as_mut().set_strong(s - 1);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn my_rc_test() {
    let a = MyRc::new("123");
    assert_eq!(a.strong_count(), 1);
    let b = a.clone();
    assert_eq!(a.strong_count(), 2);
    assert_eq!(b.strong_count(), 2);
    assert_eq!(*a, "123");
    assert_eq!(*b, "123");

    {
      let c = b.clone();
      assert_eq!(a.strong_count(), 3);
      assert_eq!(b.strong_count(), 3);
      assert_eq!(c.strong_count(), 3);
      assert_eq!(*c, "123");
    }
    assert_eq!(a.strong_count(), 2);
    assert_eq!(b.strong_count(), 2);
  }
}