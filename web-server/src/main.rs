use std::io::prelude::*;
use std::fs::File;
use std::net::{TcpListener, TcpStream};

struct ThreadPool;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
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