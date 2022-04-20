pub fn run() {
  let a = [10, 20, 30, 40, 50];

  for element in a.iter() {
    // 値は{}です
    println!("the value is: {}", element);
  }

  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("LIFTOFF!!!");
}
