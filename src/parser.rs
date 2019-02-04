use std::rc::Rc;
use std::sync::Arc;
use std::ops::Add;
use std::ops::BitOr;
use crate::runtime::Ast;
use crate::runtime::Value;
use num::BigUint;
use std::str::FromStr;
use num::BigInt;
use num::BigRational;

type ParserOut = Option<(Option<Ast>, StrStream)>;
type Parser = Box<Fn(&StrStream) -> ParserOut>;

struct ParserC {
    f: Parser,
}

impl ParserC {
    fn new(f: Parser) -> Self {
        Self { f }
    }
    fn get(self) -> Parser {
        self.f
    }
    // repeater
    fn rpt(self, s: usize) -> Self {
        if s == 0 {
            Self {
                f: Box::new(move |ss: &StrStream| -> ParserOut {
                    let mut sout = ss.clone();
                    loop {
                        if let Some((_, tail)) = (self.f)(&sout) {
                            sout = tail;
                        } else {
                            return Some((None, sout.clone()));
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
    col: usize,
    lin: usize,
}

impl StrStream {
    fn new(s: &str) -> Self {
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

    fn slice(&self) -> Option<(char, Self)> {
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

    fn map(&self, f: Parser) -> ParserOut {
        f(self)
    }
}

fn parse_char(c: char) -> Parser {
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

fn parse_string(s: &str) -> Parser {
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

fn parse_uint(ss: &StrStream) -> ParserOut {
    let mut ss1 = ss.clone();
    loop {
        if let Some((head, tail)) = ss1.slice() {
            if (head >= '0' && head <= '9')
                || head == '_'
                || head == ','{
                ss1 = tail;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    let mut s = String::new();
    for c in ss.size..ss1.size {
        s.push(ss.val.chars().nth(c).unwrap())
    }
    return Some((Some(Ast {
        val: Value::Uint(BigUint::from_str(&s).unwrap()), //?
        col: ss.col,
        lin: ss.lin
    }), ss1.clone()));
}

fn parse_int(ss: &StrStream) -> ParserOut {
    let f = (ParserC::new(parse_char('+')) | ParserC::new(parse_char('-')))
        + ParserC::new(Box::new(parse_uint));
    if let Some((_, r)) = (f.f)(ss) {
        let mut s = String::new();
        for c in ss.size..r.size {
            s.push(ss.val.chars().nth(c).unwrap())
        }
        return Some((Some(Ast {
            val: Value::Int(BigInt::from_str(&s).unwrap()), //?
            col: ss.col,
            lin: ss.lin
        }), r.clone()));
    } else {
        None
    }
}

fn parse_rational(ss: &StrStream) -> ParserOut {
    let f =
        (ParserC::new(Box::new(parse_int)) | ParserC::new(Box::new(parse_uint)))
        + ParserC::new(parse_char('/'))
        + (ParserC::new(Box::new(parse_int)) | ParserC::new(Box::new(parse_uint)));
    if let Some((_, r)) = (f.f)(ss) {
        let mut s = String::new();
        for c in ss.size..r.size {
            s.push(ss.val.chars().nth(c).unwrap())
        }
        return Some((Some(Ast {
            val: Value::Rational(BigRational::from_str(&s).unwrap()), //?
            col: ss.col,
            lin: ss.lin
        }), r.clone()));
    } else {
        None
    }
}

fn parse_char_text(ss: &StrStream) -> ParserOut {
    let f = (ParserC::new(parse_char('\'')) + ParserC::new(parse_char('\\')))
        | ParserC::new(parse_char('\''));
    if let Some((head, tail)) = (f.f)(ss)?.1.slice() {
        Some((Some(Ast {
            val: Value::Char(head),
            col: ss.col,
            lin: ss.lin
        }), tail))
    } else {
        None
    }
}

fn parse_string_text(ss: &StrStream) -> ParserOut {
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
}

///..............................................................

//#[test]
pub fn test_parse() {
    let f = ParserC::new(Box::new(parse_string_text));
    let r = StrStream::new("\"objk\"").map(f.f);
    println!("out: {:?}", r);
}

//#[test]
pub fn test_parse_c() {
    let r = parse_char('o')(&StrStream::new("opq"));
    println!("out: {:?}", r);
}

//#[test]
pub fn test_parse_str() {
    let r = parse_string("opq")(&StrStream::new("opq"));
    println!("out: {:?}", r);
}
