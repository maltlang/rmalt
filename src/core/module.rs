use std::collections::HashMap;
use value::Value;
use std::cell::Cell;

pub struct Module {
    pub path: String,
    pub asts: Cell<Vec<Value>>,
    pub vartable: Cell<HashMap<String, Value>>
}

impl Module {
    /*
    fn new(p: &String, asts: &Vec<Value>, v: &HashMap<String, Value>) -> Self {
        Module {
            path: p.clone(),
            asts: Cell::from(asts.clone()),
            vartable: Cell::from(v.clone()),
        }
    }
    */
}