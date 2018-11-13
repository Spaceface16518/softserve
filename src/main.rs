extern crate libc;

use libc::c_char;
use std::{
    ffi::CStr,
    fs::read_to_string,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

mod app;
mod statics;
mod thread;
use thread::ThreadPool;

extern "C" {
    fn parse(s: *const c_char) -> *const c_char;
}

fn main() {
    let matches = app::app(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_DESCRIPTION"),
    )
    .get_matches();
    let listener: TcpListener = TcpListener::bind(
        matches
            .value_of("port")
            .expect("Could not get value of port parameter"),
    )
    .expect("Could not bind to the given address"); // TODO: handle bind error manually
    println!(
        "listening at {}",
        matches
            .value_of("port")
            .expect("Could not get value of port parameter")
    );
    let pool = ThreadPool::new(
        matches
            .value_of("max")
            .expect("Could not get value of max threads parameter")
            .parse::<usize>()
            .unwrap_or(statics::MAX_THREADS),
    );
    println!("Thread pool initialized with {} threads", pool.size());

    for stream in listener.incoming() {
        let stream = stream.expect(
            "The stream unwrapped with an Err value for `stream.incoming()`",
        );
        pool.execute(|| {
            handle(stream);
        })
    }
}

fn handle(mut stream: TcpStream) {
    let mut buffer: [u8; 512] = [0; 512];
    stream
        .read(&mut buffer)
        .expect("Could not read from stream into buffer");

    let path: &str = unsafe {
        CStr::from_ptr(parse(buffer.as_ptr() as *const c_char))
            .to_str()
            .unwrap()
    };

    // Useful for debugging; prints request
    // unsafe {
    //     println!("{}", String::from_utf8_unchecked(buffer.to_vec()));
    // }

    let response = match read_to_string(path) {
        Ok(p) => format!("{}{}", statics::STATUS_LINE_200, p),
        // TODO: more specific error handling
        _ => format!(
            "{}{}",
            statics::STATUS_LINE_404,
            statics::ERROR_404_RESPONSE
        ),
    };

    stream
        .write(response.as_bytes())
        .expect("Could not write to stream");
    stream.flush().expect("Error flushing stream");
}
