#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::Write;

use parser::lexer::lexer;
use parser::parser;
use runtime::context::ThreadContext;
use runtime::tools::exception_to_string;

pub mod value;
pub mod func;
pub mod parser;
pub mod runtime;


pub fn copyright() -> String {
    let os_info = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "android") {
        "android"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "ios") {
        "ios"
    } else {
        "unknown"
    };
    let arch_info = if cfg!(target_arch = "x86") {
        "x86"
    } else if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "arm") {
        "arm"
    } else if cfg!(target_arch = "powerpc") {
        "powerpc"
    } else if cfg!(target_arch = "powerpc64") {
        "powerpc64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        "unknown"
    };
    let env_info = if cfg!(target_env = "gnu") {
        "gnu"
    } else if cfg!(target_env = "msvc") {
        "msvc"
    } else if cfg!(target_env = "musl") {
        "musl"
    } else {
        "unknown"
    };

    "
     __      __       _   _     |  Repo:    github.com/maltlang/rmalt
    |  \\    /  |___ _| |_| |_   |  Version: Malt(rmalt v0.1 Beta), std(lyzhstd null), repl(malt-repl null)
    | \\ \\  / / /  _` | |_   _|  |  License: MIT
    | |\\ \\/ /| | |_| | | | |__  |  Author:  lyzh(Zhihang-liu) github.com/Zhihang-Liu
    |_| \\__/ |_\\___._|_| |___/  |  Target:  ".to_string() + arch_info + "-" + os_info + "-" + env_info + "\n\n" +
        "help-list: use = type
\tuse (help?) get docs (unavailable).
\tuse (exit!) or try use Ctrl-C quit repl.
"
}

fn main() {
    println!("{}", copyright());
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
                            /*
                            let o = match i.compiler_eval(&ic) {
                                Ok(o) => {
                                    //println!("{} -> {}", o.get_type(), o.to_string());
                                    o
                                }
                                Err(e) => {
                                    match exception_to_string(e.clone()) {
                                        Some(x) => eprintln!("{}", x),
                                        None => eprintln!("exception {}", e.to_string())
                                    }
                                    Value::Nil
                                }
                            };
                            */

                            match i.eval(&ic) {
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
