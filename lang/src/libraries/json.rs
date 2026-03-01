use std::collections::HashMap;

use crate::{ast::{Expr, InternalFunctionResponse, SpannedExpr}, lexer::Span};

pub fn new(_: Vec<SpannedExpr>) -> Result<InternalFunctionResponse, (String, Span)> {
    Ok(InternalFunctionResponse {
        return_value: Expr::Object { properties: std::collections::HashMap::new() },
        replace_self: None,
    })
}

pub fn parse_obj(obj: &mut HashMap<String, serde_json::Value>) -> HashMap<String, SpannedExpr> {
    let mut map: HashMap<String, SpannedExpr> = HashMap::new();

    for (key, value) in obj.drain() {
        match value {
            serde_json::Value::Null => {
                map.insert(key, SpannedExpr { node: Expr::Null, span: Span::default() });
            }
            
            serde_json::Value::Bool(b) => {
                map.insert(key, SpannedExpr { node: Expr::Bool(b), span: Span::default() });
            }

            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    map.insert(key, SpannedExpr { node: Expr::Int(i), span: Span::default() });
                } else if let Some(f) = n.as_f64() {
                    map.insert(key, SpannedExpr { node: Expr::Float(f), span: Span::default() });
                }
            }

            serde_json::Value::String(s) => {
                map.insert(key, SpannedExpr { node: Expr::String(s), span: Span::default() });
            }

            serde_json::Value::Object(o) => {
                let mut hashmap: HashMap<String, serde_json::Value> = o.into_iter().collect();

                let properties = parse_obj(&mut hashmap);
                map.insert(key, SpannedExpr { node: Expr::Object { properties }, span: Span::default() });
            }

            serde_json::Value::Array(arr) => {
                let elements: Vec<SpannedExpr> = arr.into_iter().map(|v| {
                    let expr = match v {
                        serde_json::Value::Null => SpannedExpr { node: Expr::Null, span: Span::default() },
                        serde_json::Value::Bool(b) => SpannedExpr { node: Expr::Bool(b), span: Span::default() },
                        serde_json::Value::Number(n) => {
                            if let Some(i) = n.as_i64() {
                                SpannedExpr { node: Expr::Int(i), span: Span::default() }
                            } else if let Some(f) = n.as_f64() {
                                SpannedExpr { node: Expr::Float(f), span: Span::default() }
                            } else {
                                SpannedExpr { node: Expr::Null, span: Span::default() }
                            }
                        }
                        serde_json::Value::String(s) => SpannedExpr { node: Expr::String(s), span: Span::default() },
                        serde_json::Value::Object(o) => {
                            let mut hashmap: HashMap<String, serde_json::Value> = o.into_iter().collect();
                            let properties = parse_obj(&mut hashmap);
                            
                            SpannedExpr { node: Expr::Object { properties }, span: Span::default() }
                        }
                        serde_json::Value::Array(a) => {
                            let nested_elements: Vec<SpannedExpr> = a.into_iter().map(|v| {
                                let nested_expr = match v {
                                    serde_json::Value::Null => Expr::Null,
                                    serde_json::Value::Bool(b) => Expr::Bool(b),
                                    serde_json::Value::Number(n) => {
                                        if let Some(i) = n.as_i64() {
                                            Expr::Int(i)
                                        } else if let Some(f) = n.as_f64() {
                                            Expr::Float(f)
                                        } else {
                                            Expr::Null
                                        }
                                    }
                                    serde_json::Value::String(s) => Expr::String(s),
                                    _ => Expr::Null,
                                };

                                SpannedExpr { node: nested_expr, span: Span::default() }
                            }).collect();
                            
                            SpannedExpr { node: Expr::Array(nested_elements), span: Span::default() }
                        }
                    };

                    expr
                }).collect();

                map.insert(key, SpannedExpr { node: Expr::Array(elements), span: Span::default() });
            }
        }
    }

    map
}

pub fn parse(args: Vec<SpannedExpr>) -> Result<InternalFunctionResponse, (String, Span)> {
    let json_str = match &args[0].node {
        Expr::String(s) => s,
        _ => {
            return Err((
                "json.parse expects a string as the first argument".to_string(),
                args[0].span,
            ))
        }
    };

    let mut parsed: HashMap<String, serde_json::Value> = serde_json::from_str(json_str).map_err(|e| (
        format!("failed to parse JSON: {}", e),
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