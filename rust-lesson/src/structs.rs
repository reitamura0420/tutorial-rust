#[derive(Debug)]
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn create(width: u32, height: u32) -> Self {
    Self { width, height }
  }
  fn area(&self) {
    println!("{}", self.width * self.height);
  }
}

pub fn run() {
  let user1 = User {
    email: String::from("test@gamil.com"),
    username: String::from("dummy name"),
    active: true,
    sign_in_count: 1,
  };

  let mut user1 = User {
    email: String::from("test@gamil.com"),
    username: String::from("dummy name"),
    active: true,
    sign_in_count: 1,
  };

  user1.email = String::from("test2@gmail.com");
  println!("{:#?}", user1);

  let user2 = build_user(String::from("test3@gmail.com"), String::from("dummy name2"));

  let rect = Rectangle::create(20, 20);
  println!("{:#?}", rect);
  println!("{:#?}", rect.area());
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}
