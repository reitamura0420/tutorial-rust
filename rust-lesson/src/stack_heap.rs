enum List {
  Node(i32, Box<List>),
  Nil,
}

pub fn run() {
  // let a1: [u8; 9000000] = [1; 9000000];

  let mut v1 = vec![1, 2, 3, 4];
  let v2 = vec![5, 6, 7, 8];
  let mut v3 = vec![9, 10];

  println!("{:p}", &v1);
  println!("{:p}", &v2);

  println!("{:?}", v1.as_ptr());
  println!("{:?}", v2.as_ptr());
  println!("{}", v1.len());
  println!("{}", v2.len());
  println!("{}", v1.capacity());
  println!("{}", v2.capacity());

  v1.insert(1, 10);
  v1.remove(1);

  println!("{:?}", v1);

  v1.append(&mut v3);
  println!("{:?}", v1);

  let t1: (i64, String) = (10, String::from("hello"));
  println!("{:p}", &t1);

  println!("{:?}", t1.1.as_ptr());
  println!("{}", t1.1.len());
  println!("{}", t1.1.capacity());

  let mut b1 = Box::new(t1);
  (*b1).1 += "world";

  println!("{} {}", b1.0, b1.1);
}
