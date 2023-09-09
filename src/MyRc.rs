use std::{
    cell::Cell,
    ops::{Deref, Drop},
    boxed::Box,
};

pub struct MyRc2<'a, T> {
    value: &'a T,
    strong_count: &'static Cell<usize>,
}

impl<'a, T> MyRc2<'a, T> {
    pub fn new(value: T) -> Self {
        Self {
            value: Box::leak(Box::new(value)),
            strong_count: Box::leak(Box::new(Cell::new(1))),
        }
    }

    pub fn strong_count(&self) -> usize {
        self.strong_count.get()
    }
}

impl<'a, T> Clone for MyRc2<'a, T> {
    fn clone(&self) -> Self {
        self.strong_count.set(self.strong_count.get() + 1);
        Self {
            value: self.value,
            strong_count: self.strong_count,
        }
    }
}

impl<'a, T> Deref for MyRc2<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'a, T> Drop for MyRc2<'a, T> {
    fn drop(&mut self) {
        self.strong_count.set(self.strong_count.get() - 1);
        if self.strong_count.get() == 0 {
            unsafe {
                // 这里强制将&T转换为&mut T其实是安全的，因为Box::leak()返回的就是&mut T
                { let _: Box<T> = Box::from_raw(std::mem::transmute(self.value)); }
                { let _: Box<Cell<usize>> = Box::from_raw(std::mem::transmute(self.strong_count)); }
            }
        }
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn my_rc_test() {
    let a = MyRc2::new(String::from("123"));
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