use std::io;
use std::collections::HashMap;
use std::sync::Arc;

pub mod value;
pub mod ast;
pub mod func;
pub mod core;
pub mod parser;


fn main() {
    loop {
        println!(">>>");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let _tf = parser::lexer::lexer(&input);
        if _tf.len() == 0 { continue; }
        let ast = parser::parser_once(&_tf, 0);
        match parser::parser_once(&_tf, 0) {
            Ok((o, idx)) => println!("{}, {}:{}", o.to_string(), o.pos.line, o.pos.col),
            Err(x) => eprintln!("SyntaxError {}:{}", &_tf[x].pos.line, &_tf[x].pos.col),
        }
    }
}
