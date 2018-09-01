pub mod eval;
pub mod codegen;
pub mod module;
pub mod interpreter;

use std::sync::Arc;
use value::Value;
use value::_Tuple;
use value::func::Call;
use value::func::Function;
use value::func::Native;
use core::interpreter::InterpreterContext;


impl Call for Function {
    fn call(&self, _ic: Arc<InterpreterContext>, _args: _Tuple) -> Value {
        Value::Nil
    }
}

impl Call for Native {
    fn call(&self, ic: Arc<InterpreterContext>, args: _Tuple) -> Value {
        (self.fp)(ic, args)
    }
}