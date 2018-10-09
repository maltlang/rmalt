use std::sync::Arc;
use std::sync::RwLock;
use std::cell::RefCell;
use std::collections::HashMap;

//use lazy_static;
use regex::Regex;

use func::Native;
use value::Value;
use value::Handle;
use value::_Str;
use value::_Function;
use value::MaltResult;
use runtime::args_length_exception;
use runtime::tools::exception;
use runtime::tools::num_to_float;
use runtime::tools::num_to_uint;
use runtime::tools::num_to_int;

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

type CommonModuleContext = RefCell<HashMap<String, Arc<ModuleContext>>>;

pub struct ThreadContext {
    pub commonmod: Arc<RwLock<CommonModuleContext>>,
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
        // 先看看本函数上下文有木有
        if let Some(ref x) = self.vtab.borrow().get(sym.as_ref()) {
            return Some((*x).clone());
        }
        // 再看看环境里边有木有
        if let Some(ref x) = self.fun.env {
            return x.load_symbol(sym);
        } else {
            return None;
        }
    }
}

fn add(a: Value, b: Value) -> MaltResult {
    if a.get_type() != b.get_type() {
        return Err(exception("TypeError", "Function parameters type error"));
    }
    if let Value::Float(x) = a {
        if let Some(a) = num_to_float(b) {
            return Ok(Value::Float(x + a));
        } else {
            return Err(exception("TypeError", "Function parameters type error"));
        }
    } else if let Value::UInt(x) = a {
        if let Some(a) = num_to_uint(b) {
            return Ok(Value::UInt(x + a));
        } else {
            return Err(exception("TypeError", "Function parameters type error"));
        }
    } else if let Value::Int(x) = a {
        if let Some(a) = num_to_int(b) {
            return Ok(Value::Int(x + a));
        } else {
            return Err(exception("TypeError", "Function parameters type error"));
        }
    } else {
        return Err(exception("TypeError", "Function parameters type error"));
    }
}

impl ThreadContext {
    pub fn new() -> ThreadContext {
        // pub type CommonModuleContext = RefCell<HashMap<String, Arc<ModuleContext>>>;
        ThreadContext {
            commonmod: Arc::from(RwLock::from(RefCell::from(HashMap::new()))),
            using_mod: RefCell::from(Arc::from(ModuleContext {
                path: String::from("<Nil>"),
                expr: Vec::new(),
                vtab: RefCell::from(HashMap::new()),
            })),
            framestack: RefCell::from(vec![RefCell::from(None); 256]),
            frame_size: RefCell::from(0),
        }
    }

    pub fn test_new() -> ThreadContext {
        // 写单模块
        let mut vt: HashMap<String, Value> = HashMap::new();
        vt.insert(String::from("true"), Value::Bool(true));
        vt.insert(String::from("false"), Value::Bool(false));
        vt.insert(String::from("--version--"), Value::Native(Handle::from(Native {
            name: String::from("--version--"),
            fp: |_ic, _args| {
                let a = vec![Value::UInt(0), Value::UInt(0)];
                Ok(Value::Tuple(Handle::from(a)))
            },
        })));
        vt.insert(String::from("+"), Value::Native(Handle::from(Native {
            name: String::from("--add--"),
            fp: |_ic, args| {
                if args.len() != 2 {
                    return Err(args_length_exception());
                }
                add(args[0].clone(), args[1].clone())
            },
        })));

        let module = Arc::from(ModuleContext {
            path: String::from("System"),
            expr: Vec::new(),
            vtab: RefCell::from(vt),
        });
        // 写模块表
        let mut modu: HashMap<String, Arc<ModuleContext>> = HashMap::new();
        modu.insert(String::from("System"), module.clone());
        ThreadContext {
            commonmod: Arc::from(RwLock::from(RefCell::from(modu))),
            using_mod: RefCell::from(module),
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
            static ref RE: Regex = Regex::new(r"(.)::(.)").unwrap();
        }
        let mut a: Vec<(String, String)> = vec![];
        for cap in RE.captures_iter(sym.as_ref()) {
            a.push((
                format!("{}", &cap[0]),
                format!("{}", &cap[1])));
        }

        // 分情况处理
        if a.len() == 0 {
            // 如果没匹配到
            // found Symbol in this FunctionContext
            if self.frame_size.borrow().clone() != 0 {
                let fs = self.get_stack_top();
                if let Some(x) = fs.load_symbol(sym.clone()) {
                    return Some(x);
                }
            }
            // found Symbol in 'Prelude' ModuleContext
            let cm = self.commonmod.read().unwrap();
            if let Some(ref x) = cm.borrow().get("System") {
                if let Some(x) = x.load_symbol(sym.clone()) {
                    return Some(x);
                }
            }
            if let Some(ref x) = cm.borrow().get("Prelude") {
                if let Some(x) = x.load_symbol(sym.clone()) {
                    return Some(x);
                }
            }
            return None;
        } else if a.len() == 1 {
            // 如果匹配到了
            let cm = self.commonmod.read().unwrap();
            let c = &(&a)[0].0;
            if let Some(x) = cm.borrow().get(c.as_str()) {
                return x.load_symbol(Arc::from((&a)[0].1.clone()));
            }
            return None;
        } else {
            // 如果匹配个数大于1，那就说明有人在骚搞，打死
            return None;
        }
    }
}