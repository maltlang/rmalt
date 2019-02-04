use std::sync::Arc;
use std::collections::HashMap;
use num::bigint::BigInt;
use num::bigint::BigUint;
use num::rational::BigRational;

#[derive(Debug, Clone)]
struct MStruct {
    name: Arc<String>,
    tabl: Arc<HashMap<String, Value>>
}

#[derive(Debug, Clone)]
pub enum Value {
    None,

    Uint(BigUint),
    Int(BigInt),
    Rational(BigRational),

    Bool(bool),
    Char(char),

    CharString(Arc<String>),
    //Struct(MStruct),
    Tuple(Arc<Vec<Value>>),
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub val: Value,
    pub col: usize,
    pub lin: usize
}