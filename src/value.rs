use std::sync::Arc;
use std::collections::HashMap;
use ast::Ast;
use func::Function;
use func::Native;
use mmacro::BaseMacro;

///## 类型重命名
pub type Handle<T> = Arc<T>;
pub type _Str = Handle<String>;
pub type _Tuple = Handle<Vec<Value>>;
pub type _Dict = Handle<HashMap<String, Value>>;
pub type _Ast = Handle<Ast>;
pub type _Function = Handle<Function>;
pub type _Native = Handle<Native>;
pub type _BaseMacro = Handle<BaseMacro>;

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
    List(_List),
    Dict(_Dict),
    Object(_Dict),

    // Ast(_Ast),

    // functions
    Function(_Function),
    Native(_Native),

    // macros
    Macro(_Function),
    BaseMacro(_BaseMacro),
}


impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Nil => "nil".to_string(),
            Value::Int(ref x) => x.to_string(),
            Value::UInt(ref x) => x.to_string(),
            Value::Bool(ref x) => x.to_string(),
            Value::Char(ref x) => x.to_string(),
            Value::Float(ref x) => x.to_string(),
            Value::Symbol(ref x) => x.to_string(),
            Value::String(ref x) => x.to_string(),
            Value::Object(_) => "<object>".to_string(),
            Value::Macro(ref x) => "<macro ".to_string() + &*x.name + ">",
            Value::Native(ref x) => "<native ".to_string() + &*x.name + ">",
            Value::Function(ref x) => "<function ".to_string() + &*x.name + ">",
            Value::BaseMacro(ref x) => "<base-macro ".to_string() + &*x.name + ">",
            // 还没写好的
            //Value::Ast(ref x) => x.to_string(),
            Value::List(_) => "<list>".to_string(),
            Value::Dict(_) => "<dict>".to_string(),
            Value::Tuple(ref x) => "<tuple>".to_string(),
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

    // get value type -> string
    pub fn get_type(&self) -> String {
        match self {
            Value::Nil => "nil".to_string(),
            //Value::Ast(_) => "ast".to_string(),
            Value::Int(_) => "int".to_string(),
            Value::UInt(_) => "uint".to_string(),
            Value::Bool(_) => "bool".to_string(),
            Value::Char(_) => "char".to_string(),
            Value::List(_) => "list".to_string(),
            Value::Dict(_) => "dict".to_string(),
            Value::Float(_) => "float".to_string(),
            Value::Tuple(_) => "tuple".to_string(),
            Value::Symbol(_) => "symbol".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Object(_) => "object".to_string(),
            Value::Function(_) |
            Value::Native(_) => "function".to_string(),
            Value::Macro(_) |
            Value::BaseMacro(_) => "macro".to_string(),
            // Value::Native(_) => Some("<native>".to_string()),
        }
    }
}
