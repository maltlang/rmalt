use std::sync::Arc;
use value::Value;
use value::_Tuple;
use value::ast::Ast;
use core::module;
use core::interpreter::FunctionContext;
use core::interpreter::InterpreterContext;
use std::sync::Weak;

pub struct Function {
    pub modu: Weak<module::Module>,
    pub name: String,
    pub env: Arc<FunctionContext>,
    pub expr: Ast,
}

pub struct Native {
    pub name: String,
    pub fp: fn(Arc<InterpreterContext>, _Tuple) -> Value,
}

pub trait Call {
    fn call(&self, ic: Arc<InterpreterContext>, args: _Tuple) -> Value;
}

