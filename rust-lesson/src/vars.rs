pub mod sub_a;
mod sub_b;

const MAX_POINT: u32 = 100_000;
const _STR_TEST: &str = "test";

pub fn run() {
  let mut x = 10;
  println!("{}", x);
  x = 20;
  println!("{}", MAX_POINT);
  let _i1 = 10;
  let _f1 = 0.1;
  println!("{}", usize::BITS);
  println!("{:p}", &MAX_POINT);

  let i2: i64 = 1;
  let i3: i64 = 2;
  println!("{:p}", &i2);
  println!("{:p}", &i3);

  let y = 5;
  println!("{:p}", &y);
  let y = y + 1;
  println!("{}", y);
  {
    let y = 0;
    println!("{}", y);
  }
  println!("{}", y);

  let t1 = (500, 6.4, "dummy");
  let (x, y, z) = t1;
  println!("{} {} {}", t1.0, t1.1, t1.2);

  let mut t2 = ((0, 1), (2, 3));
  let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
  *x1_ptr = 5;
  *y1_ptr = -5;
  println!("{:?}", t2);

  let a1 = [1, 2, 3, 4, 5];
  let a2 = [0; 10];
  println!("{:?}", a1);
  println!("{:?}, {}", a2, a2[5]);

  let mut s1 = "helloイエイ";
  let s2 = "hello";

  println!("{}", s1);
  println!("{:p}", &s2);
  println!("{:?}", s1.as_ptr());
  println!("{:?}", s2.as_ptr());
  println!("{}", s1.len());
  println!("{}", s2.len());

  let mut s1 = String::from("hello");
  let mut s2 = String::from("HelloHappyWorld");

  println!("{:p}", &s1);
  println!("{:p}", &s2);

  println!("{:?}", s1.as_ptr());
  println!("{:?}", s2.as_ptr());
  println!("{}", s1.len());
  println!("{}", s2.len());
  println!("{}", s1.capacity());
  println!("{}", s2.capacity());

  s1.push_str("_new1");
  s2.push_str("_new2");
  println!("{:?}", s1);
  println!("{:?}", s2);
}
