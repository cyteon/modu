use crate::vm::value::{InternalFn, Value};

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn _modu_print(ptr: *const u8, len: usize);
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn _modu_input(ptr: *const u8, len: usize, out_len: *mut usize) -> *mut u8;
}

fn native(name: &str, func: fn(Vec<Value>) -> Result<Value, String>) -> InternalFn {
    InternalFn { name: name.to_string(), func }
}

pub fn get_functions() -> Vec<InternalFn> {
    vec![
        native("print", print),
        native("int", int),
        native("float", float),
        native("str", str),
        native("bool", bool),
    ]
}

fn print(args: Vec<Value>) -> Result<Value, String> {
    let output = args.iter().map(|v| format!("{}", v)).collect::<String>();

    #[cfg(target_arch = "wasm32")]
    {
        let text = format!("{}\n", output);
        unsafe { _modu_print(text.as_ptr(), text.len()) };
    }

    #[cfg(not(target_arch = "wasm32"))]
    println!("{}", output);

    Ok(Value::Null)
}

fn int(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("int() takes exactly one argument ({} given)", args.len()));
    }

    match &args[0] {
        Value::Float(f) => Ok(Value::Int(*f as i64)),
        Value::Bool(b) => Ok(Value::Int(if *b { 1 } else { 0 })),
        Value::String(s) => s.parse::<i64>()
            .map(|i| Value::Int(i as i64))
            .map_err(|_| format!("cannot convert '{}' to int", s)),
        
        _ => Err(format!("cannot convert {} to int", args[0].type_name())),
    }
}

fn float(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("float() takes exactly one argument ({} given)", args.len()));
    }

    match &args[0] {
        Value::Int(i) => Ok(Value::Float(*i as f64)),
        Value::Bool(b) => Ok(Value::Float(if *b { 1.0 } else { 0.0 })),
        Value::String(s) => s.parse::<f64>()
            .map(|f| Value::Float(f))
            .map_err(|_| format!("cannot convert '{}' to float", s)),
        
        _ => Err(format!("cannot convert {} to float", args[0].type_name())),
    }
}

fn str(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("str() takes exactly one argument ({} given)", args.len()));
    }

    Ok(Value::String(format!("{}", args[0])))
}

fn bool(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("bool() takes exactly one argument ({} given)", args.len()));
    }

    Ok(Value::Bool(args[0].truthy()))
}