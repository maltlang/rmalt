use std::io;
use parser::lexer::lexer;
//use parser::parser;
use parser::parser;
use value::Eval;
use core::interpreter::ThreadContext;
use std::sync::Arc;
use std::io::Write;
//use std::collections::HashMap;
//use std::sync::Arc;

pub mod value;
pub mod func;
pub mod core;
pub mod parser;


fn main() {
    loop {
        // new ThreadContext
        let tc = Arc::from(ThreadContext::new());
        let _ = io::stdout().write(String::from("λ ").as_ref());
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        // lexer
        let tf = lexer(input.as_ref());
        // parser
        match parser(tf.as_ref()) {
            Ok(x) => for i in x {
                let o = i.eval(&tc);
                println!("=> {}: {}", o.get_type(), o.to_string());
            },
            Err((pos, info)) =>
                eprintln!("*** parser-error: {}, {}:{}", info, tf[pos].pos.line, tf[pos].pos.col),
        }
    }
}
