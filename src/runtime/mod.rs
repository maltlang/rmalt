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
    // 检测语句类型
    if let Value::Symbol(ref x) = expr[0].clone() {
        if **x == "quote".to_string() {
            if expr.len() != 2 {
                return Err(exception("PredicateError", "'quote' parameters number is not 1.\n\thelp: (quote <expr>)"));
            }
            return Ok(expr[1].clone());
        } else if **x == "let".to_string() {
            if expr.len() != 3 {
                return Err(exception("PredicateError", "'let' parameters number is not 2.\n\thelp: (let <symbol> <expr>)"));
            }
            if let Value::Symbol(ref x) = expr[1].clone() {
                let val = expr[2].clone().eval(ic)?;
                if ic.frame_size.borrow().clone() == 0 {
                    // 表示在顶层作用域
                    ic.using_mod.vtab.borrow_mut().insert(x.to_string(), val.clone());
                } else {
                    // 表示在函数作用域
                    let fs = ic.framestack.borrow();
                    let fc = fs[ic.frame_size.borrow().clone() - 1].borrow();
                    let sfc = fc.clone().unwrap();
                    sfc.vtab.borrow_mut().insert(x.to_string(), val.clone());
                }
                return Ok(val);
            } else {
                return Err(exception("PredicateError", "'let' parameters 1 is not symbol type."));
            }
        } else if **x == "if".to_string() {
            // TODO: if expr eval
            if expr.len() == 3 {
                let boolval = expr[1].clone().eval(ic)?;
                if let Value::Bool(x) = boolval {
                    return if x {
                        expr[2].clone().eval(ic)
                    } else {
                        Ok(Value::Nil)
                    };
                } else {
                    return Err(exception("TypeError", "if cond expr is not bool result."));
                }
            } else if expr.len() == 4 {
                let boolval = expr[1].clone().eval(ic)?;
                if let Value::Bool(x) = boolval {
                    return if x {
                        expr[2].clone().eval(ic)
                    } else {
                        expr[3].clone().eval(ic)
                    };
                } else {
                    return Err(exception("TypeError", "if cond expr is not bool result."));
                }
            } else {
                return Err(exception("PredicateError", "'if' parameters number is not 2 or 3.\n\thelp: (if <boolexpr>  <thenexpr> [<elseexpr>])"));
            }
        } else if **x == "cond".to_string() {
            for (i, v) in expr.iter().enumerate() {
                if i != 0 {
                    if let Value::Tuple(x) = v.clone() {
                        if x.len() != 2 {
                            return Err(exception("PredicateError", "'cond' parameters tuple len() is not 2.\n\thelp: (cond [<boolexpr> <expr>]*)"));
                        }
                        // 形式正确
                        if if let Value::Bool(x) = x[0].clone().eval(ic)? { x } else {
                            return Err(exception("TypeError", "cond expr is not bool result."));
                        } {
                            return x[1].clone().eval(ic);
                        }
                    } else {
                        return Err(exception("PredicateError", "'cond' parameters is not tuple.\n\thelp: (cond [<boolexpr> <expr>]*)"));
                    }
                }
            }
            return Ok(Value::Nil);
        } else if **x == "match".to_string() {
            // TODO: match expr eval
        } else if **x == "loop!".to_string() {
            loop {
                for (i, v) in expr.iter().enumerate() {
                    if i != 0 {
                        v.clone().eval(ic)?;
                    }
                }
            }
        } else if **x == "while!".to_string() {
            if expr.len() < 2 {
                return Err(exception("PredicateError", "'while!' parameters number is less 2.\n\thelp: (while! <boolexpr> [<expr>*])"));
            }
            while if let Value::Bool(x) = expr[1].clone().eval(ic)? { x } else {
                return Err(exception("TypeError", "while! cond expr is not bool result."));
            } {
                for (i, v) in expr.iter().enumerate() {
                    if i > 1 {
                        eprintln!("item eval");
                        v.clone().eval(ic)?;
                    }
                }
            }
            return Ok(Value::Nil);
        }
        // for!是不需要存在的！
        //追加：其实while!也是不需要存在的
    }
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
        return Err(exception("CallError", &("The callee '".to_string() + &head.to_string() + "' is not function")));
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