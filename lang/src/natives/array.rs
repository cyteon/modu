use crate::vm::value::{NativeFn, Value};

pub fn get_fn(name: String) -> Option<NativeFn> {
    match name.as_str() {
        "len" => Some(NativeFn::new("len", len)),
        "push" => Some(NativeFn::new("push", push)),
        "pop" => Some(NativeFn::new("pop", pop)),
        "join" => Some(NativeFn::new("join", join)),
        _ => None,
    }
}

pub fn list_fns() -> Vec<String> {
    vec!["len".to_string(), "push".to_string(), "pop".to_string(), "join".to_string()]
}

pub fn len(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<array>.len() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Array(arr) => Ok((Value::Int(arr.len() as i64), None)),
        _ => unreachable!(),
    }
}

pub fn push(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("<array>.push() takes exactly one argument ({} given)", args.len()));
    }

    match this {
        Value::Array(mut arr) => {
            arr.push(args[0].clone());
            Ok((Value::Null, Some(Value::Array(arr))))
        }

        _ => unreachable!(),
    }
}

pub fn pop(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<array>.pop() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Array(mut arr) => {
            let popped = arr.pop().unwrap_or(Value::Null);
            Ok((popped, Some(Value::Array(arr))))
        }

        _ => unreachable!(),
    }
}

pub fn join(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("<array>.join() takes exactly one argument ({} given)", args.len()));
    }

    let sep = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("<array>.join() separator must be a string, got {}", args[0].type_name())),
    };

    match this {
        Value::Array(arr) => {
            let joined = arr.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(sep);
            Ok((Value::String(joined), None))
        }

        _ => unreachable!(),
    }
}