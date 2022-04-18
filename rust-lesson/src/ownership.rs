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

  let s3 = String::from("hello");
  let s4 = s3.clone();

  println!("{:p}", &s3);
  println!("{:p}", &s4);

  println!("{:?}", s3.as_ptr());
  println!("{:?}", s4.as_ptr());
  println!("{} {}", s3, s4);

  let s5 = String::from("hello");
  println!("{:p}", &s5);
  println!("{:?}", s5.as_ptr());
  println!("{}", s5.len());
  println!("{}", s5.capacity());
  take_ownership(s5);
  // println!("{}", s5);

  let s6 = String::from("hello");
  println!("{:p}", &s6);
  println!("{:?}", s6.as_ptr());
  println!("{}", s6.len());
  println!("{}", s6.capacity());
  let s7 = take_ownership(s6);
  println!("{:p}", &s7);
  println!("{:?}", s7.as_ptr());
  println!("{}", s7.len());
  println!("{}", s7.capacity());

  let len = calculate_length(&s7);
  println!("{} {}", s7, len);

  let mut s9 = String::from("hello");
  change(&mut s9);
  println!("{}", s9);

  let s10 = String::from("hello");
  let r1 = &s10;
  let r2 = &s10;

  // let mut s10 = String::from("hello");
  // let r1 = &s10;
  // let r2 = &mut s10;

  // println!("{}, {}, {}", s10, r1, r2);

  let mut s11 = String::from("hello");
  let r1 = &mut s11;
  println!("{}", r1);
  println!("{}", s11);

  let mut s12 = String::from("hello");
  let r1 = &s12;
  let r2 = &s12;
  println!("{} {}", r1, r2);
  let r3 = &mut s12;
  *r3 = String::from("hello_update");
}

fn take_ownership(s: String) -> String {
  // println!("{}", s);
  // println!("{:p}", &s);
  // println!("{:?}", s.as_ptr());
  // println!("{}", s.len());
  // println!("{}", s.capacity());
  s
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change(s: &mut String) {
  s.push_str("_world");
}
