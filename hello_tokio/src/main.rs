use mini_redis::{client, Result};

// #[tokio::main]
// pub async fn main() -> Result<()> {
//     let mut client = client::connect("127.0.0.1:6379").await?;

//     client.set("hello", "world".into()).await?;

//     let result = client.get("hello").await?;

//     println!("got value from the server; result={:?}", result);

//     Ok(())
// }

async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() {
    // `say_world()` を呼び出しても、`say_world()` の中身は実行されない
    let op = say_world();

    // この println! が最初に実行される
    println!("hello");

    // `op` に対して `.await` を呼び出すことで、`say_world` の中身が実行される
    op.await;
}
