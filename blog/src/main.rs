extern crate blog;

use blog::Post;

fn main() {
    let mut post = Post::new();

    // 今日はお昼にサラダを食べた
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
