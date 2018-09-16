use std::sync::Arc;
use value::Value;
use value::_Str;
use core::interpreter::InterpreterContext;
use parser::token::TokenPos;
use core::interpreter::ThreadContext;


pub struct ListAst {
    pub list: Vec<Ast>,
}

pub enum AstValue {
    ///## 字面量
    //Nil,
    Bool(bool),
    //Char(char),
    Int(i64),
    UInt(u64),
    Float(f64),

    String(_Str),
    Symbol(_Str),

    List(Arc<ListAst>),
}

pub struct Ast {
    pub val: AstValue,
    pub pos: TokenPos,
}

impl Clone for AstValue {
    fn clone(&self) -> AstValue {
        match self {
            //AstValue::Nil => AstValue::Nil,
            AstValue::Bool(ref x) => AstValue::Bool(x.clone()),
            //AstValue::Char(ref x) => AstValue::Char(x.clone()),
            AstValue::Int(ref x) => AstValue::Int(x.clone()),
            AstValue::UInt(ref x) => AstValue::UInt(x.clone()),
            AstValue::Float(ref x) => AstValue::Float(x.clone()),
            AstValue::String(ref x) => AstValue::String(x.clone()),
            AstValue::Symbol(ref x) => AstValue::Symbol(x.clone()),
            AstValue::List(ref x) => AstValue::List(x.clone()),
            /*
            AstValue::Tuple(ref x) => AstValue::Tuple(x.clone()),
            AstValue::Quote(ref x) => AstValue::Quote(x.clone()),
            AstValue::Cond(ref x) => AstValue::Cond(x.clone()),
            AstValue::Match(ref x) => AstValue::Match(x.clone()),
            AstValue::Defun(ref x) => AstValue::Defun(x.clone()),
            AstValue::FCall(ref x) => AstValue::FCall(x.clone()),
            */
        }
    }
}

impl Clone for Ast {
    fn clone(&self) -> Ast {
        Ast {
            val: self.val.clone(),
            pos: self.pos.clone(),
        }
    }
}


impl ToString for ListAst {
    fn to_string(&self) -> String {
        let mut rs = String::from("(");
        for i in &self.list {
            rs += &i.to_string();
            rs += ", ";
        }
        rs.push(')');
        rs
    }
}


impl ToString for Ast {
    fn to_string(&self) -> String {
        match self.val {
            //AstValue::Nil => "nil".to_string(),
            AstValue::Bool(ref x) => match x {
                true => "true".to_string(),
                false => "false".to_string(),
            },
            //AstValue::Char(ref x) => x.to_string(),
            AstValue::Int(ref x) => x.to_string(),
            AstValue::UInt(ref x) => x.to_string(),
            AstValue::Float(ref x) => x.to_string(),
            AstValue::String(ref x) => x.to_string(),
            AstValue::Symbol(ref x) => x.to_string(),
            AstValue::List(ref x) => x.to_string(),
            /*
            AstValue::Quote(ref x) => "quote: ".to_string() + &x.expr.to_string(),
            AstValue::Tuple(ref x) => x.to_string(),
            AstValue::Cond(x),
            AstValue::Match(x),
            AstValue::Defun(x),
            AstValue::FCall(x),*/
            _ => "还没写完".to_string(),
        }
    }
}

trait CodeGen {
    fn code_gen_to_rust(&self) -> String;
    //fn code_gen_to_dump(&self) -> Vec<u8>;
}
