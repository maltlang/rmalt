pub mod eval;
pub mod codegen;
pub mod module;
pub mod interpreter;

use std::sync::Arc;
use value::Value;
use value::_Tuple;
use func::Call;
use func::Function;
use func::Native;
use core::interpreter::ThreadContext;


impl Call for Function {
    fn call(&self, _ic: Arc<ThreadContext>, _args: _Tuple) -> Value {
        Value::Nil
    }
}

impl Call for Native {
    fn call(&self, ic: Arc<ThreadContext>, args: _Tuple) -> Value { (self.fp)(ic, args) }
}