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

pub fn to_upper(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let string = match &args[0].node {
        Expr::String(s) => s,
        _ => unreachable!(),
    };

    let upper = string.to_uppercase();

    Ok(InternalFunctionResponse {
        return_value: Expr::String(upper),
        replace_self: None,
    })
}

pub fn to_lower(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let string = match &args[0].node {
        Expr::String(s) => s,
        _ => unreachable!(),
    };

    let lower = string.to_lowercase();

    Ok(InternalFunctionResponse {
        return_value: Expr::String(lower),
        replace_self: None,
    })
}

pub fn starts_with(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let string = match &args[0].node {
        Expr::String(s) => s,
        _ => unreachable!(),
    };

    let prefix = match &args[1].node {
        Expr::String(s) => s,
        _ => {
            return Err((
                "starts_with expects a string prefix".to_string(),
                args[1].span,
            ))
        }
    };

    let starts = string.starts_with(prefix);

    Ok(InternalFunctionResponse {
        return_value: Expr::Bool(starts),
        replace_self: None,
    })
}

pub fn ends_with(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let string = match &args[0].node {
        Expr::String(s) => s,
        _ => unreachable!(),
    };

    let suffix = match &args[1].node {
        Expr::String(s) => s,
        _ => {
            return Err((
                "ends_with expects a string suffix".to_string(),
                args[1].span,
            ))
        }
    };

    let ends = string.ends_with(suffix);

    Ok(InternalFunctionResponse {
        return_value: Expr::Bool(ends),
        replace_self: None,
    })
}

pub fn chars(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let string = match &args[0].node {
        Expr::String(s) => s,
        _ => unreachable!(),
    };

    let chars: Vec<Spanned<Expr>> = string.chars().map(|c| Spanned {
        node: Expr::String(c.to_string()),
        span: args[0].span,
    }).collect();

    Ok(InternalFunctionResponse {
        return_value: Expr::Array(chars),
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
            "to_upper" => vec!["self".to_string()],
            "to_lower" => vec!["self".to_string()],
            "starts_with" => vec!["self".to_string(), "prefix".to_string()],
            "ends_with" => vec!["self".to_string(), "suffix".to_string()],
            "chars" => vec!["self".to_string()],
            _ => vec![],
        },
        func: match name {
            "split" => split,
            "replace" => replace,
            "len" => len,
            "trim" => trim,
            "to_upper" => to_upper,
            "to_lower" => to_lower,
            "starts_with" => starts_with,
            "ends_with" => ends_with,
            "chars" => chars,
            _ => return None,
        },
    })
}