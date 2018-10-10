use std::sync::Arc;
use std::cell::RefCell;
use std::collections::HashMap;

use func::Native;
use func::Function;
use value::Value;
//use value::_Str;
use value::Handle;
use value::_Tuple;
use value::_Function;
use value::MaltResult;
use runtime::context::ThreadContext;
use runtime::context::FunctionContext;
use runtime::tools::exception;
use runtime::tools::let_value;
//use runtime::tools::set_value;

pub mod tools;
pub mod system;
pub mod context;

#[inline]
fn args_length_exception() -> Value {
    exception("FunctionCallError", "Wrong number of parameters")
}

#[inline]
fn symbol_not_found_exception(sym: &str) -> Value {
    exception("SymbolError", &("Symbol '".to_string() + sym + "' not found"))
}

pub fn call_function(this: _Function, ic: &ThreadContext, args: _Tuple) -> MaltResult {
    ic.framestack.borrow_mut()[ic.frame_size.borrow().clone()]
        .replace(Option::from(Arc::from(
            FunctionContext {
                fun: this.clone(),
                vtab: RefCell::from(HashMap::new()),
            })));
    let s = ic.frame_size.borrow().clone() + 1;
    ic.frame_size.replace(s);

    if args.len() != this.argn.len() {
        return Err(args_length_exception());
    }
    if args.len() != 0 {
        for i in 0..args.len() {
            let b = ic.get_stack_top();
            b.vtab.borrow_mut().insert(this.clone().argn[i].clone(), args[i].clone());
        }
    }

    let mut r = Value::Nil;
    for i in this.clone().expr.clone() {
        let x = i.eval(ic)?;
        r = x;
    }

    ic.framestack.borrow_mut()[ic.frame_size.borrow().clone() - 1].replace(None);
    let s = ic.frame_size.borrow().clone() - 1;
    ic.frame_size.replace(s);
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
            //FIXME: 由于不是语言core的core，let将被做为函数（在System Module）而不是谓词
            if expr.len() != 3 {
                return Err(exception("PredicateError", "'let' parameters number is not 2.\n\thelp: (let <symbol> <expr>)"));
            }
            if let Value::Symbol(x) = expr[1].clone() {
                let e = expr[2].clone().eval(ic)?;
                let_value(ic, x, e.clone())?;
                //set_value(ic, x, e.clone());
                return Ok(e);
            } else {
                return Err(exception("PredicateError", "'let' parameters 1 is not symbol type."));
            }
        } else if **x == "if".to_string() {
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
            /*
            if expr.len() < 2 {
                return Err(exception("PredicateError", "'while!' parameters number is less 2.\n\thelp: (while! <boolexpr> [<expr>*])"));
            }
            while if let Value::Bool(x) = expr[1].clone().eval(ic)? { x } else {
                return Err(exception("TypeError", "while! cond expr is not bool result."));
            } {
                for (i, v) in expr.iter().enumerate() {
                    if i > 1 {
                        v.clone().eval(ic)?;
                    }
                }
            }
            return Ok(Value::Nil);
            */
        } else if **x == "lambda".to_string() {
            if expr.len() < 3 {
                return Err(exception("PredicateError", "'lambda' parameters number is less 3.\n\thelp: (lambda <tuple> [<tuple>]*)"));
            }
            let mut argn: Vec<String> = vec![];
            if let Value::Tuple(x) = expr[1].clone() {
                for i in x.iter() {
                    if let Value::Symbol(x) = i {
                        argn.push(x.to_string());
                    } else {
                        return Err(exception("PredicateError", "'lambda' defined function parameters list tiem is not symbol."));
                    }
                }
            } else {
                return Err(exception("PredicateError", "'lambda' defined function parameters list is not tuple"));
            }
            let mut e: Vec<Value> = vec![];
            for (i, v) in expr.iter().enumerate() {
                if i > 1 {
                    e.push(v.clone());
                }
            }
            let f = Function {
                modu: Arc::downgrade(&ic.using_mod.borrow()),
                name: String::from("<lambda>"),
                expr: e,
                argn,
                env: if ic.frame_size.borrow().clone() != 0 {
                    Some(ic.get_stack_top())
                } else {
                    None
                },
            };
            return Ok(Value::Function(Handle::from(f)));
        } else if **x == "fun".to_string() {
            if expr.len() < 4 {
                return Err(exception("PredicateError", "'lambda' parameters number is less 3.\n\thelp: (lambda <tuple> [<tuple>]*)"));
            }
            let name = if let Value::Symbol(name) = expr[1].clone() {
                name
            } else {
                return Err(exception("PredicateError", "'lambda' defined function parameters list tiem is not symbol."));
            };

            let mut argn: Vec<String> = vec![];
            if let Value::Tuple(x) = expr[2].clone() {
                for i in x.iter() {
                    if let Value::Symbol(x) = i {
                        argn.push(x.to_string());
                    } else {
                        return Err(exception("PredicateError", "'lambda' defined function parameters list tiem is not symbol."));
                    }
                }
            } else {
                return Err(exception("PredicateError", "'lambda' defined function parameters list is not tuple"));
            }
            let mut e: Vec<Value> = vec![];
            for (i, v) in expr.iter().enumerate() {
                if i > 2 {
                    e.push(v.clone());
                }
            }
            let f = Function {
                modu: Arc::downgrade(&ic.using_mod.borrow()),
                name: (*name).clone(),
                expr: e,
                argn,
                env: if ic.frame_size.borrow().clone() != 0 {
                    Some(ic.get_stack_top())
                } else {
                    None
                },
            };
            let fv = Value::Function(Handle::from(f));
            let_value(ic, name, fv.clone())?;
            //set_value(ic, name, fv.clone());
            return Ok(fv);
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