use std::rc::Weak;

use anyhow::Result;

use crate::{FnType, Value, W};

pub const TAN_SYM: &str = "tan";

pub fn tan() -> Value {
    Value::Closure {
        fn_type: FnType::Builtin,
        sym: TAN_SYM.into(),
        prms: vec!["x".into()],
        addr: 0,
        env: W(Weak::new()),
    }
}

pub fn tan_impl(x: &Value) -> Result<Value> {
    let x: f64 = x.clone().try_into()?;
    Ok(Value::Float(x.tan()))
}
