use std::sync::Arc;
use std::cell::RefCell;
use std::collections::HashMap;

use func::Native;
//use func::Function;
use value::Value;
use value::_Tuple;
use value::_Function;
use value::MaltResult;
use runtime::context::ThreadContext;
use runtime::context::FunctionContext;
use value::Handle;

pub mod context;

pub fn exception(class: &str, info: &str) -> Value {
    let mut r: HashMap<String, Value> = HashMap::new();
    r.insert(String::from("__class__"), Value::Symbol(Handle::from(String::from(class))));
    r.insert(String::from("__info__"), Value::String(Handle::from(String::from(info))));
    Value::Object(Arc::from(r))
}

#[inline]
fn args_length_exception() -> Value {
    exception("FunctionCallError", "Wrong number of parameters")
}

#[inline]
fn symbol_not_found_exception(sym: &str) -> Value {
    exception("SymbolError", &("Symbol '".to_string() + sym + "' not found"))
}

//#[inline]
//fn object_member_eval_is_not_function_exception() -> Value {
//    exception("TypeError", "Object member '__eval__' is not function")
//}

pub fn call_function(this: _Function, ic: &ThreadContext, args: _Tuple) -> MaltResult {
    ic.framestack.borrow_mut()[ic.frame_size.borrow().clone()]
        .replace(Option::from(Arc::from(
            FunctionContext {
                fun: this.clone(),
                vtab: RefCell::from(HashMap::new()),
            })));
    ic.frame_size.replace(ic.frame_size.borrow().clone() + 1);

    if args.len() != this.argn.len() {
        return Err(args_length_exception());
    }

    for i in 0..args.len() - 1 {
        let a = ic.framestack.borrow_mut();
        let b = a[ic.frame_size.borrow().clone() - 1].borrow_mut().clone().unwrap();
        b.vtab.borrow_mut().insert(this.clone().argn[i].clone(), args[i].clone());
    }

    let mut r = Value::Nil;
    for i in this.clone().expr.clone() {
        let x = i.eval(ic)?;
        r = x;
    }

    ic.framestack.borrow_mut()[ic.frame_size.borrow().clone() - 1].replace(None);
    ic.frame_size.replace(ic.frame_size.borrow().clone() - 1);
    return Ok(r);
}


impl Native {
    pub fn call_function(&self, ic: &ThreadContext, args: _Tuple) -> MaltResult {
        (self.fp)(ic, args)
    }
}

fn expr_eval(ic: &ThreadContext, expr: _Tuple) -> MaltResult {
    if expr.len() == 0 {
        return Ok(Value::Nil);
    }
    // 检测语句类型（暂时只有call）

    // fun call
    let mut r: Vec<Value> = vec![];
    for i in &*expr {
        let x = i.eval(ic)?;
        r.push(x);
    }
    let head = r.remove(0);
    if let Value::Function(ref x) = &head {
        return call_function(x.clone(), ic, Arc::from(r));
    } else if let Value::Native(ref x) = &head {
        return x.call_function(ic, Arc::from(r));
    } else {
        return Err(exception("CallError", "The callee is not function"));
    }
}

impl Value {
    // 慎用，这玩意会把tuple当成调用来搞
    pub fn eval(&self, ic: &ThreadContext) -> MaltResult {
        match self {
            Value::Symbol(ref x) => match ic.load_symbol(x.clone()) {
                Some(x) => Ok(x),
                None => Err(symbol_not_found_exception(x.as_ref())),
            },
            Value::Object(ref _x) => {
                /*
                if let Some(y) = x.get("__eval__") {
                    if let Value::Function(ref z) = y {
                        call_function(z.clone(), ic, Arc::from(vec![]))
                    } else if let Value::Native(ref z) = y {
                        z.call_function(ic, Arc::from(vec![]))
                    } else {
                        Err(object_member_eval_is_not_function_exception())
                    }
                }
                */
                Ok(self.clone())
            }
            // function call
            Value::Tuple(ref x) => expr_eval(ic, x.clone()),
            _ => Ok(self.clone())
        }
    }
}