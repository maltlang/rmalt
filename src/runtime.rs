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

    CharString(String),
    //Tuple(Vec<Value>),
}

/*
#[derive(Debug, Clone)]
pub struct MClosure {
    mode: Value,
    body:  Ast,
}

#[derive(Debug, Clone)]
pub struct MFunction {
    name: Arc<String>,
    attr: Vec<Arc<String>>,
    mode: Ast,
    body: Ast
}

type Load = Arc<String>;
*/

#[derive(Debug, Clone)]
pub enum Tree {
    Symbol(Arc<String>),

    Value(Value),

    Asts(Vec<Ast>),

    Tuple(Vec<Ast>),
    /*
    Load(Load),

    If(If),

    Match(Match),

    Closure(Arc<MClosure>),

    Function(Arc<MFunction>),

    Macro(Arc<MFunction>),
    */
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub val: Tree,
    pub col: usize,
    pub lin: usize
}