use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex}; // TODO tokio::sync::Mutexとの違いを調べる

use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

#[tokio::main]
async fn main() {
    // リスナーをこのアドレスにバインドする
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // タプルの2つ目の要素は、新しいコネクションのIPとポートの情報を含んでいる
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // let mut db = HashMap::new(); // データの蓄積用
    let shard = db[hash(key) % db.len()].lock().unwrap();

    let mut connection = Connection::new(socket); // TODO 余裕があったらmini-redisについて調べる

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
