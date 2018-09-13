
pub struct Function {
    pub modu: Weak<module::Module>,
    pub name: String,
    pub expr: Ast,
    pub env: Option<Arc<FunctionContext>>,
}

pub struct Native {
    pub name: String,
    pub fp: fn(Arc<ThreadContext>, _Tuple) -> Value,
}

pub trait Call {
    fn call(&self, ic: Arc<ThreadContext>, args: _Tuple) -> Value;
}
