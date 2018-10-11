use std::cell::RefCell;
use std::collections::HashMap;
use std::process::exit;

use func::Native;
use value::Value;
use value::Handle;
use value::MaltResult;

use runtime::args_length_exception;
use runtime::context::ModuleContext;
use runtime::tools::exception;
use runtime::tools::num_to_float;
use runtime::tools::num_to_uint;
use runtime::tools::num_to_int;

pub fn system_module() -> ModuleContext {
    let mut vt: HashMap<String, Value> = HashMap::new();
    //vt.insert(String::from("--name--"), Value::Symbol(Handle::from(String::from("System"))));

    vt.insert(String::from("true"), Value::Bool(true));
    vt.insert(String::from("false"), Value::Bool(false));

    //FIXME: 移动到envinfo
    /*
    vt.insert(String::from("--version--"), Value::Native(Handle::from(Native {
        name: String::from("--version--"),
        fp: |_ic, _args| {
            let a = vec![Value::UInt(0), Value::UInt(0)];
            Ok(Value::Tuple(Handle::from(a)))
        },
    })));
    */

    // tool libs
    vt.insert(String::from("exit!"), Value::Native(Handle::from(Native {
        name: String::from("exit!"),
        fp: |_ic, args| {
            if args.len() == 0 {
                exit(0);
            } else if args.len() == 1 {
                if let Ok(x) = args[0].to_int() {
                    if let Value::Int(y) = x {
                        exit(y as i32);
                    }
                    // 这里不可能出现非Int的情况
                } else {
                    return Err(exception("TypeError", "'exit!' function call parameters type error"));
                }
            }
            return Err(args_length_exception());
        },
    })));

    vt.insert(String::from("add"), Value::Native(Handle::from(Native {
        name: String::from("add"),
        fp: |_ic, args| {
            if args.len() != 2 {
                return Err(args_length_exception());
            }
            add(args[0].clone(), args[1].clone())
        },
    })));
    vt.insert(String::from("sub"), Value::Native(Handle::from(Native {
        name: String::from("sub"),
        fp: |_ic, args| {
            if args.len() != 2 {
                return Err(args_length_exception());
            }
            sub(args[0].clone(), args[1].clone())
        },
    })));
    vt.insert(String::from("mul"), Value::Native(Handle::from(Native {
        name: String::from("mul"),
        fp: |_ic, args| {
            if args.len() != 2 {
                return Err(args_length_exception());
            }
            mul(args[0].clone(), args[1].clone())
        },
    })));
    vt.insert(String::from("div"), Value::Native(Handle::from(Native {
        name: String::from("div"),
        fp: |_ic, args| {
            if args.len() != 2 {
                return Err(args_length_exception());
            }
            div(args[0].clone(), args[1].clone())
        },
    })));

    // return
    ModuleContext {
        path: String::from("System"),
        expr: Vec::new(),
        vtab: RefCell::from(vt),
    }
}

fn add(a: Value, b: Value) -> MaltResult {
    if a.get_type() != b.get_type() {
        return Err(exception("TypeError", "function call parameters type error"));
    }
    if let Value::Float(x) = a {
        if let Some(a) = num_to_float(b) {
            return Ok(Value::Float(x + a));
        } else {
            return Err(exception("TypeError", "function call parameters type error"));
        }
    } else if let Value::UInt(x) = a {
        if let Some(a) = num_to_uint(b) {
            return Ok(Value::UInt(x + a));
        } else {
            return Err(exception("TypeError", "function call parameters type error"));
        }
    } else if let Value::Int(x) = a {
        if let Some(a) = num_to_int(b) {
            return Ok(Value::Int(x + a));
        } else {
            return Err(exception("TypeError", "function call parameters type error"));
        }
    } else {
        return Err(exception("TypeError", "function call parameters type error"));
    }
}

fn sub(a: Value, b: Value) -> MaltResult {
    if a.get_type() != b.get_type() {
        return Err(exception("TypeError", "function call parameters type error"));
    }
    if let Value::Float(x) = a {
        if let Some(a) = num_to_float(b) {
            return Ok(Value::Float(x - a));
        } else {
            return Err(exception("TypeError", "function call parameters type error"));
        }
    } else if let Value::UInt(x) = a {
        if let Some(a) = num_to_uint(b) {
            return Ok(Value::UInt(x - a));
        } else {
            return Err(exception("TypeError", "function call parameters type error"));
        }
    } else if let Value::Int(x) = a {
        if let Some(a) = num_to_int(b) {
            return Ok(Value::Int(x - a));
        } else {
            return Err(exception("TypeError", "function call parameters type error"));
        }
    } else {
        return Err(exception("TypeError", "function call parameters type error"));
    }
}

fn mul(a: Value, b: Value) -> MaltResult {
    if a.get_type() != b.get_type() {
        return Err(exception("TypeError", "function call parameters type error"));
    }
    if let Value::Float(x) = a {
        if let Some(a) = num_to_float(b) {
            return Ok(Value::Float(x * a));
        } else {
            return Err(exception("TypeError", "function call parameters type error"));
        }
    } else if let Value::UInt(x) = a {
        if let Some(a) = num_to_uint(b) {
            return Ok(Value::UInt(x * a));
        } else {
            return Err(exception("TypeError", "function call parameters type error"));
        }
    } else if let Value::Int(x) = a {
        if let Some(a) = num_to_int(b) {
            return Ok(Value::Int(x * a));
        } else {
            return Err(exception("TypeError", "function call parameters type error"));
        }
    } else {
        return Err(exception("TypeError", "function call parameters type error"));
    }
}

//FIXME: 没做零除数断言和被除数返回
fn div(a: Value, b: Value) -> MaltResult {
    if a.get_type() != b.get_type() {
        return Err(exception("TypeError", "function call parameters type error"));
    }
    if let Value::Float(x) = a {
        if let Some(a) = num_to_float(b) {
            return Ok(Value::Float(x / a));
        } else {
            return Err(exception("TypeError", "function call parameters type error"));
        }
    } else if let Value::UInt(x) = a {
        if let Some(a) = num_to_uint(b) {
            return Ok(Value::UInt(x / a));
        } else {
            return Err(exception("TypeError", "function call parameters type error"));
        }
    } else if let Value::Int(x) = a {
        if let Some(a) = num_to_int(b) {
            return Ok(Value::Int(x / a));
        } else {
            return Err(exception("TypeError", "function call parameters type error"));
        }
    } else {
        return Err(exception("TypeError", "function call parameters type error"));
    }
}