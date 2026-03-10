use crate::vm::value::{NativeFn, Value};

pub fn get_fn(name: String) -> Option<NativeFn> {
    match name.as_str() {
        "min" => Some(NativeFn::new("min", min)),
        "max" => Some(NativeFn::new("max", max)),
        "abs" => Some(NativeFn::new("abs", abs)),
        "sqrt" => Some(NativeFn::new("sqrt", sqrt)),
        "round" => Some(NativeFn::new("round", round)),
        "ceil" => Some(NativeFn::new("ceil", ceil)),
        "floor" => Some(NativeFn::new("floor", floor)),
        _ => None,
    }
}

pub fn list_fns() -> Vec<String> {
    vec![
        "min".to_string(),
        "max".to_string(),
        "abs".to_string(),
        "sqrt".to_string(),
        "round".to_string(),
        "ceil".to_string(),
        "floor".to_string(),
    ]
}

pub fn min(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("<float>.min() takes exactly one argument ({} given)", args.len()));
    }

    match (&this, &args[0]) {
        (Value::Float(a), Value::Float(b)) => Ok((Value::Float((*a).min(*b)), None)),
        _ => Err(format!("<float>.min() is not supported for types {} and {}", this.type_name(), args[0].type_name())),
    }
}

pub fn max(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if args.len() != 1 {
        return Err(format!("<float>.max() takes exactly one argument ({} given)", args.len()));
    }

    match (&this, &args[0]) {
        (Value::Float(a), Value::Float(b)) => Ok((Value::Float((*a).max(*b)), None)),
        _ => Err(format!("<float>.max() is not supported for types {} and {}", this.type_name(), args[0].type_name())),
    }
}

pub fn abs(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<float>.abs() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Float(a) => Ok((Value::Float(a.abs()), None)),
        _ => unreachable!(),
    }
}

pub fn sqrt(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<float>.sqrt() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Float(a) => Ok((Value::Float(a.sqrt()), None)),
        _ => unreachable!(),
    }
}

pub fn round(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<float>.round() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Float(a) => Ok((Value::Int(a.round() as i64), None)),
        _ => unreachable!(),
    }
}

pub fn ceil(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<float>.ceil() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Float(a) => Ok((Value::Int(a.ceil() as i64), None)),
        _ => unreachable!(),
    }
}

pub fn floor(this: Value, args: Vec<Value>) -> Result<(Value, Option<Value>), String> {
    if !args.is_empty() {
        return Err(format!("<float>.floor() takes no arguments ({} given)", args.len()));
    }

    match this {
        Value::Float(a) => Ok((Value::Int(a.floor() as i64), None)),
        _ => unreachable!(),
    }
}