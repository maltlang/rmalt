use ast::Ast;
use std::sync::Arc;
use ast::TupleAst;
use ast::QuoteAst;

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
            token::TokenValue::INT(x) => Ok((Ast::Int(x), idx + 1)),
            token::TokenValue::UINT(x) => Ok((Ast::UInt(x), idx + 1)),
            token::TokenValue::FLOAT(x) => Ok((Ast::Float(x), idx + 1)),
            token::TokenValue::STRING(ref x) => Ok((Ast::String(x.clone()), idx + 1)),
            token::TokenValue::SYMBOL(ref x) => Ok((Ast::Symbol(x.clone()), idx + 1)),
            _ => parser_ex(tf, idx),
        }
        None => Err(idx - 1),
    }
}

fn parser_ex(tf: &[token::Token], idx: usize) -> Result<(Ast, usize), usize> {
    if let Some(x) = tf.get(idx) {
        if let token::TokenValue::QUO = x.val {
            let oj: (Ast, usize);
            if let Ok(o) = parser_once(tf, idx + 1) {
                oj = o;
            } else {
                return Err(idx);
            }
            return Ok((Ast::Quote(
                Arc::from(QuoteAst {
                    expr: oj.0
                })), oj.1));
        }
    } else {
        return Err(idx - 1);
    }
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
            return Ok((Ast::Tuple(Arc::new(TupleAst { tuple: arr })), sz));
        }
    } else {
        return Err(idx - 1);
    }
    Err(idx)
}