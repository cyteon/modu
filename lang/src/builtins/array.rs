use crate::{ast::{Expr, InternalFunctionResponse, Spanned}, lexer::Span};

pub fn len(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let array = match &args[0].node {
        Expr::Array(elements) => elements,
        _ => unreachable!(),
    };

    let length = array.len() as i64;
    
    Ok(InternalFunctionResponse {
        return_value: Expr::Int(length),
        replace_self: None,
    })
}

pub fn clear(_: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: Some(Expr::Array(vec![])),
    })
}

pub fn push(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let array = match &args[0].node {
        Expr::Array(elements) => elements,
        _ => unreachable!(),
    };

    if args.len() < 2 {
        return Err((
            "push expects 2 arguments".to_string(),
            args[1].span.clone(),
        ));
    }

    let mut new_array = array.clone();
    new_array.push(args[1].clone());

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: Some(Expr::Array(new_array)),
    })
}

pub fn pop(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let array = match &args[0].node {
        Expr::Array(elements) => elements,
        _ => unreachable!(),
    };

    if array.is_empty() {
        return Err((
            "cannot pop from an empty array".to_string(),
            args[0].span.clone(),
        ));
    }

    let mut new_array = array.clone();
    let popped_element = new_array.pop().unwrap();

    Ok(InternalFunctionResponse {
        return_value: popped_element.node,
        replace_self: Some(Expr::Array(new_array)),
    })
}

pub fn get_fn(name: &str) -> Option<Expr> {
    Some(Expr::InternalFunction {
        name: name.to_string(),
        args: match name {
            "len" => vec!["self".to_string()],
            "clear" => vec!["self".to_string()],
            "push" => vec!["self".to_string(), "value".to_string()],
            "pop" => vec!["self".to_string()],
            _ => vec![],
        },
        func: match name {
            "len" => len,
            "clear" => clear,
            "push" => push,
            "pop" => pop,
            _ => return None,
        },
    })
}