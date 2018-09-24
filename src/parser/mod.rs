//use std::sync::Arc;
use value::Value;
use value::Handle;
use parser::lexer::lexer;

pub mod token;
pub mod lexer;

pub fn parser(src: &String) -> Result<Vec<Value>, (usize, String)> {
    let tf = lexer(src);
    return raw_parser(tf.as_ref());
}

pub fn raw_parser(tf: &[token::Token]) -> Result<Vec<Value>, (usize, String)> {
    if tf.len() == 0 {
        Ok(vec![])
    } else {
        let mut sz = 0;
        let mut r: Vec<Value> = vec![];
        loop {
            if sz > tf.len() { break; }
            match parser_once(tf, sz) {
                Ok((val, nidx)) => {
                    sz = nidx;
                    r.push(val);
                }
                Err(e) => return Err(e),
            }
        }
        Ok(r)
    }
}

#[inline]
fn parser_once(tf: &[token::Token], idx: usize) -> Result<(Value, usize), (usize, String)> {
    if let Some(ref x) = tf.get(idx) {
        match x.val {
            token::TokenValue::INT(ref y) =>
                Ok((
                    Value::Int(y.clone()),
                    idx + 1)),
            token::TokenValue::UINT(ref y) =>
                Ok((
                    Value::UInt(y.clone()),
                    idx + 1)),
            token::TokenValue::FLOAT(ref y) =>
                Ok((
                    Value::Float(y.clone()),
                    idx + 1)),
            token::TokenValue::STRING(ref y) =>
                Ok((
                    Value::String(y.clone()),
                    idx + 1)),
            token::TokenValue::SYMBOL(ref y) =>
                Ok((
                    Value::Symbol(y.clone()),
                    idx + 1)),
            token::TokenValue::QUO =>
                match parser_once(tf, idx + 1) {
                    Ok((val, nidx)) => Ok(
                        (
                            Value::Tuple(Handle::from(vec![Value::Symbol(Handle::from("quote".to_string())), val])),
                            nidx
                        )),
                    Err(t) => Err(t),
                }
            token::TokenValue::EVL =>
                match parser_once(tf, idx + 1) {
                    Ok((val, nidx)) => Ok(
                        (
                            Value::Tuple(Handle::from(vec![Value::Symbol(Handle::from("eval".to_string())), val])),
                            nidx
                        )),
                    Err(t) => Err(t),
                }
            token::TokenValue::LMP =>
                Err((0, "".to_string())),
            token::TokenValue::LP =>
                Err((0, "".to_string())),
            _ => Err((idx, "Invalid expression begins".to_string()))
        }
    } else {
        Err((idx, "Expression not ending".to_string()))
    }
}

/*
#[inline]
pub fn parser_list(tf: &[token::Token], idx: usize) -> Result<(Value, usize), (usize, String)> {
    Err((idx, "还没写完啊，慌什么慌啊".to_string()))
}

#[inline]
pub fn parser_tuple(tf: &[token::Token], idx: usize) -> Result<(Value, usize), (usize, String)> {
    Err((idx, "还没写完啊，慌什么慌啊".to_string()))
}
*/