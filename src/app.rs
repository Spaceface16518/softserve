extern crate clap;

use clap::{App, Arg};

#[macro_export]
macro_rules! app {
    ($params:expr) => {
        App::new($params.name())
            .version($params.version())
            .author($params.authors())
            .about($params.description())
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
    };
}

type param_inner = &str;

pub struct AppParams {
    name: param_inner,
    version: param_inner,
    author: param_inner,
    description: param_inner,
}

impl AppParams {
    pub fn new(
        name: param_inner,
        version: param_inner,
        author: param_inner,
        description: param_inner,
    ) -> AppParams {
        AppParams {
            name,
            version,
            author,
            description,
        }
    }

    pub fn name(&self) -> param_inner {
        self.name
    }

    pub fn version(&self) -> param_inner {
        self.version
    }

    pub fn author(&self) -> param_inner {
        self.author
    }

    pub fn description(&self) -> param_inner {
        self.description
    }
}
