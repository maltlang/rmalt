use std::ops::Add;
use std::ops::BitOr;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;

use num::BigInt;
use num::BigRational;
use num::BigUint;

use runtime::Ast;
use runtime::Tree;
use runtime::Value;

pub type ParserOut = Option<(Option<Ast>, StrStream)>;
pub type Parser = Box<Fn(&StrStream) -> ParserOut>;

pub struct ParserC {
    pub f: Parser,
}

impl ParserC {
    pub fn new(f: Parser) -> Self {
        Self { f }
    }

    pub fn new_f(f: fn(&StrStream) -> ParserOut) -> Self {
        Self { f: Box::new(f) }
    }

    pub fn get(self) -> Parser {
        self.f
    }
    // repeater
    pub fn rpt(self, s: usize) -> Self {
        if s == 0 {
            Self {
                f: Box::new(move |ss: &StrStream| -> ParserOut {
                    let mut sout = ss.clone();
                    let mut r: Vec<Ast> = Vec::new();
                    /*Ast {
                        val: Tree::Asts(),
                        col: ss.col,
                        lin: ss.lin,
                    };*/
                    loop {
                        if let Some((val, tail)) = (self.f)(&sout) {
                            sout = tail;
                            if let Some(x) = val {
                                r.push(x);
                            }
                        } else {
                            return Some((
                                Some(Ast {
                                    val: Tree::Asts(r),
                                    col: ss.col,
                                    lin: ss.lin,
                                }),
                                sout.clone(),
                            ));
                        }
                    }
                }),
            }
        } else {
            Self {
                f: Box::new(move |ss: &StrStream| -> ParserOut {
                    let mut r = ss.clone();
                    for _ in 0..s {
                        if let Some((_, tail)) = (self.f)(&r) {
                            r = tail;
                        } else {
                            return None;
                        }
                    }
                    Some((None, r))
                }),
            }
        }
    }
}

impl Add for ParserC {
    type Output = ParserC;

    fn add(self, o: Self) -> Self::Output {
        Self {
            f: Box::new(move |ss: &StrStream| -> ParserOut {
                let (_, r) = (self.f)(ss)?;
                (o.f)(&r)
            }),
        }
    }
}

impl BitOr for ParserC {
    type Output = ParserC;

    fn bitor(self, o: Self) -> Self::Output {
        Self {
            f: Box::new(move |ss: &StrStream| -> ParserOut {
                if let Some(r) = (self.f)(ss) {
                    return Some(r);
                }
                (o.f)(ss)
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StrStream {
    size: usize,
    val: Rc<String>,
    pub col: usize,
    pub lin: usize,
}

impl StrStream {
    pub fn new(s: &str) -> Self {
        StrStream {
            size: 0,
            col: 1,
            lin: 1,
            val: Rc::new(String::from(s)),
        }
    }

    fn get_head(&self) -> char {
        self.val.chars().nth(self.size).unwrap()
    }

    /*
    fn is_end(&self) -> bool {
        self.size == self.val.chars().count() - 1
    }
    */

    fn safe_get_head(&self) -> Option<char> {
        self.val.chars().nth(self.size)
    }

    pub fn slice(&self) -> Option<(char, Self)> {
        if self.size == self.val.chars().count() {
            None
        } else {
            Some((
                self.get_head(),
                if self.get_head() == '\n' {
                    Self {
                        size: self.size + 1,
                        val: self.val.clone(),
                        col: 1,
                        lin: self.lin + 1,
                    }
                } else {
                    Self {
                        size: self.size + 1,
                        val: self.val.clone(),
                        col: self.col + 1,
                        lin: self.lin,
                    }
                },
            ))
        }
    }

    pub fn map(&self, f: Parser) -> ParserOut {
        f(self)
    }
}

pub fn parse_empty(ss: &StrStream) -> ParserOut {
    let mut ss = ss.clone();
    loop {
        if let Some((head, tail)) = ss.slice() {
            if head == ' ' || head == '\r' || head == '\t' || head == '\n' {
                ss = tail;
            } else {
                return Some((None, ss));
            }
        } else {
            return Some((None, ss));
        }
    }
}

pub fn parse_char(c: char) -> Parser {
    Box::new(move |ss| {
        if let Some((head, tail)) = ss.slice() {
            if head == c {
                Some((None, tail))
            } else {
                None
            }
        } else {
            None
        }
    })
}

fn parse_not_char(c: char) -> Parser {
    Box::new(move |ss| {
        if let Some((head, tail)) = ss.slice() {
            if head == c {
                None
            } else {
                Some((None, tail))
            }
        } else {
            None
        }
    })
}

pub fn parse_string(s: &str) -> Parser {
    parse_string_r(StrStream::new(s))
}

fn parse_string_r(sst: StrStream) -> Parser {
    Box::new(move |ss: &StrStream| -> ParserOut {
        fn rf(sst: &StrStream, ss: &StrStream) -> ParserOut {
            if let Some((head, tail)) = sst.slice() {
                rf(&tail, &parse_char(head)(ss)?.1)
            } else if let None = sst.slice() {
                Some((None, ss.clone()))
            } else {
                None
            }
        }
        rf(&sst, ss)
    })
}

/* 直觉上效率更高的算法，最开始写的是这个
        if let (
            Some((cead, cail)),
            Some((head, tail))) =
        (sst.slice(), ss.slice()) {
            if head == cead {
                rf(&cail, &tail)
            } else {
                None
            }
        } else if let (None, _) = (sst.slice(), ss.slice()) {
            Some((None, ss.clone()))
        } else {
            None
        }
*/

fn parse_sym_raw(end_char: char) -> Parser {
    Box::new(move |ss: &StrStream| -> ParserOut {
        fn func(ss: &StrStream, end_char: char) -> ParserOut {
            if let Some((head, tail)) = ss.slice() {
                if head == end_char {
                    Some((None, ss.clone()))
                } else {
                    func(&tail, end_char)
                }
            } else {
                None
            }
        }
        func(ss, end_char)
    })
}

fn parse_sym(end_char: char) -> Parser {
    Box::new(move |ss: &StrStream| -> ParserOut {
        let (_, ss1) = parse_sym_raw(end_char)(ss)?;
        let mut s = String::new();
        for c in ss.size..ss1.size {
            s.push(ss.val.chars().nth(c).unwrap())
        }
        return Some((
            Some(Ast {
                val: Tree::Symbol(Arc::from(s)), // 可能会加上转义函数
                col: ss.col,
                lin: ss.lin,
            }),
            ss1.clone(),
        ));
    })
}

///..............................................................

pub fn parse_symbol(ss: &StrStream) -> ParserOut {
    fn func(ss: &StrStream) -> ParserOut {
        if let Some((head, tail)) = ss.slice() {
            if head == ' ' || head == '\r' || head == '\t' || head == '\n' {
                Some((None, ss.clone()))
            } else {
                func(&tail)
            }
        } else {
            None
        }
    }
    let (_, ss1) = func(ss)?;
    let mut s = String::new();
    for c in ss.size..ss1.size {
        s.push(ss.val.chars().nth(c).unwrap())
    }
    return Some((
        Some(Ast {
            val: Tree::Symbol(Arc::from(s)), // 可能会加上转义函数
            col: ss.col,
            lin: ss.lin,
        }),
        parse_empty(&ss1)?.1,
    ));
}

pub fn parse_uint(ss: &StrStream) -> ParserOut {
    let mut ss1 = ss.clone();
    loop {
        if let Some((head, tail)) = ss1.slice() {
            if (head >= '0' && head <= '9') || head == '_' || head == ',' {
                ss1 = tail;
            } else {
                return None;
            }
        } else {
            break;
        }
    }
    let mut s = String::new();
    for c in ss.size..ss1.size {
        s.push(ss.val.chars().nth(c).unwrap())
    }
    return Some((
        Some(Ast {
            val: Tree::Value(Value::Uint(BigUint::from_str(&s).unwrap())), //?
            col: ss.col,
            lin: ss.lin,
        }),
        parse_empty(&ss1)?.1,
    ));
}

pub fn parse_int(ss: &StrStream) -> ParserOut {
    let f = (ParserC::new(parse_char('+')) | ParserC::new(parse_char('-')))
        + ParserC::new_f(parse_uint);
    if let Some((_, r)) = (f.f)(ss) {
        let mut s = String::new();
        for c in ss.size..r.size {
            s.push(ss.val.chars().nth(c).unwrap())
        }
        return Some((
            Some(Ast {
                val: Tree::Value(Value::Int(BigInt::from_str(&s).unwrap())), //?
                col: ss.col,
                lin: ss.lin,
            }),
            parse_empty(&r)?.1,
        ));
    } else {
        None
    }
}

pub fn parse_rational(ss: &StrStream) -> ParserOut {
    let f = (ParserC::new_f(parse_int) | ParserC::new_f(parse_uint)
        + ParserC::new(parse_char('/'))
        + (ParserC::new_f(parse_int) | ParserC::new_f(parse_uint)));
    if let Some((_, r)) = (f.f)(ss) {
        let mut s = String::new();
        for c in ss.size..r.size {
            s.push(ss.val.chars().nth(c).unwrap())
        }
        return Some((
            Some(Ast {
                val: Tree::Value(Value::Rational(BigRational::from_str(&s).unwrap())), //?
                col: ss.col,
                lin: ss.lin,
            }),
            parse_empty(&r)?.1,
        ));
    } else {
        return None;
    }
}

pub fn parse_char_text(ss: &StrStream) -> ParserOut {
    let f = (ParserC::new(parse_char('\'')) + ParserC::new(parse_char('\\')))
        | ParserC::new(parse_char('\''));
    if let Some((head, tail)) = (f.f)(ss)?.1.slice() {
        Some((
            Some(Ast {
                val: Tree::Value(Value::Char(head)),
                col: ss.col,
                lin: ss.lin,
            }),
            tail,
        ))
    } else {
        None
    }
}

pub fn parse_string_text(ss: &StrStream) -> ParserOut {
    let f = ParserC::new(parse_char('"')) + ParserC::new(parse_sym('"'));
    let (r, tail) = (f.f)(ss)?;
    let (_, tail) = parse_empty(&tail.slice()?.1)?;
    Some((r, tail))
    /*
    let mut ss1: StrStream = parse_char('"')(ss)?.1;
    let mut rs = String::new();
    loop {
        // if let Some((_, r)) = parse_char('\\')(&ss); 算了我不做转义处理了，这个交给内置函数吧
        if let Some((_, r)) = parse_not_char('"')(&ss1) {
            rs.push(ss1.get_head());
            ss1 = r;
        } else {
            ss1 = ss1.slice()?.1;
            break;
        }
    }
    return Some((Some(Ast {
        val: Value::CharString(Arc::from(rs)), //?
        col: ss.col,
        lin: ss.lin
    }), ss1.clone()));
    */
}
