/*
use func::Call;

*/

use std::sync::Arc;
use core::interpreter::ThreadContext;
use value::Value;
use value::Eval;
use func::Call;

fn list_eval(list: Arc<Vec<Value>>, ic: &Arc<ThreadContext>) -> Value {
    if list.len() == 0 {
        return Value::Nil;
    }
    if let Value::Symbol(ref x) = list[0] {
        /* quote 用宏来做吧
        if x == String::from("quote") {
            if list.len() == 2 {
                list[1]
            } else {
                // 粗错处理
                // 无效的quote表达式
            }
        } else
        */
        // 唯一的条件判断表达式
        if &**x == "cond" {
            //TODO:先写到这里，明天继续写，来不起了
        } else {
            return Value::Nil;
        }
    }
    // fcall的各种求值
    let head = list[0].eval(ic);
    let mut body: Vec<Value> = Vec::new();
    for v in list[1..].iter() {
        body.push(v.eval(&ic));
    }
    if let Value::Function(ref x) = head {
        x.call_function(&ic, Arc::from(body))
    } else if let Value::Native(ref x) = head {
        x.call_function(&ic, Arc::from(body))
    } else {
        // 出错啦，但还没有好的处理方案
        Value::Nil
    }
}

impl Eval for Value {
    fn eval(&self, ic: &Arc<ThreadContext>) -> Value {
        if let Value::Symbol(ref x) = self {
            // 实际上需要load symbol
            return Value::Symbol(x.clone());
        } else if let Value::Tuple(ref x) = self {
            return list_eval(x.clone(), ic);
        } else if let Value::Object(ref x) = self {
            if let Some(y) = x.get("__Eval__") {
                if let Value::Function(ref z) = y {
                    return z.call_function(ic, Arc::from(vec![]));
                } else if let Value::Native(ref z) = y {
                    return z.call_function(ic, Arc::from(vec![]));
                }
            }
        }
        self.clone()
    }
}