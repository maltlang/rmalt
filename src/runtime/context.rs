use std::sync::Arc;
use std::cell::RefCell;
use std::collections::HashMap;

use regex::Regex;

use value::Value;
use value::Handle;
use value::_Str;
use value::_Function;
use runtime::system::system_module;

pub struct ModuleContext {
    pub path: String,
    // 其实可以不用这个字段的，但我要codegen
    pub expr: Vec<Value>,
    pub vtab: RefCell<HashMap<String, Value>>, // env (var table)
}

pub struct FunctionContext {
    pub fun: _Function,
    pub vtab: RefCell<HashMap<String, Value>>, // env (var table)
}

pub struct ThreadContext {
    pub using_mod: RefCell<Arc<ModuleContext>>,
    pub framestack: RefCell<Vec<RefCell<Option<Arc<FunctionContext>>>>>,
    pub frame_size: RefCell<usize>,
}

/// symbol space
/// - global
///     - function
///         - locals
///         - env

/// load_symbol logic
/// if symbol => symbol
/// then locals -> env -> globals
/// then Prelude-module
/// if symbol => module | symbol
/// then module-space(global)

impl ModuleContext {
    pub fn load_symbol(&self, sym: _Str) -> Option<Value> {
        if let Some(ref x) = self.vtab.borrow().get(sym.as_ref()) {
            return Some((*x).clone());
        }
        None
    }
}

impl FunctionContext {
    pub fn load_symbol(&self, sym: _Str) -> Option<Value> {
        if let Some(ref x) = self.vtab.borrow().get(sym.as_ref()) {
            // 先看看本函数上下文有木有
            return Some((*x).clone());
        } else if let Some(ref x) = self.fun.env {
            // 再看看有木有env
            if let Some(ref x) = x.load_symbol(sym) {
                // 再看看env有木有
                return Some((*x).clone());
            }
        } else {
            if let Some(ref x) = self.fun.modu.upgrade().unwrap().load_symbol(sym) {
                // 没有的话看看所在模块有没有
                return Some((*x).clone());
            }
        }
        // 没有就真凉了
        None
    }
}

impl ThreadContext {
    pub fn new() -> ThreadContext {
        let module = Handle::from(system_module());
        let mut hs: HashMap<String, Value> = HashMap::new();
        hs.insert("--name--".to_string(), Value::String(Handle::from("--main--".to_string())));
        hs.insert("System".to_string(), Value::Module(Handle::from(module)));

        ThreadContext {
            using_mod: RefCell::from(
                Arc::from(
                    ModuleContext {
                        path: String::from("--main--"),
                        expr: Vec::new(),
                        vtab: RefCell::from(hs),
                    }
                )),
            framestack: RefCell::from(vec![RefCell::from(None); 256]),
            frame_size: RefCell::from(0),
        }
    }

    pub fn get_stack_top(&self) -> Arc<FunctionContext> {
        let fs = self.framestack.borrow();
        let fc = fs[self.frame_size.borrow().clone() - 1].borrow();
        fc.clone().unwrap()
    }

    pub fn load_symbol(&self, sym: _Str) -> Option<Value> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.*)::(.*)").unwrap();
        }
        match RE.captures(sym.as_ref()) {
            Some(x) => {
                // 如果匹配到了
                if self.frame_size.borrow().clone() != 0 {
                    // 取出栈顶
                    let cm = self.get_stack_top();
                    let c = &x[1];
                    let d = &x[2];
                    if let Some(x) = cm.load_symbol(Handle::from(c.to_string())) {
                        if let Value::Module(ref y) = x {
                            if let Some(z) = y.load_symbol(Handle::from(d.to_string())) {
                                return Some(z);
                            }
                        }
                    }
                } else {
                    // 取出正在用的module
                    let cm = self.using_mod.borrow();
                    let c = &x[1];
                    let d = &x[2];
                    if let Some(x) = cm.load_symbol(Handle::from(c.to_string())) {
                        if let Value::Module(ref y) = x {
                            if let Some(z) = y.load_symbol(Handle::from(d.to_string())) {
                                return Some(z);
                            }
                        }
                    }
                }
            }
            None => {
                // 如果没匹配到
                // found Symbol in this FunctionContext
                if self.frame_size.borrow().clone() != 0 {
                    let fs = self.get_stack_top();
                    if let Some(x) = fs.load_symbol(sym.clone()) {
                        return Some(x);
                    }
                    let w = fs.fun.modu.upgrade().unwrap();
                    if let Some(x) = w.load_symbol(Handle::from("System".to_string())) {
                        if let Value::Module(ref y) = x {
                            if let Some(z) = y.load_symbol(sym.clone()) {
                                return Some(z);
                            }
                        }
                    }
                    if let Some(x) = w.load_symbol(Handle::from("Prelude".to_string())) {
                        if let Value::Module(ref y) = x {
                            if let Some(z) = y.load_symbol(sym.clone()) {
                                return Some(z);
                            }
                        }
                    }
                } else {
                    // 自己这个module里边掏掏看
                    if let Some(x) = self.using_mod.borrow().load_symbol(Handle::from(sym.clone())) {
                        return Some(x);
                    }
                    // found Symbol in 'Prelude' ModuleContext
                    if let Some(x) = self.using_mod.borrow().load_symbol(Handle::from("System".to_string())) {
                        if let Value::Module(ref y) = x {
                            if let Some(z) = y.load_symbol(sym.clone()) {
                                return Some(z);
                            }
                        }
                    }
                    if let Some(x) = self.using_mod.borrow().load_symbol(Handle::from("Prelude".to_string())) {
                        if let Value::Module(ref y) = x {
                            if let Some(z) = y.load_symbol(sym.clone()) {
                                return Some(z);
                            }
                        }
                    }
                }
            }
        }
        None
    }
}