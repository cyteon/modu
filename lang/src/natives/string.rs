use crate::vm::value::{NativeFn, Value};

pub fn get_fn(name: String) -> Option<NativeFn> {
    match name.as_str() {
        "len" => Some(NativeFn::new("len", len)),
        "split" => Some(NativeFn::new("split", split)),
        "replace" => Some(NativeFn::new("replace", replace)),
        "trim" => Some(NativeFn::new("trim", trim)),
        "to_upper" => Some(NativeFn::new("to_upper", to_upper)),
        "to_lower" => Some(NativeFn::new("to_lower", to_lower)),
        "starts_with" => Some(NativeFn::new("starts_with", starts_with)),
        "ends_with" => Some(NativeFn::new("ends_with", ends_with)),
        "chars" => Some(NativeFn::new("chars", chars)),
        _ => None,
    }
}

pub fn len(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<string>.len() takaes no arguments ({} given)", args.len()));
    }

    match this {
        Value::String(s) => Ok((Value::Int(s.chars().count() as i64), None)),
        _ => unreachable!(),
    }
}

pub fn split(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("<string>.split() takes exactly one argument ({} given)", args.len()));
    }

    let sep = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("<string>.split() delimiter must be a string, got {}", args[0].type_name())),
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
        return Err(format!("<string>.replace() takes exactly two arguments ({} given)", args.len()));
    }

    let old = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("<string>.replace() old value must be a string, got {}", args[0].type_name())),
    };

    let new = match &args[1] {
        Value::String(s) => s,
        _ => return Err(format!("<string>.replace() new value must be a string, got {}", args[1].type_name())),
    };

    match this {
        Value::String(s) => Ok((Value::String(s.replace(old, new)), None)),
        _ => unreachable!(),
    }
}

pub fn trim(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<string>.trim() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::String(s) => Ok((Value::String(s.trim().to_string()), None)),
        _ => unreachable!(),
    }
}

pub fn to_upper(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<string>.to_upper() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::String(s) => Ok((Value::String(s.to_uppercase()), None)),
        _ => unreachable!(),
    }
}

pub fn to_lower(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<string>.to_lower() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::String(s) => Ok((Value::String(s.to_lowercase()), None)),
        _ => unreachable!(),
    }
}

pub fn starts_with(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("<string>.starts_with() takes exactly one argument ({} given)", args.len()));
    }

    let prefix = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("<string>.starts_with() prefix must be a string, got {}", args[0].type_name())),
    };

    match this {
        Value::String(s) => Ok((Value::Bool(s.starts_with(prefix)), None)),
        _ => unreachable!(),
    }
}

pub fn ends_with(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("<string>.ends_with() takes exactly one argument ({} given)", args.len()));
    }

    let suffix = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("<string>.ends_with() suffix must be a string, got {}", args[0].type_name())),
    };

    match this {
        Value::String(s) => Ok((Value::Bool(s.ends_with(suffix)), None)),
        _ => unreachable!(),
    }
}

pub fn chars(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<string>.chars() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::String(s) => {
            let chars = s.chars().map(|c| Value::String(c.to_string())).collect();
            Ok((Value::Array(chars), None))
        }

        _ => unreachable!(),
    }
}