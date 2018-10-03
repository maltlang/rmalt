extern crate core;

use std::io;
use parser::lexer::lexer;
use parser::parser;
//use std::collections::HashMap;
//use std::sync::Arc;

pub mod value;
pub mod func;
pub mod parser;
pub mod runtime;

fn main() {
    loop {
        println!(">>>");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        // lexer
        let tf = lexer(input.as_ref());
        // parser
        match parser(tf.as_ref()) {
            Ok(x) => {
                for i in x {
                    println!("{}", i.to_string());
                }
            }
            Err((pos, info)) =>
                eprintln!("*** parser-error: {}, {}:{}", info, tf[pos].pos.line, tf[pos].pos.col),
        }
    }
}
