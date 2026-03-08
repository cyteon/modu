use std::collections::HashMap;
use crate::vm::value::{BuiltinFn, Value};

pub fn object() -> Value {
    let mut methods = HashMap::new();

    methods.insert("rand".to_string(), Value::BuiltinFn(BuiltinFn::new("rand", rand)));
    methods.insert("randi".to_string(), Value::BuiltinFn(BuiltinFn::new("randi", randi)));
    
    methods.insert("sin".to_string(), Value::BuiltinFn(BuiltinFn::new("sin", sin)));
    methods.insert("cos".to_string(), Value::BuiltinFn(BuiltinFn::new("cos", cos)));
    methods.insert("tan".to_string(), Value::BuiltinFn(BuiltinFn::new("tan", tan)));
    
    methods.insert("PI".to_string(), Value::Float(std::f64::consts::PI));
    methods.insert("E".to_string(), Value::Float(std::f64::consts::E));

    Value::Object(methods)
}

fn rand(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(format!("math.rand() takes no arguments ({} given)", args.len()));
    }

    let random_value = rand::random::<f64>();

    Ok(Value::Float(random_value))
}

fn randi(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(format!("math.randi() takes no arguments ({} given)", args.len()));
    }

    let random_value = rand::random::<i64>();

    Ok(Value::Int(random_value))
}

fn sin(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("math.sin() takes exactly one argument ({} given)", args.len()));
    }

    let angle = match &args[0] {
        Value::Int(i) => *i as f64,
        Value::Float(f) => *f,
        _ => return Err(format!("math.sin() argument must be a number, got {}", args[0].type_name())),
    };

    Ok(Value::Float(angle.sin()))
}

fn cos(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("math.cos() takes exactly one argument ({} given)", args.len()));
    }

    let angle = match &args[0] {
        Value::Int(i) => *i as f64,
        Value::Float(f) => *f,
        _ => return Err(format!("math.cos() argument must be a number, got {}", args[0].type_name())),
    };

    Ok(Value::Float(angle.cos()))
}

fn tan(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("math.tan() takes exactly one argument ({} given)", args.len()));
    }

    let angle = match &args[0] {
        Value::Int(i) => *i as f64,
        Value::Float(f) => *f,
        _ => return Err(format!("math.tan() argument must be a number, got {}", args[0].type_name())),
    };

    Ok(Value::Float(angle.tan()))
}