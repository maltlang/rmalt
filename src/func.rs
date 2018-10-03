use std::sync::Arc;
use std::sync::Weak;

use value::_Tuple;
use value::Value;
use value::MaltResult;
use runtime::context::ModuleContext;
use runtime::context::FunctionContext;
use runtime::context::ThreadContext;

//#[derive(Clone)]
pub struct Function {
    pub modu: Weak<ModuleContext>,
    pub name: String,
    pub expr: Vec<Value>,
    pub argn: Vec<String>,
    pub env: Option<Arc<FunctionContext>>,
}

/*
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
    pub fp: fn(&ThreadContext, _Tuple) -> MaltResult,
}
