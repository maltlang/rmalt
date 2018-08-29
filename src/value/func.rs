use std::sync::Arc;
use value::Value;
use value::_Tuple;
use value::ast::Ast;


struct FunctionContext;
struct InterpreterContext;

pub struct Function {
    pub name    :String,
    env     :Arc<FunctionContext>,
    expr    :Ast,
}

pub struct Native {
    pub name    :String,
    fp      :fn(Arc<InterpreterContext>, _Tuple)->Value,
}

pub trait Call {
    fn call(&self, ic: Arc<InterpreterContext>, args: _Tuple)->Value;
}



/*
to core/eval.rs

impl Call for Function {
    fn call(&self, ic: Arc<InterpreterContext>, args: _Tuple)->Value {
        Value::Nil
    }
}

impl Call for Native {
    fn call(&self, ic: Arc<InterpreterContext>, args: _Tuple)->Value {
        (self.fp)(ic, args)
    }
}
*/