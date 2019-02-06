use std::sync::Arc;
use std::collections::HashMap;
use num::bigint::BigInt;
use num::bigint::BigUint;
use num::rational::BigRational;

/*
#[derive(Debug, Clone)]
struct MStruct {
    name: Arc<String>,
    tabl: Arc<HashMap<String, Tree>>
}
*/

#[derive(Debug, Clone)]
pub enum Value {
    None,

    Bool(bool),
    Char(char),

    Uint(BigUint),
    Int(BigInt),
    Rational(BigRational),

    CharString(Arc<String>),
    Tuple(Arc<Vec<Value>>),
}

#[derive(Debug, Clone)]
pub struct MClosure {
    mode: Value,
    body:  Ast,
}

#[derive(Debug, Clone)]
pub struct MFunction {
    name: Arc<String>,
    attr: Vec<Arc<String>>,
    mode: Value,
    body: Ast
}

type Open = Arc<String>;

#[derive(Debug, Clone)]
pub struct MModule {
    pub name: Arc<String>,
    pub func: Vec<Arc<MFunction>>,
}

#[derive(Debug, Clone)]
pub struct MFile {
    loads: Vec<Open>,
    modus: Arc<MModule>
}

#[derive(Debug, Clone)]
pub enum Tree {
    Symbol(Arc<String>),

    Value(Value),

    Asts(Vec<Ast>),

    Open(Open),

    Closure(Arc<MClosure>),

    Function(Arc<MFunction>),

    Module(Arc<MModule>),

    Root(Arc<MFile>),
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub val: Tree,
    pub col: usize,
    pub lin: usize
}