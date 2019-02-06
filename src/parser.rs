use std::ops::Add;
use std::ops::BitOr;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;

use num::BigInt;
use num::BigRational;
use num::BigUint;

use runtime::Ast;
use runtime::MFile;
use runtime::MFunction;
use runtime::MModule;
use runtime::Tree;
use runtime::Value;

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
    col: usize,
    lin: usize,
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

fn parse_empty(ss: &StrStream) -> ParserOut {
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

fn parse_symbol(ss: &StrStream) -> ParserOut {
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
        + ParserC::new(Box::new(parse_uint));
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
    let f = (ParserC::new(Box::new(parse_int)) | ParserC::new(Box::new(parse_uint)))
        + ParserC::new(parse_char('/'))
        + (ParserC::new(Box::new(parse_int)) | ParserC::new(Box::new(parse_uint)));
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
        None
    }
}

fn parse_char_text(ss: &StrStream) -> ParserOut {
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

fn parse_string_text(ss: &StrStream) -> ParserOut {
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

///..............................................................

pub fn parse_none(ss: &StrStream) -> ParserOut {
    if let Some((_, tail)) = parse_string(":none")(ss) {
        Some((Some(Ast {
            val: Tree::Value(Value::None),
            col: ss.col,
            lin: ss.lin
        }), parse_empty(&tail)?.1))
    } else {
        None
    }
}

pub fn parse_bool(ss: &StrStream) -> ParserOut {
    if let Some((_, tail)) = parse_string(":true")(ss) {
        Some((Some(Ast {
            val: Tree::Value(Value::Bool(true)),
            col: ss.col,
            lin: ss.lin
        }), parse_empty(&tail)?.1))
    } else if let Some((_, tail)) = parse_string(":false")(ss) {
        Some((Some(Ast {
            val: Tree::Value(Value::Bool(false)),
            col: ss.col,
            lin: ss.lin
        }), parse_empty(&tail)?.1))
    } else {
        None
    }
}

pub fn parse_atom(ss: &StrStream) -> ParserOut {
    if let Some(x) = parse_none(ss) {
        Some(x)
    } else if let Some(x) = parse_bool(ss) {
        Some(x)
    } else if let Some(x) = parse_char_text(ss) {
        Some(x)
    } else if let Some(x) = parse_string_text(ss) {
        Some(x)
    } else if let Some(x) = parse_rational(ss) {
        Some(x)
    } else if let Some(x) = parse_int(ss) {
        Some(x)
    } else if let Some(x) = parse_uint(ss) {
        Some(x)
    } else if let Some(x) = parse_symbol(ss) {
        Some(x)
    } else {
        None
    }
}

pub fn parse_open(ss: &StrStream) -> ParserOut {
    let (_, tail) =
        ((ParserC::new(parse_string("open")) + ParserC::new(Box::new(parse_empty))).f)(ss)?;
    let (r, t2) = parse_string_text(&tail)?;
    let t3 = t2
        //.map(parse_not_char('\n'))?.1
        .map(parse_char('\n'))?.1
        .map(Box::new(parse_empty))?.1;
    if let Some(Ast {
        val: (Tree::Symbol(x)),
        col,
        lin,
    }) = r
    {
        Some((
            Some(Ast {
                val: Tree::Open(x),
                col,
                lin,
            }),
            t3,
        ))
    } else {
        None
    }
}

pub fn parse_function(ss: &StrStream) -> ParserOut {
    parse_uint(ss)
}

pub fn parse_module(ss: &StrStream) -> ParserOut {
    // match
    let (_, tail) =
        ((ParserC::new(parse_string("module")) + ParserC::new(Box::new(parse_empty))).f)(ss)?;
    let (name, tail) = parse_symbol(&tail)?;
    let (_, tail) = ((ParserC::new(Box::new(parse_empty))
        + ParserC::new(parse_string("="))
        + ParserC::new(Box::new(parse_empty)))
    .f)(&tail)?;
    let (fv, tail) = ((ParserC::new(parse_char('f')).rpt(0)).f)(&tail)?;

    let (_, tail) = parse_empty(&tail)?;

    // get/box

    let n;
    let mut v: Vec<Arc<MFunction>> = Vec::new();

    if let Some(Ast { val: Tree::Symbol(x), .. }) = name {
        n = x;
    } else {
        return None;
    }

    if let Ast { val: Tree::Asts(ts), .. } = fv? {
        for i in ts.into_iter() {
            if let Ast { val: Tree::Function(x), .. } = i {
                v.push(x);
                //x
            } else {
                // 其实这段根本不会触发
                //return None;
                panic!("parse_module: ???理论上不会触发的错误");
            }
        }
    } else {
        return None;
    }

    Some((
        Some(Ast {
            val: Tree::Module(Arc::new(MModule { name: n, func: v })),
            col: ss.col,
            lin: ss.lin,
        }),
        tail,
    ))
}

pub fn parse_file(ss: &StrStream) -> ParserOut {
    let (a, b) = ((ParserC::new(Box::new(parse_open)).rpt(0)).f)(ss)?;

    if let Ast { val: Tree::Asts(ts), .. } = a? {
        let _ = ts.iter().map(|o| {

        });
    }
    unimplemented!()
}
