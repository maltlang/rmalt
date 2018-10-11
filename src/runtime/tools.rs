// 以后可能会拆成模块
use std::collections::HashMap;

use value::Value;
use value::Handle;
use runtime::context::ThreadContext;
use value::_Str;
//use std::sync::mpsc::Handle;


pub fn exception(class: &str, info: &str) -> Value {
    let mut r: HashMap<String, Value> = HashMap::new();
    r.insert(String::from("__class__"), Value::String(Handle::from(String::from(class))));
    r.insert(String::from("__info__"), Value::String(Handle::from(String::from(info))));
    Value::Object(Handle::from(r))
}

pub fn set_value(ic: &ThreadContext, sym: _Str, expr: Value) {
    if ic.frame_size.borrow().clone() != 0 {
        // 表示在函数作用域
        let sfc = ic.get_stack_top();
        sfc.vtab.borrow_mut().insert(sym.to_string(), expr);
    } else {
        // 表示在顶层作用域
        let c = ic.using_mod.borrow();
        c.vtab.borrow_mut().insert(sym.to_string(), expr);
    }
}

pub fn let_value(ic: &ThreadContext, sym: _Str, expr: Value) -> Result<(), Value> {
    if ic.frame_size.borrow().clone() != 0 {
        let sfc = ic.get_stack_top();
        if let Some(_) = sfc.vtab.borrow().get(sym.as_ref()) {
            return Err(exception("LetError", &("In Function '".to_string() +
                &sfc.fun.modu.upgrade().unwrap().path +
                "::" +
                &sfc.fun.name +
                "' repeat let")));
        }
        // 表示在函数作用域
        sfc.vtab.borrow_mut().insert(sym.to_string(), expr);
    } else {
        // 表示在顶层作用域
        let c = ic.using_mod.borrow();
        if let Some(_) = c.vtab.borrow().get(sym.as_ref()) {
            return Err(exception("LetError", &("In Module '".to_string() + &c.path + "' Repeat let value to '" + &sym + "'.")));
        }
        c.vtab.borrow_mut().insert(sym.to_string(), expr);
    }
    Ok(())
}


/// libs
pub fn num_to_uint(n: Value) -> Option<u64> {
    match n {
        Value::Float(x) => Some(x as u64),
        Value::UInt(x) => Some(x as u64),
        Value::Int(x) => Some(x as u64),
        _ => None
    }
}

pub fn num_to_int(n: Value) -> Option<i64> {
    match n {
        Value::Float(x) => Some(x as i64),
        Value::UInt(x) => Some(x as i64),
        Value::Int(x) => Some(x as i64),
        _ => None
    }
}

pub fn num_to_float(n: Value) -> Option<f64> {
    match n {
        Value::Float(x) => Some(x as f64),
        Value::UInt(x) => Some(x as f64),
        Value::Int(x) => Some(x as f64),
        _ => None
    }
}