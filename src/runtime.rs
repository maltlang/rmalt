use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MStruct {
    type_name: String,
    var_table: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
    Char(char),
    Tuple(Arc<Vec<Value>>),
    //Symbol(Arc<String>),
    String(Arc<String>),
    Struct(Arc<MStruct>),
    Closure,
}

impl Value {
    pub fn get_type(&self) -> String {
        match self {
            Value::Nil => String::from("nil"),
            Value::Int(_) => String::from("int"),
            Value::UInt(_) => String::from("uint"),
            Value::Bool(_) => String::from("bool"),
            Value::Char(_) => String::from("char"),
            Value::Float(_) => String::from("float"),
            //Value::Symbol(_) => String::from("symbol"),
            Value::String(_) => String::from("string"),
            Value::Tuple(_) => String::from("tuple"),
            Value::Struct(ref x) => "struct-".to_string() + &x.type_name,
            Value::Closure => String::from("closure"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub val: Value,
    pub col: usize,
    pub lin: usize
}