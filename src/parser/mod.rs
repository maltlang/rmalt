//use std::sync::Arc;
use value::Value;

pub mod token;
pub mod lexer;

pub fn parser(_src: String) -> Result<(Vec<Value>, usize), (usize, String)> {
    // toDo:
    return Err((0, "".to_string()));
}

fn raw_parser(_tf: &[token::Token], _idx: usize) -> Result<(Vec<Value>, usize), (usize, String)> {
    // tOdO:
    return Err((0, "".to_string()));
}

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
            //TODO:
                Err((0, "".to_string())),
            token::TokenValue::EVL =>
            //todo:
                Err((0, "".to_string())),
            token::TokenValue::LMP =>
                Err((0, "".to_string())),
            token::TokenValue::LP =>
                Err((0, "".to_string())),
            _ => Err((idx, "Invalid expression begins".to_string()))
        }
    } else {
        Err((idx, "Expression not ending".to_string()))
    }
    //return Err((0, "".to_string()));
}