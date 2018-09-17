use std::sync::Arc;
use value::Value;

pub mod token;
pub mod lexer;

pub fn parser(src: String) -> Result<(Vec<Value>, usize), (usize, String)> {
    // toDo:
    return Err((0, "".to_string()));
}

fn raw_parser(tf: &[token::Token], idx: usize) -> Result<(Vec<Value>, usize), (usize, String)> {
    // tOdO:
    return Err((0, "".to_string()));
}

fn parser_once(tf: &[token::Token], idx: usize) -> Result<(Value, usize), (usize, String)> {
    if let Some(ref x) = tf.get(idx) {
        match x.val {
            token::TokenValue::INT(ref y) => {
                return Ok((
                    Value::Int(y.clone()),
                    idx + 1));
            }
            token::TokenValue::UINT(ref y) => {
                return Ok((
                    Value::UInt(y.clone()),
                    idx + 1));
            }
            token::TokenValue::FLOAT(ref y) => {
                return Ok((
                    Value::Float(y.clone()),
                    idx + 1));
            }
            token::TokenValue::STRING(ref y) => {
                return Ok((
                    Value::String(y.clone()),
                    idx + 1));
            }
            token::TokenValue::SYMBOL(ref y) => {
                return Ok((
                    Value::Symbol(y.clone()),
                    idx + 1));
            }
            token::TokenValue::QUO => {
                //TODO:
            }
            token::TokenValue::EVL => {
                //todo:
            }
            token::TokenValue::LMP => {}
            token::TokenValue::LP => {}
            _ => {
                return Err((idx, "Invalid expression begins".to_string()));
            }
        }
    } else {
        return Err((idx, "Expression not ending".to_string()));
    }
    return Err((0, "".to_string()));
}