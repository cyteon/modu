use crate::vm::value::{NativeFn, Value};

pub fn get_fn(name: String) -> Option<NativeFn> {
    match name.as_str() {
        "min" => Some(NativeFn::new("min", min)),
        "max" => Some(NativeFn::new("max", max)),
        "abs" => Some(NativeFn::new("abs", abs)),
        "sqrt" => Some(NativeFn::new("sqrt", sqrt)),
        _ => None,
    }
}

pub fn list_fns() -> Vec<String> {
    vec![
        "min".to_string(),
        "max".to_string(),
        "abs".to_string(),
        "sqrt".to_string(),
    ]
}

pub fn min(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("<int>.min() takes exactly one argument ({} given)", args.len()));
    }

    match (&this, &args[0]) {
        (Value::Int(a), Value::Int(b)) => Ok((Value::Int((*a).min(*b)), None)),
        _ => Err(format!("<int>.min() is not supported for types {} and {}", this.type_name(), args[0].type_name())),
    }
}

pub fn max(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("<int>.max() takes exactly one argument ({} given)", args.len()));
    }

    match (&this, &args[0]) {
        (Value::Int(a), Value::Int(b)) => Ok((Value::Int((*a).max(*b)), None)),
        _ => Err(format!("<int>.max() is not supported for types {} and {}", this.type_name(), args[0].type_name())),
    }
}

pub fn abs(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<int>.abs() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Int(a) => Ok((Value::Int(a.abs()), None)),
        _ => unreachable!(),
    }
}

pub fn sqrt(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<int>.sqrt() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Int(a) => Ok((Value::Float((a as f64).sqrt()), None)),
        _ => unreachable!(),
    }
}