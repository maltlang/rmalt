use ast::Ast;
use std::sync::Arc;
use ast::TupleAst;
use ast::QuoteAst;
use ast::FCallAst;
use ast::AstValue;

pub mod token;
pub mod lexer;

//## BNF Expression
// expr ::= atom | String | Symbol | Tuple |
// defun ::= '(' "fun" Symbol '('  ')' ')'
// tuple ::= '[' [expr]* ']' ;
// atom ::= UInt | Int | Float
// default : Symbol | String | UInt | Int | Float


pub fn parser_once(tf: &[token::Token], idx: usize) -> Result<(Ast, usize), usize> {
    match tf.get(idx) {
        Some(x) => match x.val {
            token::TokenValue::INT(ref v) => Ok((Ast { val: AstValue::Int(v.clone()), pos: x.pos.clone() }, idx + 1)),
            token::TokenValue::UINT(ref v) => Ok((Ast { val: AstValue::UInt(v.clone()), pos: x.pos.clone() }, idx + 1)),
            token::TokenValue::FLOAT(ref v) => Ok((Ast { val: AstValue::Float(v.clone()), pos: x.pos.clone() }, idx + 1)),
            token::TokenValue::STRING(ref v) => Ok((Ast { val: AstValue::String(v.clone()), pos: x.pos.clone() }, idx + 1)),
            token::TokenValue::SYMBOL(ref v) => Ok((Ast { val: AstValue::Symbol(v.clone()), pos: x.pos.clone() }, idx + 1)),
            _ => parser_ex(tf, idx),
        }
        None => Err(idx - 1),
    }
}

fn parser_ex(tf: &[token::Token], idx: usize) -> Result<(Ast, usize), usize> {
    // quote
    if let Some(x) = tf.get(idx) {
        if let token::TokenValue::QUO = x.val {
            let oj: (Ast, usize);
            if let Ok(o) = parser_once(tf, idx + 1) {
                oj = o;
            } else {
                return Err(idx);
            }
            return Ok((Ast {
                val: AstValue::Quote(
                    Arc::from(QuoteAst {
                        expr: oj.0
                    })),
                pos: x.pos.clone(),
            }, oj.1));
        }
    } else {
        return Err(idx - 1);
    }
    // tuple
    if let Some(x) = tf.get(idx) {
        if let token::TokenValue::LMP = x.val {
            let mut arr: Vec<Ast> = vec![];
            let mut sz: usize = idx + 1;
            loop {
                if let token::TokenValue::RMP = tf[sz].val { break; }
                match parser_once(tf, sz) {
                    Ok((o, i)) => {
                        arr.push(o);
                        sz = i;
                    }
                    Err(x) => return Err(x),
                }
            }
            return Ok((Ast {
                val: AstValue::Tuple(Arc::new(TupleAst { tuple: arr })),
                pos: x.pos.clone(),
            }, sz + 1));
        }
    } else {
        return Err(idx - 1);
    }
    // list...
    if let Ok(x) = parser_list_struct(tf, idx) {
        return Err(idx);
    } else {
        return Err(idx);
    }
}

fn parser_list_struct(tf: &[token::Token], idx: usize) -> Result<(Ast, usize), usize> {
    if let Some(x) = tf.get(idx) {
        // other
        // fcall
        if let token::TokenValue::LP = x.val {
            let mut arr: Vec<Ast> = vec![];
            let mut sz: usize = idx + 1;
            loop {
                if let token::TokenValue::RP = tf[sz].val { break; }
                match parser_once(tf, sz) {
                    Ok((o, i)) => {
                        arr.push(o);
                        sz = i;
                    }
                    Err(x) => return Err(x),
                }
            }
            return Ok((Ast {
                val: AstValue::FCall(Arc::new(FCallAst { list: arr })),
                pos: x.pos.clone(),
            }, sz + 1));
        }
        return Err(idx);
    } else {
        return Err(idx - 1);
    }
}