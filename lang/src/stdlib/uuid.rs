use std::collections::HashMap;
use crate::vm::value::{BuiltinFn, Value};

pub fn object() -> Value {
    let mut methods = HashMap::new();

    methods.insert("v4".to_string(), Value::BuiltinFn(BuiltinFn::new("v4", v4)));
    methods.insert("v7".to_string(), Value::BuiltinFn(BuiltinFn::new("v7", v7)));

    Value::Object(methods)
}

pub fn list_fns() -> Vec<String> {
    vec![
        "v4".to_string(),
        "v7".to_string(),
    ]
}

fn v4(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(format!("uuid.v4() takes no arguments ({} given)", args.len()));
    }

    let uuid = uuid::Uuid::new_v4();

    Ok(Value::String(uuid.to_string()))
}

fn v7(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(format!("uuid.v7() takes no arguments ({} given)", args.len()));
    }

    let uuid = uuid::Uuid::now_v7();

    Ok(Value::String(uuid.to_string()))
}