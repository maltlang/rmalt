use std::collections::HashMap;
use value::Value;
use value::ast::Ast;

pub struct Module {
    pub path: String,
    pub asts: Vec<Ast>,
    pub vartable: HashMap<String, Value>
}