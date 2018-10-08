use value::Value;
use value::Handle;

pub mod token;
pub mod lexer;

/// ## Parser
/*
/// parser只是把lexer与raw_parser粘起来
pub fn parser(src: &String) -> Result<Vec<Value>, (usize, String)> {
    let tf = lexer(src);
    return raw_parser(tf.as_ref());
}
*/

pub fn parser(tf: &[token::Token]) -> Result<Vec<Value>, (usize, String)> {
    if tf.len() == 0 {
        Ok(vec![])
    } else {
        let mut sz = 0;
        let mut r: Vec<Value> = vec![];
        loop {
            if sz >= tf.len() {
                break;
            }
            let (val, nidx) = parser_once(tf, sz)?;
            sz = nidx;
            r.push(val);
        }
        Ok(r)
    }
}

#[inline]
fn parser_once(tf: &[token::Token], idx: usize) -> Result<(Value, usize), (usize, String)> {
    if let Some(ref x) = tf.get(idx) {
        match x.val {
            token::TokenValue::INT(ref y) => Ok((Value::Int(y.clone()), idx + 1)),
            token::TokenValue::UINT(ref y) => Ok((Value::UInt(y.clone()), idx + 1)),
            token::TokenValue::FLOAT(ref y) => Ok((Value::Float(y.clone()), idx + 1)),
            token::TokenValue::STRING(ref y) => Ok((Value::String(y.clone()), idx + 1)),
            token::TokenValue::SYMBOL(ref y) => Ok((Value::Symbol(y.clone()), idx + 1)),
            token::TokenValue::QUO => {
                let (val, nidx) = parser_once(tf, idx + 1)?;
                Ok((Value::Tuple(Handle::from(vec![Value::Symbol(Handle::from("quote".to_string())), val])),
                    nidx))
            }
            token::TokenValue::EVL => {
                let (val, nidx) = parser_once(tf, idx + 1)?;
                Ok((Value::Tuple(Handle::from(vec![Value::Symbol(Handle::from("eval".to_string())), val])),
                    nidx))
            }
            token::TokenValue::LMP => Ok(parser_tuple(tf, idx + 1)?),
            token::TokenValue::LP => Ok(parser_list(tf, idx + 1)?),
            _ => Err((idx, "Invalid expression begins".to_string()))
        }
    } else {
        Err((idx - 1, "Expression not ending".to_string()))
    }
}

// 这俩函数其实就差一个字

#[inline]
pub fn parser_list(tf: &[token::Token], idx: usize) -> Result<(Value, usize), (usize, String)> {
    let mut sz = idx;
    let mut r: Vec<Value> = vec![];
    loop {
        if sz >= tf.len() {
            return Err((idx - 1, "Expression not ending".to_string()));
        }
        if let token::TokenValue::RP = tf[sz].val {
            return Ok((
                Value::Tuple(Handle::from(r)),
                sz + 1
            ));
        }
        let (val, nidx) = parser_once(tf, sz)?;
        sz = nidx;
        r.push(val);
    }
}

#[inline]
pub fn parser_tuple(tf: &[token::Token], idx: usize) -> Result<(Value, usize), (usize, String)> {
    let mut sz = idx;
    let mut r: Vec<Value> = vec![];
    loop {
        if sz >= tf.len() {
            return Err((idx - 1, "Expression not ending".to_string()));
        }
        if let token::TokenValue::RMP = tf[sz].val {
            return Ok((
                Value::Tuple(Handle::from(r)),
                sz + 1
            ));
        }
        let (val, nidx) = parser_once(tf, sz)?;
        sz = nidx;
        r.push(val);
    }
}
