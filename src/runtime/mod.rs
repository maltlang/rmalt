use value::Value;
use std::sync::Arc;
use value::MaltResult;

pub mod context;

impl Value {
    // 慎用，这玩意会把list当成调用来搞
    fn eval(&self, ic: &ThreadContext) -> MaltResult {

    }
}