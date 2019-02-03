use std::rc::Rc;
use runtime::Ast;
use runtime::Value;

#[derive(Debug, Clone)]
pub struct StringStream {
    size: usize,
    col: usize,
    lin: usize,
    val: Rc<String>,
}

pub type ReturnT<R> = Result<R, ()>;
pub type Parser<X, Y> = Box<Fn(X) -> ReturnT<Y>>;
pub type DefaultParser<T> = Parser<StringStream, T>;
pub type MatchParser = DefaultParser<Option<StringStream>>;
pub type GetParser = DefaultParser<(Option<StringStream>, Ast)>;

impl StringStream {
    fn new(s: String) -> Self {
        StringStream {
            size: 0,
            col: 1,
            lin: 1,
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
            if self.val.chars().nth(self.size + 1).unwrap() == '\n' {
                Some(
                    StringStream {
                        size: self.size + 1,
                        col: 1,
                        lin: self.lin + 1,
                        val: self.val.clone(),
                    })
            } else {
                Some(
                    StringStream {
                        size: self.size + 1,
                        col: self.col + 1,
                        lin: self.lin,
                        val: self.val.clone(),
                    })
            }
        }
    }

    fn equ(&self, s2: &StringStream) -> Result<Option<StringStream>, ()> {
        if self.get_head() == s2.get_head() {
            let (a, b) = (self.next(), s2.next());
            if a.is_some() && b.is_some() {
                a.unwrap().equ(&b.unwrap())
            } else if b.is_none() {
                Ok(a)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }


    fn and(&self, f: MatchParser) -> Result<Option<StringStream>, ()> {
        let v: Option<StringStream> = f(self.clone())?;
        if v.is_some() {
            Ok(v)
        } else {
            Err(())
        }
    }

    fn or(&self, p1: Result<Option<StringStream>, ()>, p2: Result<Option<StringStream>, ()>) -> Result<Option<StringStream>, ()> {
        if p1.is_ok() {
            p1
        } else if p2.is_ok() {
            p2
        } else {
            Err(())
        }
    }

    fn end(&self, f: MatchParser) -> Result<Option<StringStream>, ()> {
        f(self.clone())
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

fn num(ss: StringStream) -> Result<Option<StringStream>, ()> {
    let v = ss.get_head();
    if v >= '0' && v <= '9' {
        Ok(ss.next())
    } else {
        Err(())
    }
}

fn num_s(ss: StringStream) -> Result<Option<StringStream>, ()> {
    match num(ss.clone()) {
        Ok(x) => match x {
            Some(x) => num_s(x),
            None => Ok(None)
        }
        Err(_) => Ok(Some(ss))
    }
}

fn char1(c: char) -> MatchParser {
    Box::new(move |x: StringStream| {
        if x.get_head() == c {
            Ok(x.next())
        } else {
            Err(())
        }
    })
}

fn symbol1(s: String) -> MatchParser {
    Box::new(move |ss: StringStream| {
        ss.equ(&StringStream::new(s.clone()))
    })
}

// get values

fn numbers(ss: StringStream) -> Result<(Option<StringStream>, Ast), ()> {
    let num= Box::new(num);
    let num_s= Box::new(num_s);
    if let Ok(a) = char1('-')(ss.clone()) {
        //let a: Option<StringStream> = a;
        if a.is_some() {
            let v= a.unwrap().and(num.clone())?.unwrap().and(num_s.clone())?;
            if v.is_none() {
                return Ok((None, Ast {
                    col: ss.col,
                    lin: ss.lin,
                    val: Value::Int(1) //FIXME
                }));
            } else {
                let a = v.unwrap().and(char1('.'))?.unwrap().and(num)?.unwrap().and(num_s)?;
                return Ok((a, Ast {
                    col: ss.col,
                    lin: ss.lin,
                    val: Value::Float(1.1) //FIXME
                }));
            }
        } else {
            return Err(());
        }
    } else {
        let mut v = ss.and(num.clone())?.unwrap().and(num_s.clone())?;
        if v.is_none() {
            return Ok((None, Ast {
                col: ss.col,
                lin: ss.lin,
                val: Value::Int(1) //FIXME
            }));
        } else {
            let a = v.unwrap().and(char1('.'))?.unwrap().and(num)?.unwrap().and(num_s)?;
            return Ok((a, Ast {
                col: ss.col,
                lin: ss.lin,
                val: Value::Float(1.1) //FIXME
            }));
        }

    }
}

// 收尾工作

fn dook(a: Result<Option<StringStream>, ()>) -> Result<(Option<StringStream>, Ast), ()> {
    match a {
        Ok(x) => Ok((x, Ast {lin: 0, col: 0, val: Value::Nil})),
        Err(_) => Err(())
    }
}

pub fn once_parser(s: String) -> Result<(Option<StringStream>, Ast), ()> {
    let _empty = Box::new(empty);
    let _empty_s = Box::new(empty_s);
    let lp = char1('(');
    let defn = symbol1("defn".to_string());
    let fuck = symbol1("fuck".to_string());
    let rp = char1(')');
    let mut ss = StringStream::new(s);
    //numbers(ss)
    dook({
        ss = ss.and(lp)?.unwrap();
        ss.or(ss.and(defn), ss.and(Box::new(num_s)))?.unwrap().end(rp)
    })
        /*
    dook(ss
        .and(lp)?.unwrap()
        .and(defn)?.unwrap()
        .and(empty)?.unwrap()
        .and(empty_s)?.unwrap()
        .and(fuck)?.unwrap()
        .end(rp))
    */
}