use std::collections::HashMap;
use value::Value;
use std::cell::Cell;

pub struct Module {
    pub path: String,
    pub asts: Vec<Value>,
    pub vartable: Cell<HashMap<String, Value>>
}