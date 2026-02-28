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

pub fn join(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let array = match &args[0].node {
        Expr::Array(elements) => elements,
        _ => unreachable!(),
    };

    let delimiter = match &args[1].node {
        Expr::String(s) => s,
        _ => {
            return Err((
                "join expects a string delimiter".to_string(),
                args[1].span,
            ))
        }
    };

    let mut string_vec = Vec::new();

    for element in array {
        string_vec.push(format!("{}", element.node));
    }

    let joined = string_vec.join(delimiter);

    Ok(InternalFunctionResponse {
        return_value: Expr::String(joined),
        replace_self: None,
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
            "join" => vec!["self".to_string(), "delimiter".to_string()],
            _ => vec![],
        },
        func: match name {
            "len" => len,
            "clear" => clear,
            "push" => push,
            "pop" => pop,
            "join" => join,
            _ => return None,
        },
    })
}