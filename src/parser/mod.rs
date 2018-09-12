use ast::Ast;
use std::sync::Arc;
use ast::AstValue;
/*
use ast::TupleAst;
use ast::QuoteAst;
use ast::FCallAst;
*/

pub mod token;
pub mod lexer;

pub fn parser(tf: &[token::Token]) -> Result<Vec<Ast>, usize> {
    if tf.len() == 0 {
        return Ok(vec![]);
    }
    let mut rs: Vec<Ast> = vec![];
    let mut sz: usize = 0;
    loop {
        if sz == tf.len() { break; }
        match parser_once(tf, sz.clone()) {
            Ok((ref x, ref i)) => {
                sz = i.clone();
                rs.push(x.clone());
            }
            Err(us) => return Err(us),
        }
    }
    return Ok(rs);
}

pub fn parser_once(tf: &[token::Token], idx: usize) -> Result<(Ast, usize), usize> {
    match tf.get(idx) {
        Some(x) => match x.val {
            token::TokenValue::INT(ref v) => Ok((Ast { val: AstValue::Int(v.clone()), pos: x.pos.clone() }, idx + 1)),
            token::TokenValue::UINT(ref v) => Ok((Ast { val: AstValue::UInt(v.clone()), pos: x.pos.clone() }, idx + 1)),
            token::TokenValue::FLOAT(ref v) => Ok((Ast { val: AstValue::Float(v.clone()), pos: x.pos.clone() }, idx + 1)),
            token::TokenValue::STRING(ref v) => Ok((Ast { val: AstValue::String(v.clone()), pos: x.pos.clone() }, idx + 1)),
            token::TokenValue::SYMBOL(ref v) => Ok((Ast { val: AstValue::Symbol(v.clone()), pos: x.pos.clone() }, idx + 1)),
            _ => parser_list(tf, idx),
        }
        None => Err(idx),
    }
}

#[inline]
fn parser_list(tf: &[token::Token], idx: usize) -> Result<(Ast, usize), usize> {
    Err(idx)
}
