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