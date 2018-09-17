use std::io;
use std::collections::HashMap;
use std::sync::Arc;

pub mod value;
pub mod func;
pub mod core;
pub mod parser;


fn main() {
    loop {
        println!(">>>");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let _tf = parser::lexer::lexer(&input);
        for i in &_tf {
            println!("{}", i.to_string());
        }
    }
}
