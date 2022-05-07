extern crate uma_app;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use uma_app::ThreadPool;

use diesel::prelude::*;
use uma_app::*;

use self::models::{NewPost, Post};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(100) {
        match stream {
            Ok(s) => {
                pool.execute(|| {
                    handle_connection(s);
                });
            }
            Err(e) => eprintln!("stream error, {}", e),
        }
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let view_html = b"GET / HTTP/1.1\r\n";
    let get_posts = b"GET /get_posts HTTP/1.1\r\n";
    let insert_posts = b"POST /insert_posts HTTP/1.1\r\n";
    let update_posts = b"PUT /update_posts HTTP/1.1\r\n";

    let response = if buffer.starts_with(view_html) {
        let filename = "hello.html";
        let status_line = "HTTP/1.1 200 OK\r\n\r\n";
        let mut file = File::open(filename).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        format!("{} {}", status_line, contents)
    } else if buffer.starts_with(get_posts) {
        let status_line = "HTTP/1.1 200 /get_posts OK\r\n\r\n";
        let result = get_posts_data();
        let serialized = serde_json::to_string(&result).unwrap();
        format!("{} {:?}", status_line, serialized)
    } else if buffer.starts_with(insert_posts) {
        let status_line = "HTTP/1.1 200 /insert_posts OK\r\n\r\n";
        insert_posts_data();
        format!("{} {}", status_line, "true")
    } else {
        let filename = "404.html";
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut file = File::open(filename).unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        format!("{} {}", status_line, contents)
    };

    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}

fn get_posts_data() -> Vec<Post> {
    use self::schema::posts::dsl::*;
    let connection = establish_connection();
    println!("test2");
    let result = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.name);
    //     println!("-----------\n");
    //     println!("{}", post.evaluation_point);
    //     println!("-----------\n");
    //     println!("{}", post.skill_point);
    // }

    result
}

fn insert_posts_data() {
    use schema::posts;

    let connection = establish_connection();

    let uma_name = "トウカイテイオー";
    let skill = "20000";
    let evaluation = "19500";

    println!("What would you like your title to be?");

    let new_post = NewPost {
        name: uma_name,
        skill_point: skill,
        evaluation_point: evaluation,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&connection)
        .expect("Error saving new post");
}
