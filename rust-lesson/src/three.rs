fn change_fahrenheit_to_celsius(fahrenheit: &u32) -> u32 {
  fahrenheit + 273
}

fn calculate_fibonacci(count: &u32) -> u32 {
  let mut fibonacci = (0, 1);
  for _number in 1..*count + 1 {
    let temp = fibonacci;
    fibonacci = (temp.1, temp.0 + temp.1);
  }
  fibonacci.1
}

pub fn run() {
  let fahrenheit: u32 = 3;
  let count: u32 = 2;
  println!("celsius: {}", change_fahrenheit_to_celsius(&fahrenheit));
  println!("fibonacci: {}", calculate_fibonacci(&count));
}
