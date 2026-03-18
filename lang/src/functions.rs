use crate::vm::value::{BuiltinFn, Value};

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn _modu_print(ptr: *const u8, len: usize);
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn _modu_input(ptr: *const u8, len: usize, out_len: *mut usize) -> *mut u8;
}

fn builtin(name: &str, func: fn(Vec<Value>) -> Result<Value, String>) -> BuiltinFn {
    BuiltinFn { name: name.to_string(), func }
}

pub fn get_functions() -> Vec<BuiltinFn> {
    vec![
        builtin("print", print),
        builtin("input", input),
        builtin("int", int),
        builtin("float", float),
        builtin("str", str),
        builtin("bool", bool),
        builtin("type", r#type),
        builtin("exit", exit),
        builtin("error", error),
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

fn input(args: Vec<Value>) -> Result<Value, String> {
    for arg in args.clone() {
        print!("{}", arg);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        Ok(Value::String(input.trim_end().to_string()))
    }

    #[cfg(target_arch = "wasm32")]
    {
        let prompt = args.iter().map(|v| format!("{}", v)).collect::<String>();
        let input;

        unsafe {
            let mut out_len: usize = 0;
            let ptr = _modu_input(prompt.as_ptr(), prompt.len(), &mut out_len);

            if ptr.is_null() {
                return Err("input failed".to_string());
            }

            let bytes = std::slice::from_raw_parts(ptr, out_len);
            input = String::from_utf8_lossy(bytes).to_string();
        }

        Ok(Value::String(input.trim_end().to_string()))
    }
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

fn r#type(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("type() takes exactly one argument ({} given)", args.len()));
    }

    Ok(Value::String(args[0].type_name().to_string()))
}

fn exit(args: Vec<Value>) -> Result<Value, String> {
    if args.len() == 0 {
        std::process::exit(0);
    } else if args.len() == 1 {
        match &args[0] {
            Value::Int(i) => std::process::exit(*i as i32),
            _ => return Err(format!("exit() argument must be an int, got {}", args[0].type_name())),
        }
    } else {
        return Err(format!("exit() takes at most one argument ({} given)", args.len()));
    }
}

fn error(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("error() takes exactly one argument ({} given)", args.len()));
    }

    Err(format!("{}", args[0]))
}