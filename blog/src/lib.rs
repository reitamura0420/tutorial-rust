pub struct Post {
  pub content: String,
  pub is_publish: bool,
  pub is_request_review: bool,
}

impl Post {
  pub fn new() -> Post {
    Post {
      content: String::from(""),
      is_publish: false,
      is_request_review: false,
    }
  }

  pub fn add_text(&mut self, value: &str) {
    self.content.push_str(value);
  }

  pub fn request_review(&mut self) {
    self.is_request_review = true;
  }

  pub fn approve(&mut self) {
    self.is_request_review = false;
    self.is_publish = true;
  }

  pub fn content(&self) -> String {
    if self.is_publish && !(self.is_request_review) {
      self.content.to_string()
    } else {
      String::from("")
    }
  }
}
