use std::fs::read_to_string;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

mod app;
mod statics;
mod thread;
use thread::ThreadPool;

fn main() {
    let matches = app::app(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_DESCRIPTION"),
    )
    .get_matches();
    let listener: TcpListener = TcpListener::bind(matches.value_of("port").unwrap()).unwrap();
    println!("listening at {}", matches.value_of("port").unwrap());
    let pool = ThreadPool::new(
        matches
            .value_of("max")
            .unwrap()
            .parse::<usize>()
            .unwrap_or(statics::MAX_THREADS),
    );
    println!("Thread pool initialized with {} threads", pool.size());

    let path: &'static str = "./test_data/file.txt";

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle(stream, Path::new(path));
        })
    }
}

fn handle(mut stream: TcpStream, path: &Path) {
    let mut buffer: [u8; 512] = [0; 512];
    stream.read(&mut buffer).unwrap();

    // TODO: get path of file from request (probably use some abstraction of a request)

    // Useful for debugging; prints request
    // unsafe {
    //     println!("{}", String::from_utf8_unchecked(buffer.to_vec()));
    // }

    let response = match read_to_string(path) {
        Ok(p) => format!("{}{}", statics::STATUS_LINE_200, p),
        _ => format!(
            "{}{}",
            statics::STATUS_LINE_404,
            statics::ERROR_404_RESPONSE
        ),
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
