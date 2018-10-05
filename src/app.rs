extern crate serde_json as json;

use std::env::Args;
use std::fs::read_to_string;
use std::net::Ipv4Addr;

pub fn parse(argv: Args) -> Config {
    let argv = argv.collect();

    let (host, port) = parse_ip(argv[1]).get_self_components();

    let router = Router::from_json(Router::read_in_file(argv[2]));
    Config { host, port, router }
}

type Port = u32;

// COMBAK: Switch the roles of `parse_ip` and `ParseIp::new`?

fn parse_ip(raw: String) -> ParseIp {
    // TODO: write parse_ip method
}

/// A simple representation of the result of an IP parse, generally produced by
/// the method `parse_ip`. Can technically also be created by `ParseIp::new`
/// but this is discouraged as it has the same effect as calling the
/// `parse_ip` method (in fact it calls `parse_ip` internally - a design flaw?
/// maybe)
struct ParseIp {
    ip: Ipv4Addr,
    port: Port,
}

impl ParseIp {
    pub fn new(raw: String) -> ParseIp {
        parse_ip(raw)
    }
    pub fn to_full_repr(&self) -> String {
        format!("{:?}:{}", self.ip, self.port).to_string()
    }

    pub fn ip_octets(&self) -> [u8; 4] {
        self.ip.octets()
    }

    pub fn get_self_components(&self) -> (Ipv4Addr, Port) {
        (self.ip, self.port)
    }
}

struct Config {
    host: Ipv4Addr,
    port: Port,
    router: Router,
}

pub struct Router {
    table: Table,
}

impl Router {
    pub fn new(table: Table) -> Router {
        Router { table }
    }

    pub fn from_json(json: String) -> Router {
        Router::new(Router::deser_table(json))
    }

    pub fn deser_table(json: String) -> Table {
        // TODO: implement deser_table method of Router
    }

    pub fn read_in_file(file_path: &Path) -> String {
        read_to_string(file_path).unwrap().to_string()
    }
}

struct Table {
    zip: Vec<(FileSource, ServerEndpoint)>,
}

type FileSource = String;
type ServerEndpoint = String;
