//use std::io;

extern crate num;

mod runtime;
mod parser;

fn main() {
    let r =
        parser::parse_src::parse_atom(&parser::parserc::StrStream::new(
            "#false"
        ));
    println!("{:?}", r);
}
