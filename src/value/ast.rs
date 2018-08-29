use std::sync::Arc;
use value::Value;
use value::_Str;

struct FunctionContext;
struct InterpreterContext;

pub struct CondAst {

}

pub struct MatchAst {

}

/*
pub struct Loop {

}

pub struct For {

}

pub struct While {

}
*/

pub struct DefunAst {

}

pub struct FCallAst {

}

pub enum Ast {
    ///## 字面量
    Nil,
    Bool(bool),
    Char(char),
    Int(i64),
    UInt(u64),
    Float(f64),

    String(_Str),
    Symbol(_Str),

    ///## 控制结构
    Cond(CondAst),
    Match(MatchAst),
    Defun(DefunAst),
    FCall(FCallAst),
}

trait Eval {
    fn eval(&self, ic :Arc<InterpreterContext>) -> Value;
}
