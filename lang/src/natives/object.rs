use crate::vm::value::{NativeFn, Value};

pub fn get_fn(name: String) -> Option<NativeFn> {
    match name.as_str() {
        "get" => Some(NativeFn::new("get", get)),
        "set" => Some(NativeFn::new("set", set)),
        "has" => Some(NativeFn::new("has", has)),
        "delete" => Some(NativeFn::new("delete", delete)),
        "stringify" => Some(NativeFn::new("stringify", stringify)),
        "keys" => Some(NativeFn::new("keys", keys)),
        "values" => Some(NativeFn::new("values", values)),
        _ => None,
    }
}

pub fn list_fns() -> Vec<String> {
    vec![
        "get".to_string(),
        "set".to_string(),
        "has".to_string(),
        "delete".to_string(),
        "stringify".to_string(),
        "keys".to_string(),
        "values".to_string(),
    ]
}

pub fn get(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("<object>.get() takes exactly one argument ({} given)", args.len()));
    }

    let key = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("<object>.get() key must be a string, got {}", args[0].type_name())),
    };

    match this {
        Value::Object(obj) => Ok((obj.get(key).cloned().unwrap_or(Value::Null), None)),
        _ => unreachable!(),
    }
}

pub fn set(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 2 {
        return Err(format!("<object>.set() takes exactly two arguments ({} given)", args.len()));
    }

    let key = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("<object>.set() key must be a string, got {}", args[0].type_name())),
    };

    match this {
        Value::Object(mut obj) => {
            obj.insert(key.clone(), args[1].clone());
            Ok((Value::Null, Some(Value::Object(obj))))
        }

        _ => unreachable!(),
    }
}

pub fn has(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("<object>.has() takes exactly one argument ({} given)", args.len()));
    }

    let key = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("<object>.has() key must be a string, got {}", args[0].type_name())),
    };

    match this {
        Value::Object(obj) => Ok((Value::Bool(obj.contains_key(key)), None)),
        _ => unreachable!(),
    }
}

pub fn delete(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("<object>.delete() takes exactly one argument ({} given)", args.len()));
    }

    let key = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("<object>.delete() key must be a string, got {}", args[0].type_name())),
    };

    match this {
        Value::Object(mut obj) => {
            let removed = obj.remove(key).is_some();
            Ok((Value::Bool(removed), Some(Value::Object(obj))))
        }

        _ => unreachable!(),
    }
}

pub fn stringify(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<object>.stringify() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Object(obj) => {
            let mut parts = Vec::new();
            for (k, v) in obj.iter() {
                match v {
                    Value::String(s) => parts.push(format!("\"{}\": \"{}\"", k, s)),
                    _ => parts.push(format!("\"{}\": {}", k, v)),
                }
            }
            
            Ok((Value::String(format!("{{ {} }}", parts.join(", "))), None))
        }
        _ => unreachable!(),
    }
}

pub fn keys(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<object>.keys() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Object(obj) => {
            let keys: Vec<Value> = obj.keys().cloned().map(Value::String).collect();
            Ok((Value::Array(keys), None))
        }

        _ => unreachable!(),
    }
}

pub fn values(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<object>.values() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Object(obj) => {
            let values: Vec<Value> = obj.values().cloned().collect();
            Ok((Value::Array(values), None))
        }

        _ => unreachable!(),
    }
}