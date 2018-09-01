extern crate regex;

use parser::token::TokenValue;
use parser::token::TokenPos;
use parser::token::Token;
use std::sync::Arc;

//use regex::Regex;
use self::regex::Regex;

enum Mode {
    CODE,
    NOTE,
    STRING,
}

fn other_get(s: &str) -> TokenValue {
    let nr: regex::Regex = regex::Regex::new(r"(-|\+)?[0-9]+[.][0-9]+").unwrap();
    let ir: regex::Regex = regex::Regex::new(r"(-|\+)?[0-9]+").unwrap();
    if nr.is_match(s) {
        return match s.parse() {
            Ok(x) => TokenValue::FLOAT(x),
            Err(_) => TokenValue::SYMBOL(Arc::new(s.to_string())),
        };
    }
    if ir.is_match(s) {
        if s.chars().nth(0).unwrap() == '+' || s.chars().nth(0).unwrap() == '-' {
            return match s.parse() {
                Ok(x) => TokenValue::INT(x),
                Err(_) => TokenValue::SYMBOL(Arc::new(s.to_string())),
            };
        } else {
            return match s.parse() {
                Ok(x) => TokenValue::UINT(x),
                Err(_) => TokenValue::SYMBOL(Arc::new(s.to_string())),
            };
        }
    }
    TokenValue::SYMBOL(Arc::new(s.to_string()))
}

//impl Token {
pub fn lexer(s: &str) -> Vec<Token> {
    let mut line: usize = 1;
    let mut col: usize = 1;

    let mut mode = Mode::CODE;
    let mut strpos = TokenPos { line: 0, col: 0 };
    let mut strbuf = String::new();
    let mut rs: Vec<Token> = vec![];
    for i in s.chars() {
        match mode {
            Mode::CODE => {
                match i {
                    '#' => {
                        if (*strbuf).len() > 0 {
                            rs.push(Token {
                                val: other_get(&strbuf),
                                pos: strpos,
                            });
                            strpos = TokenPos { line: 0, col: 0 };
                            strbuf.clear();
                        };
                        mode = Mode::NOTE;
                        continue;
                    }
                    ' ' | '\t' => {
                        if (*strbuf).len() > 0 {
                            rs.push(Token {
                                val: other_get(&strbuf),
                                pos: strpos,
                            });
                            strpos = TokenPos { line: 0, col: 0 };
                            strbuf.clear();
                        };
                        col += 1;
                    }
                    '\n' => {
                        if (*strbuf).len() > 0 {
                            rs.push(Token {
                                val: other_get(&strbuf),
                                pos: strpos,
                            });
                            strpos = TokenPos { line: 0, col: 0 };
                            strbuf.clear();
                        };
                        line += 1;
                        col = 0;
                    }
                    '(' => {
                        if (*strbuf).len() > 0 {
                            rs.push(Token {
                                val: other_get(&strbuf),
                                pos: strpos,
                            });
                            strpos = TokenPos { line: 0, col: 0 };
                            strbuf.clear();
                        };
                        rs.push(Token {
                            val: TokenValue::LP,
                            pos: TokenPos {
                                line,
                                col,
                            },
                        });
                    }
                    ')' => {
                        if (*strbuf).len() > 0 {
                            rs.push(Token {
                                val: other_get(&strbuf),
                                pos: strpos,
                            });
                            strpos = TokenPos { line: 0, col: 0 };
                            strbuf.clear();
                        };
                        rs.push(Token {
                            val: TokenValue::RP,
                            pos: TokenPos {
                                line,
                                col,
                            },
                        });
                    }
                    '[' => {
                        if (*strbuf).len() > 0 {
                            rs.push(Token {
                                val: other_get(&strbuf),
                                pos: strpos,
                            });
                            strpos = TokenPos { line: 0, col: 0 };
                            strbuf.clear();
                        };
                        rs.push(Token {
                            val: TokenValue::LMP,
                            pos: TokenPos {
                                line,
                                col,
                            },
                        });
                    }
                    ']' => {
                        if (*strbuf).len() > 0 {
                            rs.push(Token {
                                val: other_get(&strbuf),
                                pos: strpos,
                            });
                            strpos = TokenPos { line: 0, col: 0 };
                            strbuf.clear();
                        };
                        rs.push(Token {
                            val: TokenValue::RMP,
                            pos: TokenPos {
                                line,
                                col,
                            },
                        });
                    }
                    '\'' => {
                        if (*strbuf).len() > 0 {
                            rs.push(Token {
                                val: other_get(&strbuf),
                                pos: strpos,
                            });
                            strpos = TokenPos { line: 0, col: 0 };
                            strbuf.clear();
                        };
                        rs.push(Token {
                            val: TokenValue::QUO,
                            pos: TokenPos {
                                line,
                                col,
                            },
                        });
                    }
                    '\"' => {
                        println!("lexer string");
                        if (*strbuf).len() > 0 {
                            rs.push(Token {
                                val: other_get(&strbuf),
                                pos: strpos,
                            });
                            strpos.line = 0;
                            strpos.col = 0;//= TokenPos { line: 0, col: 0 };
                            strbuf.clear();
                        }
                        mode = Mode::STRING;
                        strpos = TokenPos {
                            line,
                            col,
                        };
                    }
                    _ => {
                        if strpos.line == 0 {
                            strpos = TokenPos { line, col };
                        }
                        strbuf.push(i);
                    }
                };
            }
            Mode::NOTE => {
                match i {
                    '\n' => {
                        mode = Mode::CODE;
                        line += 1;
                        col = 0;
                    }
                    _ => continue,
                };
            }
            Mode::STRING => {
                match i {
                    '\n' => {
                        line += 1;
                        col = 0;
                    }
                    '\"' => {
                        // 判断之前的是不是'\\'如果是，就不退回
                        rs.push(Token {
                            val: TokenValue::STRING(Arc::new(strbuf)),
                            pos: strpos,
                        });
                        strpos = TokenPos {
                            line: 0,
                            col: 0,
                        };
                        strbuf = String::new();
                    }
                    _ => { strbuf.push(i); }
                };
            }
        };
        col += 1;
    }
    rs
}
//}