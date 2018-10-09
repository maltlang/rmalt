#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use parser::lexer::lexer;
use parser::parser;
use runtime::context::ThreadContext;
use std::io::Write;
//use std::collections::HashMap;
//use std::sync::Arc;

pub mod value;
pub mod func;
pub mod parser;
pub mod runtime;

fn main() {
    // 创建上下文对象
    let ic = ThreadContext::test_new();
    loop {
        let _ = std::io::stdout().write("λ ".as_ref());
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        // lexer
        match lexer(input.as_ref()) {
            Ok(tf) =>
            // parser
                match parser(tf.as_ref()) {
                    Ok(x) => {
                        for i in x {
                            match i.eval(&ic) {
                                Ok(o) => println!("{} -> {}", o.get_type(), o.to_string()),
                                Err(e) => println!("{}", e.to_string()),
                            }
                        }
                    }
                    Err((pos, info)) =>
                        eprintln!("*** parser-error: {}, {}:{}", info, tf[pos].pos.line, tf[pos].pos.col),
                }
            Err(e) =>
                eprintln!("*** lexer-error: 字符串未结束, {}:{}", e.line, e.col),
        }
    }
}
