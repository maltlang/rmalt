use std::collections::HashMap;
use value::Value;
use ast::Ast;
use std::cell::Cell;

pub struct Module {
    pub path: String,
    pub asts: Vec<Ast>,
    pub vartable: Cell<HashMap<String, Value>>
}