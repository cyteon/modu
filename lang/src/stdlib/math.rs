use std::collections::HashMap;
use crate::vm::value::{BuiltinFn, Value};

pub fn object() -> Value {
    let mut methods = HashMap::new();

    methods.insert("rand".to_string(), Value::BuiltinFn(BuiltinFn::new("rand", rand)));
    methods.insert("randi".to_string(), Value::BuiltinFn(BuiltinFn::new("randi", randi)));
    methods.insert("rand_range".to_string(), Value::BuiltinFn(BuiltinFn::new("rand_range", rand_range)));

    methods.insert("sin".to_string(), Value::BuiltinFn(BuiltinFn::new("sin", sin)));
    methods.insert("cos".to_string(), Value::BuiltinFn(BuiltinFn::new("cos", cos)));
    methods.insert("tan".to_string(), Value::BuiltinFn(BuiltinFn::new("tan", tan)));

    methods.insert("asin".to_string(), Value::BuiltinFn(BuiltinFn::new("asin", asin)));
    methods.insert("acos".to_string(), Value::BuiltinFn(BuiltinFn::new("acos", acos)));
    methods.insert("atan".to_string(), Value::BuiltinFn(BuiltinFn::new("atan", atan)));

    methods.insert("asind".to_string(), Value::BuiltinFn(BuiltinFn::new("asind", asind)));
    methods.insert("acosd".to_string(), Value::BuiltinFn(BuiltinFn::new("acosd", acosd)));
    methods.insert("atand".to_string(), Value::BuiltinFn(BuiltinFn::new("atand", atand)));
    
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

fn rand_range(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("math.rand_range() takes exactly two arguments ({} given)", args.len()));
    }

    let min = match &args[0] {
        Value::Int(i) => *i as f64,
        Value::Float(f) => *f,
        _ => return Err(format!("math.rand_range() arguments must be numbers, got {}", args[0].type_name())),
    };

    let max = match &args[1] {
        Value::Int(i) => *i as f64,
        Value::Float(f) => *f,
        _ => return Err(format!("math.rand_range() arguments must be numbers, got {}", args[1].type_name())),
    };

    if min > max {
        return Err(format!("math.rand_range() requires min to be less than max ({} >= {})", min, max));
    }

    let random_value = rand::random::<f64>() * (max - min) + min;

    Ok(Value::Float(random_value))
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

    Ok(Value::Float(angle.to_radians().sin()))
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

    Ok(Value::Float(angle.to_radians().cos()))
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

    Ok(Value::Float(angle.to_radians().tan()))
}

fn asin(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("math.asin() takes exactly one argument ({} given)", args.len()));
    }

    let value = match &args[0] {
        Value::Int(i) => *i as f64,
        Value::Float(f) => *f,
        _ => return Err(format!("math.asin() argument must be a number, got {}", args[0].type_name())),
    };

    Ok(Value::Float(value.asin()))
}

fn acos(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("math.acos() takes exactly one argument ({} given)", args.len()));
    }

    let value = match &args[0] {
        Value::Int(i) => *i as f64,
        Value::Float(f) => *f,
        _ => return Err(format!("math.acos() argument must be a number, got {}", args[0].type_name())),
    };

    Ok(Value::Float(value.acos()))
}

fn atan(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("math.atan() takes exactly one argument ({} given)", args.len()));
    }

    let value = match &args[0] {
        Value::Int(i) => *i as f64,
        Value::Float(f) => *f,
        _ => return Err(format!("math.atan() argument must be a number, got {}", args[0].type_name())),
    };

    Ok(Value::Float(value.atan()))
}

fn asind(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("math.asind() takes exactly one argument ({} given)", args.len()));
    }

    let value = match &args[0] {
        Value::Int(i) => *i as f64,
        Value::Float(f) => *f,
        _ => return Err(format!("math.asind() argument must be a number, got {}", args[0].type_name())),
    };

    Ok(Value::Float(value.asin().to_degrees()))
}

fn acosd(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("math.acosd() takes exactly one argument ({} given)", args.len()));
    }

    let value = match &args[0] {
        Value::Int(i) => *i as f64,
        Value::Float(f) => *f,
        _ => return Err(format!("math.acosd() argument must be a number, got {}", args[0].type_name())),
    };

    Ok(Value::Float(value.acos().to_degrees()))
}

fn atand(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("math.atand() takes exactly one argument ({} given)", args.len()));
    }

    let value = match &args[0] {
        Value::Int(i) => *i as f64,
        Value::Float(f) => *f,
        _ => return Err(format!("math.atand() argument must be a number, got {}", args[0].type_name())),
    };

    Ok(Value::Float(value.atan().to_degrees()))
}