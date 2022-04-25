use std::mem::drop;

struct Pointer {
  data: String,
}

impl Drop for Pointer {
  fn drop(&mut self) {
    println!("Dropping Pointer with data {}!", self.data);
  }
}

pub fn run() {
  let c = Pointer {
    data: String::from("my stuff"),
  };
  let d = Pointer {
    data: String::from("other stuff"),
  };
  println!("finish");
  drop(c);
  println!("dropped before");
}
