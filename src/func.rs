use std::sync::Arc;
use value::Value;
use value::MaltResult;
use value::_Tuple;
//use runtime::module;
//use runtime::interpreter::FunctionContext;
//use runtime::interpreter::ThreadContext;
use std::sync::Weak;

/*
//#[derive(Clone)]
pub struct Function {
    pub modu: Weak<module::Module>,
    pub name: Arc<String>,
    pub expr: Arc<Vec<Value>>,
    pub argn: Arc<Vec<String>>,
    pub env: Option<Arc<FunctionContext>>,
}

//impl Copy for Function { }

impl Clone for Function {
    fn clone(&self) -> Self {
        Function {
            modu: self.modu.clone(),
            name: self.name.clone(),
            expr: self.expr.clone(),
            argn: self.argn.clone(),
            env: self.env.clone(),
        }
    }
}
*/

pub struct Native {
    pub name: String,
    pub fp: fn(_Tuple) -> MaltResult,
}

pub trait Call {
    fn call_function(&self, args: _Tuple) -> MaltResult;
}
