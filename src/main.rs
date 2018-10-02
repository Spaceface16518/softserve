use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

mod statics;
mod thread;

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

    let (status_line, data) = if buffer.starts_with(statics::GET_PREFIX.as_bytes()) {
        (statics::STATUS_LINE_200, statics::DEF_RESPONSE)
    } else {
        (statics::STATUS_LINE_404, statics::ERROR_404_RESPONSE)
    };

    let response = format!("{}{}", status_line, data);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
