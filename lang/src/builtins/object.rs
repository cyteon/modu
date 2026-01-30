use crate::{ast::{Expr, InternalFunctionResponse, Spanned}, lexer::Span};

pub fn get(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let object = match &args[0].node {
        Expr::Object { properties } => properties,
        _ => {
            return Err((
                "get expects an object as the first argument".to_string(),
                args[0].span,
            ))
        }
    };

    let key = match &args[1].node {
        Expr::String(s) => s,
        _ => {
            return Err((
                "get expects a string as the second argument".to_string(),
                args[1].span,
            ))
        }
    };

    match object.get(key) {
        Some(value) => Ok(InternalFunctionResponse {
            return_value: value.clone(),
            replace_self: None,
        }),

        None => Err((
            format!("Key '{}' not found in object", key),
            args[1].span,
        )),
    }
}

pub fn set(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let object = match &args[0].node {
        Expr::Object { properties } => properties.clone(),
        _ => {
            return Err((
                "set expects an object as the first argument".to_string(),
                args[0].span,
            ))
        }
    };

    let key = match &args[1].node {
        Expr::String(s) => s.clone(),
        _ => {
            return Err((
                "set expects a string as the second argument".to_string(),
                args[1].span,
            ))
        }
    };

    let mut new_properties = object;
    new_properties.insert(key, args[2].node.clone());

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: Some(Expr::Object {
            properties: new_properties,
        }),
    })
}

pub fn has(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let object = match &args[0].node {
        Expr::Object { properties } => properties,
        _ => {
            return Err((
                "has expects an object as the first argument".to_string(),
                args[0].span,
            ))
        }
    };

    let key = match &args[1].node {
        Expr::String(s) => s,
        _ => {
            return Err((
                "has expects a string as the second argument".to_string(),
                args[1].span,
            ))
        }
    };

    let exists = object.contains_key(key);

    Ok(InternalFunctionResponse {
        return_value: Expr::Bool(exists),
        replace_self: None,
    })
}

pub fn delete(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let object = match &args[0].node {
        Expr::Object { properties } => properties.clone(),
        _ => {
            return Err((
                "delete expects an object as the first argument".to_string(),
                args[0].span,
            ))
        }
    };

    let key = match &args[1].node {
        Expr::String(s) => s.clone(),
        _ => {
            return Err((
                "delete expects a string as the second argument".to_string(),
                args[1].span,
            ))
        }
    };

    let mut new_properties = object;
    new_properties.remove(&key);

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: Some(Expr::Object {
            properties: new_properties,
        }),
    })
}

pub fn to_string(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let object = &args[0].node;

    let result = match object {
        Expr::Object { properties } => {
            let mut parts = vec![];
            for (key, value) in properties {
                let value_str = match value {
                    Expr::String(s) => format!("\"{}\"", s),
                    Expr::Int(n) => n.to_string(),
                    Expr::Float(f) => f.to_string(),
                    Expr::Bool(b) => b.to_string(),
                    Expr::Null => "null".to_string(),
                    _ => "\"<complex_value>\"".to_string(),
                };
                parts.push(format!("\"{}\": {}", key, value_str));
            }
            format!("{{{}}}", parts.join(", "))
        }
        _ => {
            return Err((
                "to_string expects an object as the first argument".to_string(),
                args[0].span,
            ))
        }
    };

    Ok(InternalFunctionResponse {
        return_value: Expr::String(result),
        replace_self: None,
    })
}

pub fn get_fn(name: &str) -> Option<Expr> {
    Some(Expr::InternalFunction {
        name: name.to_string(),
        args: match name {
            "get" => vec!["self".to_string(), "key".to_string()],
            "set" => vec!["self".to_string(), "key".to_string(), "value".to_string()],
            "has" => vec!["self".to_string(), "key".to_string()],
            "delete" => vec!["self".to_string(), "key".to_string()],
            "to_string" => vec!["self".to_string()],
            _ => vec![],
        },
        func: match name {
            "get" => get,
            "set" => set,
            "has" => has,
            "delete" => delete,
            "to_string" => to_string,
            _ => return None,
        },
    })
}