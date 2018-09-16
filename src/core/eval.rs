use ast::Ast;
use std::sync::Arc;
use core::interpreter::ThreadContext;
use value::Value;
use ast::AstValue;
use func::Call;

impl Ast {
    // 相当于quote
    pub fn to_value(&self) -> Value {
        match self.val {
            // 正常返回
            //AstValue::Nil => Value::Nil,
            AstValue::Bool(ref x) => Value::Bool(x.clone()),
            //AstValue::Char(ref x) => Value::Char(x.clone()),
            AstValue::Int(ref x) => Value::Int(x.clone()),
            AstValue::UInt(ref x) => Value::UInt(x.clone()),
            AstValue::Float(ref x) => Value::Float(x.clone()),
            AstValue::String(ref x) => Value::String(x.clone()),
            AstValue::Symbol(ref x) => Value::Symbol(x.clone()),
            // 转换成tuple返回
            AstValue::List(ref x) => {
                let mut r: Vec<Value> = vec![];
                for i in &x.list {
                    r.push(i.to_value());
                }
                Value::Tuple(Arc::from(r))
            }
        }
    }

    pub //fn eval(&self) -> Value {
    fn eval(&self, ic: &Arc<ThreadContext>) -> Value {
        match self.val {
            // list（各种call的处理）
            AstValue::List(ref x) => {
                // 判断长度，长度为零直接扔
                if x.list.len() == 0 {
                    return Value::Nil;
                } else {
                    // 判断类型，用很垃圾的tag
                    let mut is_macro = '\0'; // 'm' 'b' 'f'
                    let first_object = x.list[0].eval(ic);
                    match first_object {
                        Value::Macro(_) => is_macro = 'm',
                        Value::BaseMacro(_) => is_macro = 'b',
                        Value::Native(_) |
                        Value::Function(_) => is_macro = 'f',
                        _ => {
                            //tOdO: print stackframe info
                            eprintln!("stackframe info ...");
                            eprintln!("line pos info ...");
                            eprintln!("***Error: Invalid call expression")
                        }
                    }
                    // 根据各种情况处理
                    if is_macro == 'b' {
                        let mut a: Vec<Ast> = vec![];
                        for (i, o) in x.list.iter().enumerate() {
                            if i != 0 {
                                a.push(o.clone());
                            }
                        }
                        match first_object {
                            Value::BaseMacro(x) => x.unfold(ic, a),
                            _ => Value::Nil,
                        }
                    } else if is_macro == 'm' {
                        let mut a: Vec<Value> = vec![];
                        for (i, o) in x.list.iter().enumerate() {
                            if i != 0 {
                                a.push(o.to_value());
                            }
                        }
                        match first_object {
                            Value::Macro(x) => x.call(ic, Arc::from(a)),
                            _ => Value::Nil,
                        }
                    } else //if is_macro == 'f'
                    {
                        let mut a: Vec<Value> = vec![];
                        for (i, o) in x.list.iter().enumerate() {
                            if i != 0 {
                                a.push(o.eval(ic));
                            }
                        }
                        match first_object {
                            Value::Function(x) => x.call(ic, Arc::from(a)),
                            Value::Native(x) => x.call(ic, Arc::from(a)),
                            // 从逻辑上来说不会触发这一步，上同
                            _ => Value::Nil,
                        }
                    }
                }
                //return Value::Nil;//todo:delete
            }
            // 正常返回（嘛耶)
            _ => self.to_value(),
        }
    }
}