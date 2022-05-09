extern crate uma_app;
use percent_encoding::percent_decode;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use uma_app::ThreadPool;

use diesel::prelude::*;
use uma_app::*;

use self::models::{ChangePost, NewPost, Post};

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
    let update_posts = b"POST /update_posts HTTP/1.1\r\n";
    let delete_posts = b"POST /delete_posts HTTP/1.1\r\n";

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
        let req_msg = str::from_utf8(&buffer).unwrap();
        let re = Regex::new(r"name=.*$").unwrap();
        let caps = re.captures(req_msg).unwrap();
        let request_text = caps.at(0).unwrap().trim();
        let request_text_list = request_text.split('&').fold(Vec::new(), |mut s, i| {
            s.push(i.to_string());
            s
        });
        let format_text_list: Vec<String> = request_text_list
            .iter()
            .map(|x| {
                let re_before_equal = Regex::new(r"(.*)=").unwrap();
                re_before_equal.replace_all(x, "").to_string()
            })
            .collect();
        let mut iter = percent_decode(format_text_list[0].as_bytes())
            .decode_utf8()
            .unwrap();
        let name = iter.to_mut();
        let re_number_equal = Regex::new(r"\d+").unwrap();
        let score_str = re_number_equal
            .captures(&format_text_list[2])
            .unwrap()
            .at(0)
            .unwrap();
        insert_posts_data(name, &format_text_list[1], score_str);
        format!("{} {}", status_line, "true")
    } else if buffer.starts_with(update_posts) {
        let status_line = "HTTP/1.1 200 /update_posts OK\r\n\r\n";
        let req_msg = str::from_utf8(&buffer).unwrap();
        println!("{}", req_msg);
        let re = Regex::new(r"id=.*$").unwrap();
        let caps = re.captures(req_msg).unwrap();
        let request_text = caps.at(0).unwrap().trim();
        let request_text_list = request_text.split('&').fold(Vec::new(), |mut s, i| {
            s.push(i.to_string());
            s
        });
        let format_text_list: Vec<String> = request_text_list
            .iter()
            .map(|x| {
                let re_before_equal = Regex::new(r"(.*)=").unwrap();
                re_before_equal.replace_all(x, "").to_string()
            })
            .collect();
        let id: i32 = format_text_list[0].parse::<i32>().unwrap();
        let mut iter = percent_decode(format_text_list[1].as_bytes())
            .decode_utf8()
            .unwrap();
        let name = iter.to_mut();
        let re_number_equal = Regex::new(r"\d+").unwrap();
        let score_str = re_number_equal
            .captures(&format_text_list[3])
            .unwrap()
            .at(0)
            .unwrap();
        update_posts_data(id, name, &format_text_list[2], score_str);
        format!("{} {}", status_line, "true")
    } else if buffer.starts_with(delete_posts) {
        let status_line = "HTTP/1.1 200 /delete_posts OK\r\n\r\n";
        let req_msg = str::from_utf8(&buffer).unwrap();
        let re = Regex::new(r"id=.*$").unwrap();
        let caps = re.captures(req_msg).unwrap();
        let request_text = String::from(caps.at(0).unwrap().trim());
        let re_before_equal = Regex::new(r"(.*)=").unwrap();
        let id_str = re_before_equal.replace_all(&request_text, "").to_string();
        let re_number_equal = Regex::new(r"\d+").unwrap();
        let id_str2 = re_number_equal.captures(&id_str).unwrap().at(0).unwrap();
        println!("{}", id_str2.len());
        println!("{}", id_str2.trim().len());
        println!("{}", String::from("1").len());
        delete_posts_data(id_str2.parse().unwrap());
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
    let result = posts
        .load::<Post>(&connection)
        .expect("Error loading posts");
    result
}

fn insert_posts_data(uma_name: &str, evaluation: &str, skill: &str) {
    use schema::posts;

    let connection = establish_connection();

    let new_post = NewPost {
        name: uma_name,
        evaluation_point: evaluation,
        skill_point: skill,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&connection)
        .expect("Error saving new post");
}

fn update_posts_data(id: i32, uma_name: &str, evaluation: &str, skill: &str) {
    use self::schema::posts::dsl::posts;
    let connection = &mut establish_connection();

    println!("{}", id);
    println!("{}", uma_name);
    println!("{}", evaluation);
    println!("{}", skill);

    let new_post = ChangePost {
        name: Some(uma_name),
        evaluation_point: Some(evaluation),
        skill_point: Some(skill),
    };

    diesel::update(posts.find(id))
        .set(&new_post)
        .execute(connection)
        .expect("error");
}

fn delete_posts_data(id: i32) {
    use self::schema::posts::dsl::posts;
    let connection = &mut establish_connection();

    diesel::delete(posts.find(id))
        .execute(connection)
        .expect("Error deleting posts");
}
