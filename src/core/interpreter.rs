use std::sync::Arc;
use std::sync::RwLock;
use std::cell::Cell;
use std::collections::HashMap;

use value::Value;
use func::Function;
use core::module;

pub struct FunctionContext {
    pub fp: Arc<Function>,
    pub ap: Value,
    pub vartable: Cell<HashMap<String, Value>>,
    pub next: Arc<FunctionContext>,
}

pub type InterpreterContext = HashMap<String, Arc<module::Module>>;

pub struct ThreadContext {
    pub ic: Arc<RwLock<Cell<InterpreterContext>>>,
    //pub name: String,
    pub using_module: Cell<Option<Arc<module::Module>>>,
    pub framestack: Cell<Option<Arc<FunctionContext>>>,
}

impl ThreadContext {
    //pub fn init(argc: u64, argv: &[&str]) -> Self {}
    pub fn new() -> Self {
        ThreadContext {
            ic: Arc::new(RwLock::from(Cell::from(HashMap::new()))),
            framestack: Cell::from(None),
            using_module: Cell::from(None),
        }
    }
}

/*
pub struct InterpreterContext {
    pub module_table: Cell<HashMap<String, Arc<module::Module>>>,
}

impl InterpreterContext {
    //pub fn init(argc: u64, argv: &[&str]) -> InterpreterContext {}
    //pub fn new() -> InterpreterContext {}
    //pub fn run(&self) -> ! {}
}
*/
