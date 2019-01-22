use lexer::Token;
use runtime::Ast;
use std::rc::Rc;

pub fn parser(_tk_stream: Vec<Token>) -> Ast {
    Ast::None
}

#[derive(Debug, Clone)]
pub struct StringStream {
    size: usize,
    val: Rc<String>,
}

pub type Parser<X, Y> = Box<Fn(X) -> Result<Y, ()>>;
pub type DefaultParser<T> = Parser<StringStream, T>;
pub type MatchParser = DefaultParser<Option<StringStream>>;

impl StringStream {
    fn new(s: String) -> Self {
        StringStream {
            size: 0,
            val: Rc::new(s),
        }
    }
    fn get_head(&self) -> char {
        self.val.chars().nth(self.size).unwrap()
    }
    fn next(&self) -> Option<Self> {
        if self.size + 1 >= self.val.chars().count() {
            None
        } else {
            Some(
                StringStream {
                    size: self.size + 1,
                    val: self.val.clone(),
                })
        }
    }
}

fn empty(ss: StringStream) -> Result<Option<StringStream>, ()> {
    if ss.get_head() == ' ' ||
        ss.get_head() == '\0' ||
        ss.get_head() == '\t' ||
        ss.get_head() == '\n' ||
        ss.get_head() == '\r' {
        Ok(ss.next())
    } else {
        Err(())
    }
}

fn empty_s(ss: StringStream) -> Result<Option<StringStream>, ()> {
    match empty(ss.clone()) {
        Ok(x) => match x {
            Some(x) => empty_s(x),
            None => Ok(None)
        }
        Err(_) => Ok(Some(ss))
    }
}

pub fn char1(c: char) -> MatchParser {
    Box::new(move |x: StringStream| {
        if x.get_head() == c {
            Ok(x.next())
        } else {
            Err(())
        }
    })
}

pub fn symbol1(s: String) -> MatchParser {
    Box::new(move |ss: StringStream| {
        let mut s1 = StringStream::new(s.clone());
        let mut s2 = ss;
        loop {
            if let Some(x) = char1(s1.get_head())(s2.clone())? {
                s2 = x;
                if let Some(x) = s1.next() {
                    s1 = x;
                } else {
                    return Ok(Some(s2));
                }
            } else {
                if let None = s1.next() {
                    return Ok(None);
                } else {
                    return Err(());
                }
            }
        }
    })
}

fn next(r: Result<Option<StringStream>, ()>, n: MatchParser) -> Result<Option<StringStream>, ()> {
    match r {
        Ok(Some(x)) => n(x),
        _ => Err(())
    }
}

fn emm(a: Result<Option<StringStream>, ()>) -> Result<Ast, ()> {
    match a {
        Ok(_) => Ok(Ast::None),
        Err(_) => Err(())
    }
}

pub fn run_parser(s: String) -> Result<Ast, ()> {
    let lp = char1('(');
    let defn = symbol1("defn".to_string());
    let fuck = symbol1("fuck".to_string());
    let rp = char1(')');
    emm(
        next(
            next(
                next(
                    next(
                        lp(StringStream::new(s)), defn), Box::new(empty_s)), fuck), rp))
}