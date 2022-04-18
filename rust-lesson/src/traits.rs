trait Fruits {
  fn price(&self) -> u32;
}

struct Apple;

impl Fruits for Apple {
  fn price(&self) -> u32 {
    10
  }
}

struct Banana;
impl Fruits for Banana {
  fn price(&self) -> u32 {
    5
  }
}

trait Summary {
  fn summarize(&self) -> String {
    String::from("(Read More...)")
  }
}

struct NewsArticle {
  headline: String,
  location: String,
  author: String,
  content: String,
}

impl Summary for NewsArticle {
  // fn summarize(&self) -> String {
  //   format!("{}, by {} ({})", self.headline, self.author, self.location)
  // }
}

impl Message for NewsArticle {}

struct Tweet {
  username: String,
  content: String,
  reply: bool,
  retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}, by {} ", self.username, self.content)
  }
}

trait Message {
  fn message(&self) -> String {
    String::from("Message")
  }
}

pub fn run() {
  let apple = Apple {};
  let banana = Banana {};
  get_price(apple);
  get_price(banana);

  let tweet = Tweet {
    username: String::from("chell"),
    content: String::from("Rust learn now"),
    reply: false,
    retweet: false,
  };

  let article = NewsArticle {
    headline: String::from("2022/04/18"),
    location: String::from("Tokyo"),
    author: String::from("chell"),
    content: String::from("Rust learn now"),
  };

  println!("{}", tweet.summarize());

  notify(&article);
  notify_another(&article);
}

fn get_price<T: Fruits>(fruits: T) {
  println!("price is: {}", fruits.price());
}

fn notify(item: &impl Summary) {
  println!("{}", item.summarize());
}

fn notify_another(item: &(impl Summary + Message)) {
  println!("{}", item.summarize());
  println!("{}", item.message());
}
