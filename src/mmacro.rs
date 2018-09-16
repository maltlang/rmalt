use std::sync::Arc;
use ast::Ast;
use value::Value;
use core::interpreter::ThreadContext;

pub struct BaseMacro {
    pub name: String,
    pub fp: fn(&Arc<ThreadContext>, Vec<Ast>) -> Value,
}


impl BaseMacro {
    pub fn unfold(&self, ic: &Arc<ThreadContext>, args: Vec<Ast>) -> Value {
        (self.fp)(ic, args)
    }
}
