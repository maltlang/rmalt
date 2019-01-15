#[derive(Debug)]
pub enum Token {
    Lp,
    Rp,
    Sym(String),
    Str(String),
    Int(i64),
    UInt(u64),
    Float(f64),
}

fn check_text(raw_src: &str, ln: usize, rn: usize) -> Token {
    let mut src = String::new();
    for i in ln..rn {
        src.push(raw_src.chars().nth(i).unwrap());
    }
    if src.starts_with("\"") && src.ends_with("\"") {
        Token::Str(String::from(src))
    } else {
        let mut dot_size = 0 as usize;
        for (i, c) in src.clone().chars().enumerate() {
            /*if dot_size > 1 {
                return Token::Sym(String::from(src));
            }*/
            if c == '-' || c == '+' {
                if i != 0 {
                    return Token::Sym(String::from(src));
                }
            } else {
                if c == '.' {
                    dot_size += 1;
                } else {
                    if !(c == '_' || (c >= '0' && c <= '9')) {
                        return Token::Sym(String::from(src));
                    }
                }
            }
        }
        if dot_size == 0 {
            if src.chars().nth(0).unwrap() == '-' || src.chars().nth(0).unwrap() == '+' {
                println!("{}", src);
                Token::Int(src.parse().unwrap())
            } else {
                Token::UInt(src.parse().unwrap())
            }
        } else if dot_size == 1 {
            Token::Float(src.parse().unwrap())
        } else {
            Token::Sym(String::from(src))
        }
    }
}

pub fn lexer(src: &str) -> Option<Vec<Token>> {
    let mut in_string = false;
    let mut have_flag = false;
    let mut flag = 0 as usize;
    let mut res: Vec<Token> = Vec::new();
    for (i, c) in src.chars().enumerate() {
        if in_string {
            if c == '"' && src.chars().nth(i-1).unwrap() != '/' {
                res.push(check_text(&src, flag, i+1));
                in_string = false;
                have_flag = false;
            }
        } else {
            if c == '(' {
                if have_flag {
                    res.push(check_text(&src, flag, i));
                    have_flag = false;
                }
                res.push(Token::Lp);
            } else if c == ')' {
                if have_flag {
                    res.push(check_text(&src, flag, i));
                    have_flag = false;
                }
                res.push(Token::Rp);
            } else if c == '"' {
                if have_flag {
                    res.push(check_text(&src, flag, i));
                }
                if i + 1 >= src.chars().count() {
                    return None;
                } else {
                    in_string = true;
                    have_flag = true;
                    flag = i;
                }
            } else if c == ' ' || c == '\0' || c == '\t' || c == '\n' || c == '\r' {
                if have_flag {
                    res.push(check_text(&src, flag, i));
                    have_flag = false;
                }
            } else {
                if !have_flag {
                    have_flag = true;
                    flag = i;
                }
            }
        }
    }
    Some(res)
}