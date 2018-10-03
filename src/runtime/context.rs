use std::sync::Arc;
use std::sync::RwLock;
use std::cell::RefCell;
use std::collections::HashMap;

use value::Value;
use value::_Str;
use value::_Function;

pub struct ModuleContext {
    pub path: String,
    pub expr: Vec<Value>, // 其实可以不用这个字段的，但我要codegen
    pub vtab: HashMap<String, Value>, // env (var table)
}

pub struct FunctionContext {
    pub fun: _Function,
    pub vtab: RefCell<HashMap<String, Value>>, // env (var table)
}

pub type CommonModuleContext = RefCell<HashMap<String, Arc<ModuleContext>>>;

pub struct ThreadContext {
    pub commonmod: Arc<RwLock<CommonModuleContext>>,
    pub using_mod: Arc<ModuleContext>,
    pub framestack: RefCell<Vec<RefCell<Option<Arc<FunctionContext>>>>>,
    pub frame_size: RefCell<usize>,
}

impl ThreadContext {
    pub fn load_symbol(&self, _sym: _Str) -> Option<Value> {
        None
    }
}