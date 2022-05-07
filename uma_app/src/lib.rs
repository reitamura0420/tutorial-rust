#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use self::models::{NewPost, Post};

trait FnBox {
  fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
    (*self)()
  }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

impl ThreadPool {
  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);

    self.sender.send(Message::NewJob(job)).unwrap();
  }

  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { workers, sender }
  }
}

struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let message = receiver.lock().unwrap().recv().unwrap();

      match message {
        Message::NewJob(job) => {
          println!("Worker {} got a job; executing.", id);

          job.call_box();
        }
        Message::Terminate => {
          // ワーカー{}は停止するよう指示された
          println!("Worker {} was told to terminate.", id);

          break;
        }
      }
    });
    Worker {
      id,
      thread: Some(thread),
    }
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Sending terminate message to all workers.");

    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }
    println!("Shutting down all workers.");
    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);

      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

enum Message {
  NewJob(Job),
  Terminate,
}

pub fn establish_connection() -> MysqlConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  MysqlConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
#[test]
fn test_worker() {
  let (sender, receiver) = mpsc::channel();
  let receiver = Arc::new(Mutex::new(receiver));
  let worker = Worker::new(1, receiver);

  assert_eq!(worker.id, 1);
  assert!(worker.thread.is_some());
}

pub fn create_post(
  conn: &MysqlConnection,
  name: &str,
  evaluation_point: &str,
  skill_point: &str,
) -> Post {
  use schema::posts;

  let new_post = NewPost {
    name,
    evaluation_point,
    skill_point,
  };

  diesel::insert_into(posts::table)
    .values(&new_post)
    .execute(conn)
    .expect("Error saving new post");

  posts::table.order(posts::id.desc()).first(conn).unwrap()
}
