use std::sync::Arc;
use std::collections::HashMap;
use func::Function;
use func::Native;
use runtime::context::ModuleContext;
use runtime::tools::exception;

///## 类型重命名

pub type Handle<T> = Arc<T>;
pub type _Str = Handle<String>;
pub type _Tuple = Handle<Vec<Value>>;
pub type _Dict = Handle<HashMap<String, Value>>;
pub type _Function = Handle<Function>;
pub type _Native = Handle<Native>;
pub type _Module = Handle<ModuleContext>;

/*
pub type _List = Handle<LList>;

pub struct LList {
    len: usize,
    car: Value,
    cdr: Option<_List>,
}

impl LList {
    fn new() -> LList {
        LList {
            len: 0,
            car: Value::Nil,
            cdr: None,
        }
    }
    fn from(a: Value, d: Option<_List>) -> LList {
        match d {
            Some(x) => LList {
                len: x.len + 1,
                car: a,
                cdr: Some(x),
            },
            None => LList {
                len: 1,
                car: a,
                cdr: None,
            },
        }
    }
}
*/


///## Value union
pub enum Value {
    // atom
    Nil,
    Bool(bool),
    Char(char),
    Int(i64),
    UInt(u64),
    Float(f64),
    // Heap Objects
    Symbol(_Str),
    String(_Str),
    Tuple(_Tuple),
    //List(_List),
    Dict(_Dict),
    Object(_Dict),
    // functions
    Function(_Function),
    Native(_Native),
    // macros
    Macro(_Function),
    BaseMacro(_Native),
    Module(_Module),
}

pub type MaltResult = Result<Value, Value>;

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Nil => Value::Nil,
            Value::Int(ref x) => Value::Int(*x),
            Value::UInt(ref x) => Value::UInt(*x),
            Value::Bool(ref x) => Value::Bool(*x),
            Value::Char(ref x) => Value::Char(*x),
            Value::Float(ref x) => Value::Float(*x),
            Value::Dict(ref x) => Value::Dict(x.clone()),
            Value::Tuple(ref x) => Value::Tuple(x.clone()),
            Value::Symbol(ref x) => Value::Symbol(x.clone()),
            Value::String(ref x) => Value::String(x.clone()),
            Value::Object(ref x) => Value::Object(x.clone()),
            Value::Module(ref x) => Value::Module(x.clone()),
            Value::Macro(ref x) => Value::Macro(x.clone()),
            Value::Native(ref x) => Value::Native(x.clone()),
            Value::Function(ref x) => Value::Function(x.clone()),
            Value::BaseMacro(ref x) => Value::BaseMacro(x.clone()),
        }
    }
}

fn to_string(this: &Vec<Value>) -> String {
    let mut s = String::from("(");
    for (i, v) in this.iter().enumerate() {
        if i != 0 {
            s.push_str(" ".to_string().as_ref());
        }
        s.push_str(v.to_string().as_ref());
    }
    s.push_str(")".to_string().as_ref());
    s
}

fn dict_to_string(d: _Dict) -> String {
    let mut s = String::from("{");
    for (mut c, v) in &(*d) {
        s.push(' ');
        s.push_str(c.as_ref());
        s.push_str(": ");
        s.push_str(v.to_string().as_ref());
        s.push(',');
    }
    s.push_str(" }");
    s
}

fn default_object_to_string(d: _Dict) -> String {
    match d.get("__class__") {
        Some(x) => x.to_string() + " " + &dict_to_string(d.clone()),
        None => dict_to_string(d.clone()),
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Nil => "nil".to_string(),
            Value::Int(ref x) => x.to_string(),
            Value::UInt(ref x) => x.to_string(),
            Value::Bool(ref x) => x.to_string(),
            Value::Char(ref x) => x.to_string(),
            Value::Dict(ref x) => dict_to_string(x.clone()),
            Value::Float(ref x) => x.to_string(),
            Value::Tuple(ref x) => to_string(x),
            Value::Symbol(ref x) => x.to_string(),
            Value::String(ref x) => /*"\"".to_string() +*/ (**x).clone() /*+ "\""*/,
            Value::Object(ref x) => default_object_to_string(x.clone()),
            Value::Module(ref x) => "<module '".to_string() + &x.path + "'>",
            Value::Macro(ref x) => "<macro '".to_string() + &*x.name + "'>",
            Value::Native(ref x) => "<native '".to_string() + &*x.name + "'>",
            Value::Function(ref x) => "<function '".to_string() + &*x.name + "'>",
            Value::BaseMacro(ref x) => "<base-macro '".to_string() + &*x.name + "'>",
        }
    }
}

// value opertional tool functions
impl Value {
    // is_atom(in stack)
    pub fn is_atom(&self) -> bool {
        match self {
            Value::Nil |
            Value::Int(_) |
            Value::UInt(_) |
            Value::Bool(_) |
            Value::Char(_) |
            Value::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Int(_) |
            Value::UInt(_) |
            Value::Float(_) => true,
            _ => false,
        }
    }

    // get value type -> string
    pub fn get_type(&self) -> String {
        match self {
            Value::Nil => "nil".to_string(),
            Value::Int(_) => "int".to_string(),
            Value::UInt(_) => "uint".to_string(),
            Value::Bool(_) => "bool".to_string(),
            Value::Char(_) => "char".to_string(),
            Value::Dict(_) => "dict".to_string(),
            Value::Float(_) => "float".to_string(),
            Value::Tuple(_) => "tuple".to_string(),
            Value::Symbol(_) => "symbol".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Object(_) => "object".to_string(),
            Value::Function(_) |
            Value::Native(_) => "function".to_string(),
            Value::Macro(_) |
            Value::BaseMacro
            (_) => "macro".to_string(),
            Value::Module(_) => "module".to_string(),
        }
    }

    pub fn to_uint(&self) -> MaltResult {
        match self {
            Value::Bool(x) => Ok(Value::UInt(*x as u64)),
            Value::Int(x) => Ok(Value::UInt(*x as u64)),
            Value::UInt(x) => Ok(Value::UInt(*x as u64)),
            Value::Char(x) => Ok(Value::UInt(*x as u64)),
            Value::Float(x) => Ok(Value::UInt(*x as u64)),
            //MIFME: Value::Object
            _ => Err(exception("TypeError", "This value cannot be effectively converted to uint")),
        }
    }

    pub fn to_int(&self) -> MaltResult {
        match self {
            Value::Bool(x) => Ok(Value::Int(*x as i64)),
            Value::Int(x) => Ok(Value::Int(*x as i64)),
            Value::UInt(x) => Ok(Value::Int(*x as i64)),
            Value::Char(x) => Ok(Value::Int(*x as i64)),
            Value::Float(x) => Ok(Value::Int(*x as i64)),
            //MIXME: Value::Object
            _ => Err(exception("TypeError", "This value cannot be effectively converted to int")),
        }
    }

    pub fn to_float(&self) -> MaltResult {
        match self {
            Value::Int(x) => Ok(Value::Float(*x as f64)),
            Value::UInt(x) => Ok(Value::Float(*x as f64)),
            Value::Float(x) => Ok(Value::Float(*x as f64)),
            //MIXME: Value::Object
            _ => Err(exception("TypeError", "This value cannot be effectively converted to float")),
        }
    }

    pub fn to_char(&self) -> MaltResult {
        match self {
            Value::Bool(x) => Ok(Value::Char(*x as u8 as char)),
            Value::Int(x) => Ok(Value::Char(*x as u8 as char)),
            Value::UInt(x) => Ok(Value::Char(*x as u8 as char)),
            Value::Char(x) => Ok(Value::Char(*x as u8 as char)),
            //MIXME: Value::String, Value::Object
            _ => Err(exception("TypeError", "This value cannot be effectively converted to char")),
        }
    }
}
