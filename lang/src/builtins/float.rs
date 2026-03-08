use crate::vm::value::{NativeFn, Value};

pub fn get_fn(name: String) -> Option<NativeFn> {
    match name.as_str() {
        "min" => Some(NativeFn::new("min", min)),
        "max" => Some(NativeFn::new("max", max)),
        _ => None,
    }
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