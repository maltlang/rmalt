use ast::Ast;
use std::sync::Arc;
use ast::AstValue;
use ast;
use std::prelude::v1::Vec;
use ast::ListAst;
/*
use ast::TupleAst;
use ast::QuoteAst;
use ast::FCallAst;
*/

pub mod token;
pub mod lexer;

//TODO:全部推倒重来！

pub fn parser(tf: &[token::Token]) -> Result<Vec<Ast>, usize> {
    if tf.len() == 0 {
        return Ok(vec![]);
    }
    let mut rs: Vec<Ast> = vec![];
    let mut sz: usize = 0;
    loop {
        if sz >= tf.len() { break; }
        match parser_once(tf, sz) {
            Ok((x, i)) => {
                sz = i + 1;
                rs.push(x);
            }
            Err(us) => return Err(us),
        }
    }
    return Ok(rs);
}

pub fn parser_once(tf: &[token::Token], idx: usize) -> Result<(Ast, usize), usize> {
    match tf.get(idx) {
        Some(x) => match x.val {
            token::TokenValue::INT(ref v) => Ok((Ast { val: AstValue::Int(v.clone()), pos: x.pos.clone() }, idx)),
            token::TokenValue::UINT(ref v) => Ok((Ast { val: AstValue::UInt(v.clone()), pos: x.pos.clone() }, idx)),
            token::TokenValue::FLOAT(ref v) => Ok((Ast { val: AstValue::Float(v.clone()), pos: x.pos.clone() }, idx)),
            token::TokenValue::STRING(ref v) => Ok((Ast { val: AstValue::String(v.clone()), pos: x.pos.clone() }, idx)),
            token::TokenValue::SYMBOL(ref v) => Ok((Ast { val: AstValue::Symbol(v.clone()), pos: x.pos.clone() }, idx)),
            _ => parser_list(tf, idx),
        }
        None => Err(idx),
    }
}

#[inline]
fn parser_list(tf: &[token::Token], idx: usize) -> Result<(Ast, usize), usize> {
    // 因为 parser once有检测第一个item的match，所以这里不用检测
    match tf[idx].val {
        token::TokenValue::LP => {}
        _ => {
            return Err(idx);
        }
    }
    let mut sz: usize = idx + 1;
    let mut list: Vec<ast::Ast> = vec![];
    loop {
        if let Some(x) = tf.get(sz) {
            if let token::TokenValue::RP = x.val {
                //break;
                return Ok((
                    Ast {
                        val: AstValue::List(Arc::from(
                            ListAst {
                                list: list,
                            })),
                        pos: tf[idx].pos.clone(),
                    },
                    sz));
            }
            match parser_once(tf, sz) {
                Ok((o, i)) => {
                    list.push(o);
                    sz = i + 1;
                }
                Err(e) => return Err(e),
            }
        } else {
            return Err(idx - 1);
        }
    }
}

