use std::collections::HashMap;

use crate::{ast::{Expr, InternalFunctionResponse, Spanned}, lexer::Span};

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
            format!("Could not convert string to int: {}", e),
            args[0].span,
        ))?,
        Expr::Bool(b) => if *b { 1 } else { 0 },
        _ => return Err((
            format!("Cannot convert {:?} to int", args[0].node),
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
            format!("Could not convert string to float: {}", e),
            args[0].span,
        ))?,
        Expr::Bool(b) => if *b { 1.0 } else { 0.0 },
        _ => return Err((
            format!("Cannot convert {:?} to float", args[0].node),
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
            format!("Cannot convert {:?} to bool", args[0].node),
            args[0].span,
        )),
    }; 

    Ok(InternalFunctionResponse {
        return_value: Expr::Bool(boolean),
        replace_self: None,
    })
}

pub fn fill_context(context: &mut HashMap<String, Expr>) {
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
}