use std::sync::Arc;
use std::sync::Weak;
use std::cell::Cell;
use std::collections::HashMap;
use value::Value;
use ast::Ast;
use func::Function;
use core::module;

pub struct FunctionContext {
    pub fp: Weak<Function>,
    pub ap: Weak<Ast>,
    pub vartable: HashMap<String, Value>,
    pub next: Box<FunctionContext>,
}

pub struct ThrandContext {
    pub name: String,
    pub using_module: Weak<module::Module>,
    pub framestack: Cell<Box<FunctionContext>>,
}

pub struct InterpreterContext {
    pub module_table: HashMap<String, Arc<module::Module>>,
    pub thrand_pool: Vec<ThrandContext>,
}

impl InterpreterContext {
    pub fn init(argc: u64, argv: &[&str]) -> InterpreterContext {

    }
    pub fn new() -> InterpreterContext {

    }
    pub fn run(&self) -> ! {

    }
}