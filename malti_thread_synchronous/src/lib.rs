use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

trait FnBox {
  fn call_box(self: Box<Self>); // FnOnce()クロージャがcall_boxメソッドを使用できる
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
    (*self)()
  }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel(); // multi producer single consumer 受信と送信を扱う。川の流れ。

    let receiver = Arc::new(Mutex::new(receiver)); // Arc: スレッドセーフな参照カウント式ポインタ, 参照カウント：ライフサイクルの管理の一つ Mutex: 共有参照

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);

    self.sender.send(job).unwrap();
  }
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || {
      while let Ok(job) = receiver.lock().unwrap().recv() {
        println!("Worker {} got a job; executing.", id);

        job.call_box();
      }
    });

    Worker { id, thread }
  }
}

// spawnは所有権渡してるんですね。はい。
// pub fn spawn<F, T>(f: F) -> JoinHandle<T> // JoinHandle。あるスレッドに入るための所有許可。dropされると関連するスレッドを切り離す
//     where
//         F: FnOnce() -> T + Send + 'static, // FnOnce 全てのクロージャに対応。所有権を奪い、自身にムーブする。
//         T: Send + 'static
