use reqwest::Client;
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
        let client = Client::new();
        let url = "http://127.0.0.1:8080";
        let response = client.get(url).send().await.unwrap();
        let body = response.text().await.unwrap();
        ("HTTP/1.1 200 OK\r\n\r\n", body)
    } else if buffer.starts_with(sleep) {
        let client = Client::new();
        let url = "http://127.0.0.1:8080/sleep";
        let response = client.get(url).send().await.unwrap();
        let body = response.text().await.unwrap();
        ("HTTP/1.1 200 OK\r\n\r\n", body)
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", String::from("404"))
    };

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
