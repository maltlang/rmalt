use std::sync::Arc;
use std::sync::Weak;

use value::_Tuple;
use value::Value;
use value::MaltResult;
use runtime::context::ModuleContext;
use runtime::context::FunctionContext;
use runtime::context::ThreadContext;

pub struct Function {
    pub modu: Weak<ModuleContext>,
    pub name: String,
    pub expr: Vec<Value>,
    pub argn: Vec<String>,
    pub env: Option<Arc<FunctionContext>>,
}

pub type MaltNativeInterface = fn(&ThreadContext, _Tuple) -> MaltResult;

pub struct Native {
    pub name: String,
    pub fp: MaltNativeInterface,
}
