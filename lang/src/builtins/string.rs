use crate::vm::value::{NativeFn, Value};

fn native(name: &str, func: fn(Value, Vec<Value>) -> Result<(Value, Option<Value>), String>) -> NativeFn {
    NativeFn { name: name.to_string(), func }
}

pub fn get_fn(name: String) -> Option<NativeFn> {
    match name.as_str() {
        "len" => Some(native("len", len)),
        "split" => Some(native("split", split)),
        "replace" => Some(native("replace", replace)),
        "trim" => Some(native("trim", trim)),
        "to_upper" => Some(native("to_upper", to_upper)),
        "to_lower" => Some(native("to_lower", to_lower)),
        "starts_with" => Some(native("starts_with", starts_with)),
        "ends_with" => Some(native("ends_with", ends_with)),
        "chars" => Some(native("chars", chars)),
        _ => None,
    }
}

pub fn len(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("len() takaes no arguments ({} given)", args.len()));
    }

    match this {
        Value::String(s) => Ok((Value::Int(s.chars().count() as i64), None)),
        _ => unreachable!(),
    }
}

pub fn split(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("split() takes exactly one argument ({} given)", args.len()));
    }

    let sep = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("split() delimiter must be a string, got {}", args[0].type_name())),
    };

    match this {
        Value::String(s) => {
            let parts = s.split(sep).map(|part| Value::String(part.to_string())).collect();
            Ok((Value::Array(parts), None))
        }

        _ => unreachable!(),
    }
}

pub fn replace(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 2 {
        return Err(format!("replace() takes exactly two arguments ({} given)", args.len()));
    }

    let old = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("replace() old value must be a string, got {}", args[0].type_name())),
    };

    let new = match &args[1] {
        Value::String(s) => s,
        _ => return Err(format!("replace() new value must be a string, got {}", args[1].type_name())),
    };

    match this {
        Value::String(s) => Ok((Value::String(s.replace(old, new)), None)),
        _ => unreachable!(),
    }
}

pub fn trim(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("trim() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::String(s) => Ok((Value::String(s.trim().to_string()), None)),
        _ => unreachable!(),
    }
}

pub fn to_upper(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("to_upper() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::String(s) => Ok((Value::String(s.to_uppercase()), None)),
        _ => unreachable!(),
    }
}

pub fn to_lower(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("to_lower() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::String(s) => Ok((Value::String(s.to_lowercase()), None)),
        _ => unreachable!(),
    }
}

pub fn starts_with(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("starts_with() takes exactly one argument ({} given)", args.len()));
    }

    let prefix = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("starts_with() prefix must be a string, got {}", args[0].type_name())),
    };

    match this {
        Value::String(s) => Ok((Value::Bool(s.starts_with(prefix)), None)),
        _ => unreachable!(),
    }
}

pub fn ends_with(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("ends_with() takes exactly one argument ({} given)", args.len()));
    }

    let suffix = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("ends_with() suffix must be a string, got {}", args[0].type_name())),
    };

    match this {
        Value::String(s) => Ok((Value::Bool(s.ends_with(suffix)), None)),
        _ => unreachable!(),
    }
}

pub fn chars(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("chars() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::String(s) => {
            let chars = s.chars().map(|c| Value::String(c.to_string())).collect();
            Ok((Value::Array(chars), None))
        }

        _ => unreachable!(),
    }
}