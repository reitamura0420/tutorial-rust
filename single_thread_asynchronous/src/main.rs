use std::thread;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, contents) = if buffer.starts_with(get) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n\r\n", "get")
        // TODO リダイレクトでPOLに飛ぶ処理をかく
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(20));
        ("HTTP/1.1 200 OK\r\n\r\n", "sleep")
        // TODO リダイレクトでPOLに飛ぶ処理をかく
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404")
    };

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
