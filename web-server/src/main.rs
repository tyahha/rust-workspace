use std::io::prelude::*;
use std::fs::File;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

enum Message {
    NewJob(Job),
    Terminate,
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// #Panics
    ///
    /// The 'new' function will panic if the size is zero.
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = std::thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                if let Message::NewJob(job) = message {
                    println!("Worker {} got a job: executing.", id);
                    job();
                } else {
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

type Job = Box<dyn FnOnce() + Send + 'static>;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shut down!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];

    stream.read(&mut buf).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buf[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_code, file_name_body) = if buf.starts_with(get) {
        ("200 OK", "hello")
    } else if buf.starts_with(sleep) {
        std::thread::sleep(std::time::Duration::from_secs(5));
        ("200 OK", "hello")
    } else {
        ("404 NOT FOUND", "404")
    };

    let file_path = format!("web-server/{}.html", file_name_body);
    let mut file = File::open(file_path).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status_code, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}