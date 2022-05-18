// use mini_redis::{client, Result};

// #[tokio::main]
// pub async fn main() -> Result<()> {
//   // mini-redis アドレスへのコネクションを開く
//   let mut client = client::connect("127.0.0.1:6379").await?;

//   // "hello" というキーに "world" という値をセット
//   client.set("hello", "world".into()).await?;

//   // "hello" の値を取得
//   let result = client.get("hello").await?;

//   println!("got value from the server; result={:?}", result);

//   Ok(())
// }

// use tokio::task;

// #[tokio::main]
// async fn main() {
//   let v = vec![1, 2, 3];

//   task::spawn(async move {
//     println!("Here's a vec: {:?}", v);
//   });

//   // println!("Here's a vec: {:?}", v); これは所有権渡してるからエラーになる
// }

// use std::rc::Rc;
// use tokio::task::yield_now;

// #[tokio::main]
// async fn main() {
//   tokio::spawn(async {
//     // { } で囲っていることにより、`rc` が `.await` の前に drop される
//     {
//       let rc = Rc::new("hello");
//       println!("{}", rc);
//     }

//     // `rc` はもはや使用されない。
//     // タスクがスケジューラに戻るときには破棄されている
//     yield_now().await;
//   });
// }
