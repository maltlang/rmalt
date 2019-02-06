//use std::io;

extern crate num;

mod runtime;
mod parser;

fn main() {
    let r =
        parser::parse_atom(&parser::StrStream::new(
            "#false"
        ));
    println!("{:?}", r);
}
