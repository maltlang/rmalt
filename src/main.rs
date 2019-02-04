//use std::io;

extern crate num;

mod runtime;
mod parser;

fn main() {
    //println!("{}", copyright());
    /*
    loop {
        println!(">>> ");
        let mut src = String::new();
        let _ = io::stdin().read_line(&mut src);
        let token_stream = lexer::lexer(src.trim_right_matches("\r")).unwrap();
        println!("{:?}", token_stream);
    }
    */
    //let x = parser::once_parser("(1)".to_string());
    //println!("{:?}", x);

    parser::test_parse_c();
    parser::test_parse_str();
    parser::test_parse()
}
