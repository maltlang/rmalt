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
        match parser::parser(&_tf) {
            Ok(ref o) => for ref i in o {
                println!("Ast: {}", i.to_string());
            }
            Err(x) => eprintln!("SyntaxError {}:{}", &_tf[x].pos.line, &_tf[x].pos.col),
        }
    }
}
