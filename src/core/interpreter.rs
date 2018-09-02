use std::sync::Arc;
use std::sync::Weak;
use std::sync::Mutex;
use std::cell::Cell;
use std::collections::HashMap;
use value::Value;
use func::Function;
use ast::Ast;
use core::module;


pub struct FunctionContext {
    pub fp: Weak<Function>,
    pub ap: Weak<Ast>,
    pub vartable: Cell<HashMap<String, Value>>,
    pub next: Box<FunctionContext>,
}

pub struct ThrandContext {
    pub ic: Weak<Mutex<InterpreterContext>>,
    pub name: String,
    pub using_module: Weak<Mutex<module::Module>>,
    pub framestack: Cell<Box<FunctionContext>>,
}

pub struct InterpreterContext {
    pub module_table: Cell<HashMap<String, Arc<Mutex<module::Module>>>>,
    //pub thrand_pool: Vec<ThrandContext>,
    // other
}

impl InterpreterContext {
    // pub fn init(argc: u64, argv: &[&str]) -> InterpreterContext {}
    //pub fn new() -> InterpreterContext {}
    //pub fn run(&self) -> ! {}
}