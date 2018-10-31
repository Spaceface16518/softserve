extern crate clap;

use self::clap::{App, Arg};
use std::net::SocketAddrV4;
use std::path::Path;

#[inline]
pub fn app<'a, 'b>(
    name: &'b str,
    version: &'b str,
    author: &'b str,
    description: &'b str,
) -> App<'a, 'b> {
    App::new(name)
        .version(version)
        .author(author)
        .about(description)
        // MARK: arg port
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .required(true)
                .help("Sets the port to listen at"),
        )
        // MARK: arg path
        .arg(
            Arg::with_name("path")
                .short("s")
                .long("path")
                .takes_value(true)
                .required(true)
                .help("Sets the path of the public asset folder"),
        )
}

pub struct Config<'a> {
    addr: &'a SocketAddrV4,
    path: &'a Path,
}

impl<'a> Config<'a> {
    pub fn new(addr: &'a SocketAddrV4, path: &'a Path) -> Config<'a> {
        Config { addr, path }
    }
}
