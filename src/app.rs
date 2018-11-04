extern crate clap;

use self::clap::{App, Arg, ArgMatches};
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
        // MARK: max threads option
        .arg(
            Arg::with_name("max")
            .short("m")
            .long("maxt")
            .takes_value(true)
            .required(false)
            .help("Sets the maximum additional threads this server is allowed to spawn. The default is 4")
            .default_value("4")
        )
}
