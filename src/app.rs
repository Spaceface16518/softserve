extern crate clap;

use self::clap::{App, Arg};

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
