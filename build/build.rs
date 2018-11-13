extern crate cc;

fn main() {
    cc::Build::new().file("src/parse.c").compile("parse")
}
