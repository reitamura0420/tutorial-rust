pub fn run() {
  let s1 = String::from("hello");
  let s2 = s1;

  println!("{}", s2);

  let i1 = 1;
  let i2 = i1;

  println!("{} {}", i1, i2);

  println!("{:p}", &i1);
  println!("{:p}", &i2);

  let sl1 = "literal";
  let sl2 = sl1;

  println!("{:p}", &sl1);
  println!("{:p}", &sl2);
}
