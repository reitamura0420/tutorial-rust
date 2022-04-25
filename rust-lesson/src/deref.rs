use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.0
  }
}

pub fn run() {
  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);
}

// #[test]
// fn run_test() {
//   let x = 5;
//   let y = &x;

//   assert_eq!("{}", x);
//   assert_eq!("{}", *y);
// }
