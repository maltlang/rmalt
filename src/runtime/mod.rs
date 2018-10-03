use std::sync::Arc;
use std::cell::RefCell;
use std::collections::HashMap;

use func::Function;
use func::Native;
use value::Value;
use value::_Tuple;
use value::_Function;
use value::MaltResult;
use runtime::context::ThreadContext;
use runtime::context::FunctionContext;
use value::_Dict;

pub mod context;

impl Function {
    fn call_function(this: _Function, ic: &ThreadContext, args: _Tuple) -> MaltResult {
        ic.framestack.borrow_mut()[ic.frame_size.borrow().clone()]
            .replace(Option::from(Arc::from(
                FunctionContext {
                    fun: this.clone(),
                    vtab: RefCell::from(HashMap::new()),
                })));
        ic.frame_size.replace(ic.frame_size.borrow().clone() + 1);

        for i in 0..args.len() - 1 {
            let a = ic.framestack.borrow_mut();
            let b = a[ic.frame_size.borrow().clone() - 1].borrow_mut().clone().unwrap();
            b.vtab.borrow_mut().insert(this.clone().argn[i].clone(), args[i].clone());
        }

        let mut r = Value::Nil;
        for i in this.clone().expr.clone() {
            match i.eval(ic) {
                Ok(x) => {
                    r = x;
                }
                Err(e) => return Err(e)
            }
        }

        ic.framestack.borrow_mut()[ic.frame_size.borrow().clone() - 1].replace(None);
        ic.frame_size.replace(ic.frame_size.borrow().clone() - 1);
        return Ok(r);
    }
}

impl Native {
    fn call_function(&self, ic: &ThreadContext, args: _Tuple) -> MaltResult {
        (self.fp)(ic, args)
    }
}

// other
fn symbol_not_found_exception() -> Value {
    let mut r: HashMap<String, Value> = HashMap::new();
    r.insert("__class__".to_string(), Value::Symbol(Arc::from("SymbolError".to_string())));
    r.insert("__info__".to_string(), Value::String(Arc::from("Symbol not found".to_string())));
    Value::Object(Arc::from(r))
}

impl Value {
    // 慎用，这玩意会把list当成调用来搞
    pub fn eval(&self, ic: &ThreadContext) -> MaltResult {
        match self {
            Value::Symbol(ref x) => match ic.load_symbol(x.clone()) {
                Some(x) => Ok(x),
                None => Err(symbol_not_found_exception()),
            },
            Value::Object(ref x) => Ok(Value::Object(x.clone())),
            // function call
            Value::Tuple(ref x) => Ok(Value::Tuple(x.clone())),
            _ => Ok(self.clone())
        }
    }
}