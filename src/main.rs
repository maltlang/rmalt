#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use parser::lexer::lexer;
use parser::parser;
use runtime::context::ThreadContext;
use std::io::Write;
use value::Value;
use runtime::tools::exception_to_string;
//use std::collections::HashMap;
//use std::sync::Arc;

pub mod value;
pub mod func;
pub mod parser;
pub mod runtime;

fn main() {
    // 创建上下文对象
    let ic = ThreadContext::new();
    loop {
        let _ = std::io::stdout().write("λ ".as_ref());
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).
            unwrap();
        //input.trim_right();
        // lexer
        match lexer(input.as_ref()) {
            Ok(tf) =>
            // parser
                match parser(tf.as_ref()) {
                    Ok(x) => {
                        for i in x {
                            let o = match i.compiler_eval(&ic) {
                                Ok(o) => o,
                                Err(e) => {
                                    eprintln!("{}", e.to_string());
                                    Value::Nil
                                }
                            };
                            match o.eval(&ic) {
                                Ok(o) => println!("{} -> {}", o.get_type(), o.to_string()),
                                Err(e) => match exception_to_string(e.clone()) {
                                    Some(x) => eprintln!("{}", x),
                                    None => eprintln!("exception {}", e.to_string())
                                },
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
