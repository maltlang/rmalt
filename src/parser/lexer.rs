use parser::token::TokenValue;
use parser::token::TokenPos;
use parser::token::Token;
use std::sync::Arc;

enum Mode {
    CODE,
    NOTE,
    STRING,
}

fn other_get(_s: &str) -> TokenValue {
    TokenValue::QUO
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
                            strpos = TokenPos { line: 0, col: 0 };
                        };
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