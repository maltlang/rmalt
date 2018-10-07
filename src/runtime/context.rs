use std::sync::Arc;
use std::sync::RwLock;
use std::cell::RefCell;
use std::collections::HashMap;

//use lazy_static;
use regex::Regex;

use value::Value;
use value::_Str;
use value::_Function;

pub struct ModuleContext {
    pub path: String,
    pub expr: Vec<Value>,
    // 其实可以不用这个字段的，但我要codegen
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

/// symbol space
/// - global
///     - function
///         - locals
///         - env

/// load_symbol logic
/// if symbol => symbol
/// then locals -> env
/// then global
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
        }
        // 木有QwQ
        None
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

    pub fn load_symbol(&self, sym: _Str) -> Option<Value> {
        // 切分
        ///*
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.)::(.)").unwrap();
        }
        //*/
        //let RE = Regex::new(r"(.)::(.)").unwrap();
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
                    return Some(x);
                }
            }
            // found Symbol in this ModuleContext
            if let Some(x) = self.using_mod.load_symbol(sym.clone()) {
                return Some(x);
            }
            // found Symbol in 'Prelude' ModuleContext
            let cm = self.commonmod.read().unwrap();
            return match cm.borrow().get("Prelude") {
                Some(x) => x.load_symbol(sym.clone()),
                None => None,
            };
        } else if a.len() == 1 {
            // 如果匹配到了
            let cm = self.commonmod.read().unwrap();
            let c = &(&a)[0].0;
            return match cm.borrow().get(c.as_str()) {
                Some(x) => x.load_symbol(Arc::from((&a)[0].1.clone())),
                None => None,
            };
        } else {
            // 如果匹配个数大于1，那就说明有人在骚搞，打死
            return None;
        }
    }
}