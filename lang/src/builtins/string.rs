use crate::{ast::{Expr, InternalFunctionResponse, Spanned}, lexer::Span};

pub fn split(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let string = match &args[0].node {
        Expr::String(s) => s,
        _ => unreachable!(),
    };

    let delimiter = match &args[1].node {
        Expr::String(s) => s,
        _ => {
            return Err((
                "split expects a string delimiter".to_string(),
                args[1].span,
            )) 
        }
    };

    let split: Vec<&str> = if delimiter.is_empty() {
        string.split(delimiter).filter(|s| !s.is_empty()).collect()
    } else {
        string.split(delimiter).collect()
    };
    
    let mut expr_vec = Vec::new();

    for s in split {
        expr_vec.push(Spanned {
            node: Expr::String(s.to_string()),
            span: args[0].span,
        });
    }
    
    Ok(InternalFunctionResponse {
        return_value: Expr::Array(expr_vec),
        replace_self: None,
    })
}

pub fn replace(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let string = match &args[0].node {
        Expr::String(s) => s,
        _ => unreachable!(),
    };

    let a = match &args[1].node {
        Expr::String(s) => s,
        _ => {
            return Err((
                "replace expects a string to replace".to_string(),
                args[1].span,
            ))
        }
    };

    let b = match &args[2].node {
        Expr::String(s) => s,
        _ => {
            return Err((
                "replace expects a string to replace with".to_string(),
                args[2].span,
            ))
        }
    };

    let replaced = string.replace(a, b);

    Ok(InternalFunctionResponse {
        return_value: Expr::String(replaced),
        replace_self: None,
    })
}

pub fn trim(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let string = match &args[0].node {
        Expr::String(s) => s,
        _ => unreachable!(),
    };

    let trimmed = string.trim().to_string();

    Ok(InternalFunctionResponse {
        return_value: Expr::String(trimmed),
        replace_self: None,
    })
}

pub fn len(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let string = match &args[0].node {
        Expr::String(s) => s,
        _ => unreachable!(),
    };

    let length = string.len() as i64;
    
    Ok(InternalFunctionResponse {
        return_value: Expr::Int(length),
        replace_self: None,
    })
}

pub fn get_fn(name: &str) -> Option<Expr> {
    Some(Expr::InternalFunction {
        name: name.to_string(),
        args: match name {
            "split" => vec!["self".to_string(), "delimiter".to_string()],
            "replace" => vec!["self".to_string(), "a".to_string(), "b".to_string()],
            "len" => vec!["self".to_string()],
            "trim" => vec!["self".to_string()],
            _ => vec![],
        },
        func: match name {
            "split" => split,
            "replace" => replace,
            "len" => len,
            "trim" => trim,
            _ => return None,
        },
    })
}