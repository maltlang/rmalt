use std::sync::Arc;
use std::sync::Weak;
use std::sync::Mutex;
use std::cell::Cell;
use std::collections::HashMap;
use std::thread::Thread;
use value::Value;
use func::Function;
use ast::Ast;
use core::module;


pub struct FunctionContext {
    pub fp: Arc<Function>,
    pub ap: Weak<Ast>,
    pub vartable: Cell<HashMap<String, Value>>,
    pub next: Arc<FunctionContext>,
}

pub struct ThreadContext {
    pub ic: Arc<Mutex<InterpreterContext>>,
    //pub th: Arc<Mutex<Thread>>,
    pub name: String,
    pub using_module: Arc<module::Module>,
    pub framestack: Cell<Arc<FunctionContext>>,
}

pub struct InterpreterContext {
    pub module_table: Cell<HashMap<String, Arc<module::Module>>>,
}

impl InterpreterContext {
    //pub fn init(argc: u64, argv: &[&str]) -> InterpreterContext {}
    //pub fn new() -> InterpreterContext {}
    //pub fn run(&self) -> ! {}
}