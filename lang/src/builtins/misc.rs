use std::collections::HashMap;
use crate::{ast::{Expr, InternalFunctionResponse, Spanned}, lexer::Span};

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn _modu_print(ptr: *const u8, len: usize);
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn _modu_input(ptr: *const u8, len: usize, out_len: *mut usize) -> *mut u8;
}

pub fn print(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let mut output = String::new();

    for arg in args {
        output.push_str(&format!("{}", arg.node));
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("{}", output);
    }

    #[cfg(target_arch = "wasm32")]
    {
        let text = format!("{}\n", output);

        unsafe {
            _modu_print(text.as_ptr(), text.len());
        }
    }

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: None,
    })
}

pub fn eprint(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let mut output = String::new();

    for arg in args {
        output.push_str(&format!("{}", arg.node));
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        eprintln!("{}", output);
    }

    #[cfg(target_arch = "wasm32")]
    {
        let text = format!("{}\n", output);

        unsafe {
            _modu_print(text.as_ptr(), text.len());
        }
    }

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: None,
    })
}

pub fn input(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    use std::io::{self, Write};

    for arg in args.clone() {
        print!("{}", arg.node);
    }
    
    let span = args.first().map(|a| a.span.clone()).unwrap_or_default();

    #[cfg(not(target_arch = "wasm32"))]
    {
        io::stdout().flush().map_err(|e| (format!("Failed to flush stdout: {}", e), span))?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| (format!("Failed to read input: {}", e), span))?;

        Ok(InternalFunctionResponse {
            return_value: Expr::String(input.trim_end().to_string()),
            replace_self: None,
        })
    }

    #[cfg(target_arch = "wasm32")]
    {
        let prompt = args.iter().map(|a| format!("{}", a.node)).collect::<String>();
        let input;

        unsafe {
            let mut out_len: usize = 0;
            let ptr = _modu_input(prompt.as_ptr(), prompt.len(), &mut out_len as *mut usize);

            if ptr.is_null() {
                return Err((
                    "Failed to read input from JavaScript".to_string(),
                    span,
                ));
            }

            let bytes = Vec::from_raw_parts(ptr, out_len, out_len);
            input = String::from_utf8(bytes).map_err(|e| (
                format!("Invalid UTF-8 input from JavaScript: {}", e),
                span,
            ))?;
        }

        Ok(InternalFunctionResponse {
            return_value: Expr::String(input),
            replace_self: None,
        })
    }
}

pub fn exit(_: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    std::process::exit(0);
}

pub fn str(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let string = match &args[0].node {
        Expr::Int(n) => n.to_string(),
        Expr::Float(f) => f.to_string(),
        Expr::String(s) => s.clone(),
        Expr::Bool(b) => b.to_string(),
        Expr::Null => "null".to_string(),
        _ => return Err((
            format!("Cannot convert {:?} to string", args[0].node),
            args[0].span,
        )),
    }; 

    Ok(InternalFunctionResponse {
        return_value: Expr::String(string),
        replace_self: None,
    })
}

pub fn int(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let integer = match &args[0].node {
        Expr::Int(n) => *n,
        Expr::Float(f) => *f as i64,
        Expr::String(s) => s.parse::<i64>().map_err(|e| (
            format!("could not convert string to int: {}", e),
            args[0].span,
        ))?,
        Expr::Bool(b) => if *b { 1 } else { 0 },
        _ => return Err((
            format!("cannot convert {} to int", args[0].node),
            args[0].span,
        )),
    }; 

    Ok(InternalFunctionResponse {
        return_value: Expr::Int(integer),
        replace_self: None,
    })
}

pub fn float(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let float = match &args[0].node {
        Expr::Int(n) => *n as f64,
        Expr::Float(f) => *f,
        Expr::String(s) => s.parse::<f64>().map_err(|e| (
            format!("could not convert string to float: {}", e),
            args[0].span,
        ))?,
        Expr::Bool(b) => if *b { 1.0 } else { 0.0 },
        _ => return Err((
            format!("cannot convert '{}' to float", args[0].node),
            args[0].span,
        )),
    }; 

    Ok(InternalFunctionResponse {
        return_value: Expr::Float(float),
        replace_self: None,
    })
}

pub fn bool(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let boolean = match &args[0].node {
        Expr::Int(n) => *n != 0,
        Expr::Float(f) => *f != 0.0,
        Expr::String(s) => s == "true",
        Expr::Bool(b) => *b,
        Expr::Null => false,
        _ => return Err((
            format!("cannot convert '{}' to bool", args[0].node),
            args[0].span,
        )),
    }; 

    Ok(InternalFunctionResponse {
        return_value: Expr::Bool(boolean),
        replace_self: None,
    })
}

pub fn type_of(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let type_name = match &args[0].node {
        Expr::Int(_) => "int",
        Expr::Float(_) => "float",
        Expr::String(_) => "string",
        Expr::Bool(_) => "bool",
        Expr::Null => "null",
        Expr::Function { .. } => "function",
        Expr::InternalFunction { .. } => "function",
        Expr::Array(_) => "array",
        Expr::Object {..} => "object",
        Expr::Module { .. } => "module",
        
        #[cfg(not(target_arch = "wasm32"))]
        Expr::FFILibrary { .. } => "ffilibrary",
        #[cfg(not(target_arch = "wasm32"))]
        Expr::File(_) => "file",

        _ => "unknown",

    }.to_string();

    Ok(InternalFunctionResponse {
        return_value: Expr::String(type_name),
        replace_self: None,
    })
}

pub fn fill_context(context: &mut HashMap<String, Expr>) {
    context.insert(
        "print".to_string(),
        Expr::InternalFunction {
            name: "print".to_string(),
            args: vec!["__args__".to_string()],
            func: print,
        },
    );

    context.insert(
        "eprint".to_string(),
        Expr::InternalFunction {
            name: "eprint".to_string(),
            args: vec!["__args__".to_string()],
            func: eprint,
        },
    );

    context.insert(
        "input".to_string(),
        Expr::InternalFunction {
            name: "input".to_string(),
            args: vec!["__args__".to_string()],
            func: input,
        },
    );

    context.insert(
        "exit".to_string(),
        Expr::InternalFunction {
            name: "exit".to_string(),
            args: vec!["__args__".to_string()],
            func: exit,
        },
    );

    context.insert(
        "str".to_string(),
        Expr::InternalFunction {
            name: "str".to_string(),
            args: vec!["value".to_string()],
            func: str,
        },
    );

    context.insert(
        "int".to_string(),
        Expr::InternalFunction {
            name: "int".to_string(),
            args: vec!["value".to_string()],
            func: int,
        },
    );

    context.insert(
        "float".to_string(),
        Expr::InternalFunction {
            name: "float".to_string(),
            args: vec!["value".to_string()],
            func: float,
        },
    );

    context.insert(
        "bool".to_string(),
        Expr::InternalFunction {
            name: "bool".to_string(),
            args: vec!["value".to_string()],
            func: bool,
        },
    );

    context.insert(
        "type".to_string(),
        Expr::InternalFunction {
            name: "type".to_string(),
            args: vec!["value".to_string()],
            func: type_of,
        },
    );
}