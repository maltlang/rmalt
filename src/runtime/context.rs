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
use runtime::exception;

pub struct ModuleContext {
    pub path: String,
    // 其实可以不用这个字段的，但我要codegen
    pub expr: Vec<Value>,
    pub vtab: HashMap<String, Value>, // env (var table)
}

pub struct FunctionContext {
    pub fun: _Function,
    pub vtab: RefCell<HashMap<String, Value>>, // env (var table)
}

type CommonModuleContext = RefCell<HashMap<String, Arc<ModuleContext>>>;

pub struct ThreadContext {
    pub commonmod: Arc<RwLock<CommonModuleContext>>,
    pub using_mod: Arc<ModuleContext>,
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
        if let Some(ref x) = self.vtab.get(sym.as_ref()) {
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
            // 如果没有env，就看global有木有 else
            let m = self.fun.modu.upgrade().unwrap(); // 逻辑上来说函数的生命周期不可能比模块短，如果是，那就说明有人在骚搞
            // found Symbol in this ModuleContext
            if let Some(x) = m.load_symbol(sym) {
                return Some(x);
            }
            // 没有就凉啦
            return None;
        }
    }
}

/// libs
fn num_to_uint(n: Value) -> Option<u64> {
    match n {
        Value::Float(x) => Some(x as u64),
        Value::UInt(x) => Some(x as u64),
        Value::Int(x) => Some(x as u64),
        _ => None
    }
}

fn num_to_int(n: Value) -> Option<i64> {
    match n {
        Value::Float(x) => Some(x as i64),
        Value::UInt(x) => Some(x as i64),
        Value::Int(x) => Some(x as i64),
        _ => None
    }
}

fn num_to_float(n: Value) -> Option<f64> {
    match n {
        Value::Float(x) => Some(x as f64),
        Value::UInt(x) => Some(x as f64),
        Value::Int(x) => Some(x as f64),
        _ => None
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
            using_mod: Arc::from(ModuleContext {
                path: String::from("__none__"),
                expr: Vec::new(),
                vtab: HashMap::new(),
            }),
            framestack: RefCell::from(vec![RefCell::from(None); 256]),
            frame_size: RefCell::from(0),
        }
    }

    pub fn test_new() -> ThreadContext {
        // 写单模块
        let mut vt: HashMap<String, Value> = HashMap::new();
        vt.insert(String::from("__version__"), Value::Native(Handle::from(Native {
            name: String::from("__version__"),
            fp: |_ic, _args| {
                let a = vec![Value::UInt(0), Value::UInt(0)];
                Ok(Value::Tuple(Handle::from(a)))
            },
        })));
        vt.insert(String::from("+"), Value::Native(Handle::from(Native {
            name: String::from("__add__"),
            fp: |_ic, args| {
                if args.len() != 2 {
                    return Err(args_length_exception());
                }
                add(args[0].clone(), args[1].clone())
            },
        })));
        // 写模块表
        let mut modu: HashMap<String, Arc<ModuleContext>> = HashMap::new();
        modu.insert(String::from("Prelude"), Arc::from(ModuleContext {
            path: String::from("Prelude"),
            expr: Vec::new(),
            vtab: vt,
        }));
        ThreadContext {
            commonmod: Arc::from(RwLock::from(RefCell::from(modu))),
            using_mod: Arc::from(ModuleContext {
                path: String::from("__none__"),
                expr: Vec::new(),
                vtab: HashMap::new(),
            }),
            framestack: RefCell::from(vec![RefCell::from(None); 256]),
            frame_size: RefCell::from(0),
        }
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
                let a = self.framestack.borrow();
                let fs = a[self.frame_size.borrow().clone()].borrow();
                if let Some(x) = fs.clone().unwrap().load_symbol(sym.clone()) {
                    println!("在函数里边找到了");
                    return Some(x);
                }
            }
            // found Symbol in 'Prelude' ModuleContext
            let cm = self.commonmod.read().unwrap();
            if let Some(ref x) = cm.borrow().get("Prelude") {
                return x.load_symbol(sym.clone());
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