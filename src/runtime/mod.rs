use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MStruct {
    type_name: String,
    var_table: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum RawValue {
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
    Char(char),
    Symbol(Arc<String>),
    String(Arc<String>),
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
    Char(char),
    Tuple(Arc<Vec<Value>>),
    Symbol(Arc<String>),
    String(Arc<String>),
    Struct(Arc<MStruct>),
    Closure,
}

fn get_tuple_type(_v : &Vec<Value>) -> String {
    String::from("()")
}

impl Value {
    pub fn get_type(&self) -> String {
        match self {
            Value::Int(_) => String::from("int"),
            Value::UInt(_) => String::from("uint"),
            Value::Bool(_) => String::from("bool"),
            Value::Char(_) => String::from("char"),
            Value::Float(_) => String::from("float"),
            Value::Symbol(_) => String::from("symbol"),
            Value::String(_) => String::from("string"),
            Value::Tuple(_) => String::from("tuple"),
            Value::Struct(ref x) => "struct-".to_string() + &x.type_name,
            Value::Closure => String::from("closure"),
        }
    }
    pub fn get_type_info(&self) -> String {
        //FIXME
        match self {
            Value::Tuple(ref x) => String::from(get_tuple_type(x)),
            Value::Struct(ref x) => "struct ".to_string() + &x.type_name + "{" + {
                ""
            } + "}",
            Value::Closure => String::from("closure"),
            _ => self.get_type()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ast {
    None, //FIXME
    Atom(RawValue),
    List(Vec<RawValue>),
}