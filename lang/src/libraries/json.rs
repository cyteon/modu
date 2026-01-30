use std::collections::HashMap;

use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

pub fn new(_: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    Ok(InternalFunctionResponse {
        return_value: Expr::Object { properties: std::collections::HashMap::new() },
        replace_self: None,
    })
}

pub fn parse_obj(obj: &mut HashMap<String, serde_json::Value>) -> HashMap<String, Expr> {
    let mut map = HashMap::new();

    for (key, value) in obj.drain() {
        match value {
            serde_json::Value::Null => {
                map.insert(key, Expr::Null);
            }
            
            serde_json::Value::Bool(b) => {
                map.insert(key, Expr::Bool(b));
            }

            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    map.insert(key, Expr::Int(i));
                } else if let Some(f) = n.as_f64() {
                    map.insert(key, Expr::Float(f));
                }
            }

            serde_json::Value::String(s) => {
                map.insert(key, Expr::String(s));
            }

            v => {
                map.insert(key, Expr::String(v.to_string()));
            }
        }
    }

    map
}

pub fn parse(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let json_str = match &args[0].node {
        Expr::String(s) => s,
        _ => {
            return Err((
                "parse expects a string as the first argument".to_string(),
                args[0].span,
            ))
        }
    };

    let mut parsed: HashMap<String, serde_json::Value> = serde_json::from_str(json_str).map_err(|e| (
        format!("Failed to parse JSON: {}", e),
        args[0].span,
    ))?;

    let properties = parse_obj(&mut parsed);

    Ok(InternalFunctionResponse {
        return_value: Expr::Object { properties },
        replace_self: None,
    })
}

pub fn get_object() -> Expr {
    let mut symbols = std::collections::HashMap::new();

    symbols.insert(
        "new".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "new".to_string(),
                args: vec![],
                func: new,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "parse".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "parse".to_string(),
                args: vec!["json_str".to_string()],
                func: parse,
            },
            span: Span::default(),
        },
    );

    Expr::Module { symbols }
}