extern crate clap;

use self::clap::{App, Arg, ArgMatches};
use std::net::SocketAddrV4;

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

pub fn handle_matches(matches: ArgMatches) -> Config {
    let addr: SocketAddrV4 = matches.value_of("port").unwrap().parse().unwrap();
    let path: String = matches.value_of("path").unwrap().to_string();
    Config {
        addr: addr.clone(),
        path: path.clone(),
    }
}

pub struct Config {
    addr: SocketAddrV4,
    path: String,
}

impl Config {
    pub fn new(addr: SocketAddrV4, path: String) -> Config {
        Config { addr, path }
    }
}
