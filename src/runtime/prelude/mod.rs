use std::io;
use std::process::exit;
use std::cell::RefCell;
use std::collections::HashMap;

use func::Native;
use value::Value;
use value::Handle;
use value::MaltResult;

use runtime::args_length_exception;
use runtime::context::ModuleContext;
use runtime::tools::exception;
use std::io::Write;

pub fn system_module() -> ModuleContext {
    let mut vt: HashMap<String, Value> = HashMap::new();
    vt.insert(String::from("-module-name"), Value::Symbol(Handle::from(String::from("Prelude"))));

    vt.insert(String::from("-lang/version"), Value::Tuple(Handle::from(
        vec![Value::UInt(1), Value::UInt(0)])));

    // envs
    if cfg!(target_os = "linux") {
        vt.insert(String::from("-target/os"), Value::Symbol(Handle::from(String::from("linux"))));
    } else if cfg!(target_os = "windows") {
        vt.insert(String::from("-target/os"), Value::Symbol(Handle::from(String::from("windows"))));
    } else if cfg!(target_os = "android") {
        vt.insert(String::from("-target/os"), Value::Symbol(Handle::from(String::from("android"))));
    } else if cfg!(target_os = "macos") {
        vt.insert(String::from("-target/os"), Value::Symbol(Handle::from(String::from("macos"))));
    } else if cfg!(target_os = "ios") {
        vt.insert(String::from("-target/os"), Value::Symbol(Handle::from(String::from("ios"))));
    } else {
        vt.insert(String::from("-target/os"), Value::Symbol(Handle::from(String::from("unknown"))));
    }
    if cfg!(target_arch = "x86") {
        vt.insert(String::from("-target/arch"), Value::Symbol(Handle::from(String::from("x86"))));
    } else if cfg!(target_arch = "x86_64") {
        vt.insert(String::from("-target/arch"), Value::Symbol(Handle::from(String::from("x86_64"))));
    } else if cfg!(target_arch = "arm") {
        vt.insert(String::from("-target/arch"), Value::Symbol(Handle::from(String::from("arm"))));
    } else if cfg!(target_arch = "powerpc") {
        vt.insert(String::from("-target/arch"), Value::Symbol(Handle::from(String::from("powerpc"))));
    } else if cfg!(target_arch = "powerpc64") {
        vt.insert(String::from("-target/arch"), Value::Symbol(Handle::from(String::from("powerpc64"))));
    } else if cfg!(target_arch = "aarch64") {
        vt.insert(String::from("-target/arch"), Value::Symbol(Handle::from(String::from("aarch64"))));
    } else {
        vt.insert(String::from("-target/arch"), Value::Symbol(Handle::from(String::from("unknown"))));
    }
    if cfg!(target_env = "gnu") {
        vt.insert(String::from("-target/env"), Value::Symbol(Handle::from(String::from("gnu"))));
    } else if cfg!(target_env = "msvc") {
        vt.insert(String::from("-target/env"), Value::Symbol(Handle::from(String::from("msvc"))));
    } else if cfg!(target_env = "musl") {
        vt.insert(String::from("-target/env"), Value::Symbol(Handle::from(String::from("musl"))));
    } else {
        vt.insert(String::from("-target/env"), Value::Symbol(Handle::from(String::from("unknown"))));
    }

    if cfg!(target_family = "unix") {
        vt.insert(String::from("-target/family"), Value::Symbol(Handle::from(String::from("unix"))));
    } else if cfg!(target_family = "windows") {
        vt.insert(String::from("-target/family"), Value::Symbol(Handle::from(String::from("windows"))));
    } else {
        vt.insert(String::from("-target/family"), Value::Symbol(Handle::from(String::from("Other"))));
    }


    vt.insert(String::from("nil"), Value::Nil);

    vt.insert(String::from("true"), Value::Bool(true));
    vt.insert(String::from("false"), Value::Bool(false));

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

    /*
    vt.insert(String::from("print!"), Value::Native(Handle::from(Native {
        name: String::from("print!"),
        fp: |_ic, args| {
            for v in args.iter() {
                io::stdout().write(v.to_string().as_bytes()).expect("*** io-error");
            }
            Ok(Value::Nil)
        },
    })));
    */

    vt.insert(String::from("input!"), Value::Native(Handle::from(Native {
        name: String::from("input!"),
        fp: |_ic, args| {
            for v in args.iter() {
                let _ = io::stdout().write(v.to_string().as_ref());
                let _ = io::stdout().flush();
            }
            let mut input = String::new();
            io::stdin().read_line(&mut input).
                unwrap();
            Ok(Value::String(Handle::from(input)))
        },
    })));

    vt.insert(String::from("print!"), Value::Native(Handle::from(Native {
        name: String::from("print!"),
        fp: |_ic, args| {
            for v in args.iter() {
                let _ = io::stdout().write(v.to_string().as_ref());
                let _ = io::stdout().flush();
            }
            Ok(Value::Nil)
        },
    })));

    vt.insert(String::from("println!"), Value::Native(Handle::from(Native {
        name: String::from("println!"),
        fp: |_ic, args| {
            for v in args.iter() {
                let _ = io::stdout().write(v.to_string().as_ref());
                let _ = io::stdout().write("\n".as_ref());
                let _ = io::stdout().flush();
            }
            Ok(Value::Nil)
        },
    })));

    vt.insert(String::from("+"), Value::Native(Handle::from(Native {
        name: String::from("+"),
        fp: |_ic, args| {
            let mut s = Value::UInt(0);
            if args.len() == 0 {
                Ok(s)
            } else {
                for (i, v) in args.iter().enumerate() {
                    if i == 0 {
                        s = v.clone();
                    } else {
                        if !v.is_number() {
                            return Err(exception("TypeError", "+ oper parameters type is not number"));
                        }
                        s = ex_add_once(s, v);
                    }
                }
                Ok(s)
            }
        },
    })));
    vt.insert(String::from("-"), Value::Native(Handle::from(Native {
        name: String::from("-"),
        fp: |_ic, args| {
            let mut s = Value::UInt(0);
            if args.len() == 0 {
                Ok(s)
            } else {
                for (i, v) in args.iter().enumerate() {
                    if i == 0 {
                        s = v.clone();
                    } else {
                        if !v.is_number() {
                            return Err(exception("TypeError", "+ oper parameters type is not number"));
                        }
                        s = ex_sub_once(s, v);
                    }
                }
                Ok(s)
            }
        },
    })));

    vt.insert(String::from("*"), Value::Native(Handle::from(Native {
        name: String::from("*"),
        fp: |_ic, args| {
            let mut s = Value::UInt(0);
            if args.len() == 0 {
                Ok(s)
            } else {
                for (i, v) in args.iter().enumerate() {
                    if i == 0 {
                        s = v.clone();
                    } else {
                        if !v.is_number() {
                            return Err(exception("TypeError", "+ oper parameters type is not number"));
                        }
                        s = ex_mul_once(s, v);
                    }
                }
                Ok(s)
            }
        },
    })));

    vt.insert(String::from("/"), Value::Native(Handle::from(Native {
        name: String::from("/"),
        fp: |_ic, args| {
            let mut s = Value::UInt(0);
            if args.len() == 0 {
                Ok(s)
            } else {
                for (i, v) in args.iter().enumerate() {
                    if i == 0 {
                        s = v.clone();
                    } else {
                        if !v.is_number() {
                            return Err(exception("TypeError", "+ oper parameters type is not number"));
                        }
                        s = ex_div_once(s, v)?;
                    }
                }
                Ok(s)
            }
        },
    })));

    vt.insert(String::from("and"), Value::Native(Handle::from(Native {
        name: String::from("and"),
        fp: |_ic, args| {
            for i in &*args {
                if let Value::Bool(x) = i {
                    if !*x {
                        return Ok(Value::Bool(false));
                    }
                } else {
                    return Err(exception("TypeError", "function 'and' parmeter type is not bool"));
                }
            }
            Ok(Value::Bool(true))
        },
    })));

    vt.insert(String::from("or"), Value::Native(Handle::from(Native {
        name: String::from("or"),
        fp: |_ic, args| {
            for i in &*args {
                if let Value::Bool(x) = i {
                    if *x {
                        return Ok(Value::Bool(true));
                    }
                } else {
                    return Err(exception("TypeError", "function 'or' parmeter type is not bool"));
                }
            }
            Ok(Value::Bool(false))
        },
    })));

    vt.insert(String::from("not"), Value::Native(Handle::from(Native {
        name: String::from("not"),
        fp: |_ic, args| {
            if args.len() != 1 {
                return Ok(Value::Bool(false));
            }
            if let Value::Bool(x) = args[0].clone() {
                return Ok(Value::Bool(!x));
            } else {
                return Err(exception("TypeError", "function 'not' parmeter type is not bool"));
            }
        },
    })));

    ModuleContext {
        path: String::from("Prelude"),
        expr: Vec::new(),
        vtab: RefCell::from(vt),
    }
}

fn ex_add_once(s: Value, v: &Value) -> Value {
    match (s.clone(), v) {
        (Value::UInt(x), Value::UInt(y)) => {
            Value::Int((x + *y) as i64)
        }
        (Value::UInt(x), Value::Int(y)) => {
            Value::Int((x as i64) + *y)
        }
        (Value::UInt(x), Value::Float(y)) => {
            Value::Float((x as f64) + *y)
        }
        (Value::Int(x), Value::UInt(y)) => {
            Value::Int(x + (*y as i64))
        }
        (Value::Int(x), Value::Int(y)) => {
            Value::Int(x + *y)
        }
        (Value::Int(x), Value::Float(y)) => {
            Value::Float((x as f64) + *y)
        }
        (Value::Float(x), Value::UInt(y)) => {
            Value::Float(x + (*y as f64))
        }
        (Value::Float(x), Value::Int(y)) => {
            Value::Float(x + (*y as f64))
        }
        (Value::Float(x), Value::Float(y)) => {
            Value::Float(x + (*y as f64))
        }
        _ => {
            s
        }
    }
}

fn ex_sub_once(s: Value, v: &Value) -> Value {
    match (s.clone(), v) {
        (Value::UInt(x), Value::UInt(y)) => {
            Value::Int((x - *y) as i64)
        }
        (Value::UInt(x), Value::Int(y)) => {
            Value::Int((x as i64) - *y)
        }
        (Value::UInt(x), Value::Float(y)) => {
            Value::Float((x as f64) - *y)
        }
        (Value::Int(x), Value::UInt(y)) => {
            Value::Int(x - (*y as i64))
        }
        (Value::Int(x), Value::Int(y)) => {
            Value::Int(x - *y)
        }
        (Value::Int(x), Value::Float(y)) => {
            Value::Float((x as f64) - *y)
        }
        (Value::Float(x), Value::UInt(y)) => {
            Value::Float(x - (*y as f64))
        }
        (Value::Float(x), Value::Int(y)) => {
            Value::Float(x - (*y as f64))
        }
        (Value::Float(x), Value::Float(y)) => {
            Value::Float(x - (*y as f64))
        }
        _ => {
            s
        }
    }
}

fn ex_mul_once(s: Value, v: &Value) -> Value {
    match (s.clone(), v) {
        (Value::UInt(x), Value::UInt(y)) => {
            Value::Int((x * *y) as i64)
        }
        (Value::UInt(x), Value::Int(y)) => {
            Value::Int((x as i64) * *y)
        }
        (Value::UInt(x), Value::Float(y)) => {
            Value::Float((x as f64) * *y)
        }
        (Value::Int(x), Value::UInt(y)) => {
            Value::Int(x * (*y as i64))
        }
        (Value::Int(x), Value::Int(y)) => {
            Value::Int(x * *y)
        }
        (Value::Int(x), Value::Float(y)) => {
            Value::Float((x as f64) * *y)
        }
        (Value::Float(x), Value::UInt(y)) => {
            Value::Float(x * (*y as f64))
        }
        (Value::Float(x), Value::Int(y)) => {
            Value::Float(x * (*y as f64))
        }
        (Value::Float(x), Value::Float(y)) => {
            Value::Float(x * (*y as f64))
        }
        _ => {
            s
        }
    }
}

fn ex_div_once(s: Value, v: &Value) -> MaltResult {
    Ok(match (s.clone(), v) {
        (Value::UInt(x), Value::UInt(y)) => {
            if *y == 0 {
                return Err(exception("ZeroDivisionError", "division by zero"));
            }
            Value::Int((x / *y) as i64)
        }
        (Value::UInt(x), Value::Int(y)) => {
            Value::Int((x as i64) / *y)
        }
        (Value::UInt(x), Value::Float(y)) => {
            Value::Float((x as f64) / *y)
        }
        (Value::Int(x), Value::UInt(y)) => {
            Value::Int(x / (*y as i64))
        }
        (Value::Int(x), Value::Int(y)) => {
            Value::Int(x / *y)
        }
        (Value::Int(x), Value::Float(y)) => {
            Value::Float((x as f64) / *y)
        }
        (Value::Float(x), Value::UInt(y)) => {
            Value::Float(x / (*y as f64))
        }
        (Value::Float(x), Value::Int(y)) => {
            Value::Float(x / (*y as f64))
        }
        (Value::Float(x), Value::Float(y)) => {
            Value::Float(x / (*y as f64))
        }
        _ => {
            s
        }
    })
}