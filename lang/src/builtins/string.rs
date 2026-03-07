use crate::vm::value::{NativeFn, Value};

fn native(name: &str, func: fn(Value, Vec<Value>) -> Result<(Value, Option<Value>), String>) -> NativeFn {
    NativeFn { name: name.to_string(), func }
}

pub fn get_fn(name: String) -> Option<NativeFn> {
    match name.as_str() {
        "len" => Some(native("len", len)),
        _ => None,
    }
}

pub fn len(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("len() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::String(s) => Ok((Value::Int(s.chars().count() as i64), None)),
        _ => unreachable!(),
    }
}