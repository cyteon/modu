use crate::vm::value::{NativeFn, Value};

pub fn get_fn(name: String) -> Option<NativeFn> {
    match name.as_str() {
        "len" => Some(NativeFn::new("len", len)),
        "push" => Some(NativeFn::new("push", push)),
        "pop" => Some(NativeFn::new("pop", pop)),
        "join" => Some(NativeFn::new("join", join)),
        "min" => Some(NativeFn::new("min", min)),
        "max" => Some(NativeFn::new("max", max)),
        "reverse" => Some(NativeFn::new("reverse", reverse)),
        "sort" => Some(NativeFn::new("sort", sort)),
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

pub fn min(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<array>.min() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Array(arr) => {
            let min_value = arr.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).cloned().unwrap_or(Value::Null);
            Ok((min_value, None))
        }

        _ => unreachable!(),
    }
}

pub fn max(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<array>.max() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Array(arr) => {
            let max_value = arr.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).cloned().unwrap_or(Value::Null);
            Ok((max_value, None))
        }

        _ => unreachable!(),
    }
}

pub fn reverse(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<array>.reverse() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Array(arr) => {
            let mut reversed = arr.clone();
            reversed.reverse();
            Ok((Value::Array(reversed), None))
        }

        _ => unreachable!(),
    }
}

pub fn sort(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<array>.sort() takes no arguments ({} given)", args.len()));
    }

    let mut err = None;

    match this {
        Value::Array(arr) => {
            let mut sorted = arr.clone();

            sorted.sort_by(|a, b| {
                match a.partial_cmp(b) {
                    Some(ordering) => ordering,
                    None => {
                        err = Some(format!("partial_cmp failed for values '{}' and '{}'", a, b));
                        std::cmp::Ordering::Equal
                    }
                }
            });

            if let Some(e) = err {
                return Err(e);
            }

            Ok((Value::Array(sorted), None))
        }

        _ => unreachable!(),
    }
}