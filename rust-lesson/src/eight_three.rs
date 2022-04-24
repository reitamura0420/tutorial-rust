use std::collections::HashMap;
use std::io;

pub fn run() {
  let mut map = HashMap::new();
  loop {
    println!("Please select!! 1: open, 2: add, 3: end");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    match guess {
      1 => {
        println!("{:?}", map);
      }
      2 => {
        println!("Which department would you like to add");

        let mut department = String::new();

        io::stdin()
          .read_line(&mut department)
          .expect("Failed to read line");

        let department = department.trim().to_string();

        println!("Which employer do you want to add");

        let mut employer = String::new();

        io::stdin()
          .read_line(&mut employer)
          .expect("Failed to read line");

        let employer = employer.trim().to_string();

        map.insert(department, employer);

        println!("The addition is complete");
      }
      3 => {
        break;
      }
      _ => {
        println!("値が不正でもう一度入力して下さい");
        continue;
      }
    }
  }
}
