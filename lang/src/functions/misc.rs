use std::collections::HashMap;

use crate::{ast::{Expr, InternalFunctionResponse, Spanned}, lexer::Span};

pub fn print(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    for arg in args {
        print!("{}", arg.node);
    }
    println!();

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: None,
    })
}

pub fn input(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    use std::io::{self, Write};

    for arg in args {
        print!("{}", arg.node);
    }
    
    io::stdout().flush().map_err(|e| (format!("Failed to flush stdout: {}", e), args[0].span))?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| (format!("Failed to read input: {}", e), args[0].span))?;

    Ok(InternalFunctionResponse {
        return_value: Expr::String(input.trim_end().to_string()),
        replace_self: None,
    })
}

pub fn exit(_: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    std::process::exit(0);
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
}