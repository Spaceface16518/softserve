use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::read_to_string;

mod statics;
mod thread;
use thread::ThreadPool;

fn main() {
    let bind = statics::PORT; // TODO: make port decider
    let listener: TcpListener = TcpListener::bind(bind).unwrap();
    println!("listening at {}", bind);
    let pool = ThreadPool::new(statics::MAX_THREADS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle(stream);
        })
    }
}

fn handle(mut stream: TcpStream) {
    let mut buffer: [u8; 512] = [0; 512];
    stream.read(&mut buffer).unwrap();

    // TODO: refactor this and include file loading, default response, and error404 support

    let (status_line, data) = if buffer.starts_with(statics::GET_PREFIX.as_bytes()) {
        (statics::STATUS_LINE_200, "test_data/file.txt")
    } else {
        (statics::STATUS_LINE_404, "test_data/error404.html")
    };

    let response = format!("{}{}", status_line, read_to_string(data).unwrap().as_str());

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

enum BufCheck {
    File(String),
    Def,
    Error404,
}
