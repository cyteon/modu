use crate::{ast::{Expr, InternalFunctionResponse, Spanned}, lexer::Span};

pub fn min(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let a = match &args[0].node {
        Expr::Int(v) => v,
        _ => unreachable!(),
    };

    let b = match &args[1].node {
        Expr::Int(v) => v,
        _ => {
            return Err((
                "can only compare two integers".to_string(),
                args[1].span.clone(),
            ));
        }
    };
    
    Ok(InternalFunctionResponse {
        return_value: Expr::Int(*a.min(b)),
        replace_self: None,
    })
}

pub fn max(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let a = match &args[0].node {
        Expr::Int(v) => v,
        _ => unreachable!(),
    };

    let b = match &args[1].node {
        Expr::Int(v) => v,
        _ => {
            return Err((
                "can only compare two integers".to_string(),
                args[1].span.clone(),
            ));
        }
    };
    
    Ok(InternalFunctionResponse {
        return_value: Expr::Int(*a.max(b)),
        replace_self: None,
    })
}

pub fn get_fn(name: &str) -> Option<Expr> {
    Some(Expr::InternalFunction {
        name: name.to_string(),
        args: match name {
            "min" => vec!["self".to_string(), "b".to_string()],
            "max" => vec!["self".to_string(), "b".to_string()],
            _ => vec![],
        },
        func: match name {
            "min" => min,
            "max" => max,
            _ => return None,
        },
    })
}