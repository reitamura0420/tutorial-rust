pub fn run() {
  let favorite_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();

  if let Some(color) = favorite_color {
    println!("{}", color);
  } else if is_tuesday {
    println!("green");
  } else if let Ok(age) = age {
    if age > 30 {
      println!("purple");
    } else {
      println!("orange");
    }
  } else {
    println!("blue");
  }
}

pub fn run2() {
  let mut stack = Vec::new();

  stack.push(1);
  stack.push(2);
  stack.push(3);

  while let Some(top) = stack.pop() {
    println!("{}", top);
  }
}

pub fn run3() {
  let v = vec![1, 2, 3];
  for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
  }
}

struct Point {
  x: i32,
  y: i32,
}

pub fn run4() {
  let p = Point { x: 0, y: 7 };

  match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

pub fn run5() {
  let msg = Message::ChangeColor(0, 160, 255);

  match msg {
    Message::Quit => {
      println!("The Quit variant has no data to destructure.")
    }
    Message::Move { x, y } => {
      println!("Move in the x direction {} and in the y direction {}", x, y);
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
      println!("Change the color to red {}, green {}, and blue {}", r, g, b)
    }
  }
}

pub fn run6() {
  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, _, third, _, fifth) => {
      // 何らかの数値: {}, {}, {}
      println!("Some numbers: {}, {}, {}", first, third, fifth)
    }
  }

  match numbers {
    (first, .., last) => {
      println!("Some numbers: {}, {}", first, last);
    }
  }

  let s = Some(String::from("Hello!"));

  if let Some(_) = s {
    println!("found a string");
  }

  println!("{:?}", s);
}

pub fn run7() {
  let mut robot_name = Some(String::from("Bors"));

  match robot_name {
    Some(ref mut name) => *name = String::from("Another name"),
    None => (),
  }

  println!("robot_name is: {:?}", robot_name);
}

pub fn run8() {
  let x = 4;
  let y = false;

  match x {
    4..=6 if y => println!("yes"),
    _ => println!("no"),
  }
}

enum Message2 {
  Hello { id: i32 },
}

pub fn run9() {
  let msg = Message2::Hello { id: 5 };

  match msg {
    Message2::Hello {
      id: id_variable @ 3..=7,
    } => {
      // 範囲内のidが見つかりました: {}
      println!("Found an id in range: {}", id_variable)
    }
    Message2::Hello { id: 10..=12 } => {
      // 別の範囲内のidが見つかりました
      println!("Found an id in another range")
    }
    Message2::Hello { id } => {
      // それ以外のidが見つかりました
      println!("Found some other id: {}", id)
    }
  }
}
