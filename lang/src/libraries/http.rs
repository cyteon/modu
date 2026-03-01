use std::collections::HashMap;

use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

fn handle_response(response: reqwest::blocking::Response) -> Result<InternalFunctionResponse, (String, Span)> {
    let status = response.status();

    let mut properties = HashMap::new();

    properties.insert(
        "status".to_string(),
        SpannedExpr {
            node: Expr::Int(status.as_u16() as i64),
            span: Span::default(),
        }
    );

    properties.insert(
        "status_text".to_string(),
        SpannedExpr {
            node: Expr::String(status.canonical_reason().unwrap_or("").to_string()),
            span: Span::default(),
        }
    );

    let headers = Expr::Object {
        properties: response.headers().iter().map(|(k, v)| {
            (
                k.to_string(),
                SpannedExpr {
                    node: Expr::String(v.to_str().unwrap_or("").to_string()),
                    span: Span::default(),
                }
            )
        }).collect(),
    };

    properties.insert(
        "headers".to_string(),
        SpannedExpr {
            node: headers,
            span: Span::default(),
        }
    );

    let body = response.text().map_err(|e| (
        format!("failed to read response body: {}", e),
        Span::default(),
    ))?;

    properties.insert(
        "body".to_string(),
        SpannedExpr {
            node: Expr::String(body),
            span: Span::default(),
        }
    );

    properties.insert(
        "ok".to_string(),
        SpannedExpr {
            node: Expr::Bool(status.is_success()),
            span: Span::default(),
        }
    );

    Ok(InternalFunctionResponse {
        return_value: Expr::Object { properties },
        replace_self: None,
    })
}

pub fn get(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    if args.len() == 0 {
        return Err((
            "http.get expects atleast 1 argument".to_string(),
            args[0].span,
        ));
    } else if args.len() > 2 {
        return Err((
            "http.get takes at most 2 arguments".to_string(),
            args[1].span,
        ));
    }

    let url = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "http.get expects only string arguments".to_string(),
            args[0].span,
        )),
    };

    let client = reqwest::blocking::Client::new();
    let mut req = client.get(url);

    if args.len() == 2 {
        let headers = match &args[1].node {
            Expr::Object { properties } => properties,
            _ => return Err((
                "http.get expects the second argument to be an object".to_string(),
                args[1].span,
            )),
        };

        for (key, value) in headers {
            let value = match &value.node {
                Expr::String(s) => s,
                _ => return Err((
                    "http.get expects header values to be strings".to_string(),
                    args[1].span,
                )),
            };

            req = req.header(key, value);
        }
    }

    let response = req.send().map_err(|e| (
        format!("failed to perform GET request: {}", e),
        args[0].span,
    ))?;

    handle_response(response)
}

pub fn post(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    if args.len() == 0 {
        return Err((
            "http.post expects atleast 1 argument".to_string(),
            args[0].span,
        ));
    } else if args.len() > 3 {
        return Err((
            "http.post takes at most 3 arguments".to_string(),
            args[2].span,
        ));
    }

    let url = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "http.post expects a string as the first argument".to_string(),
            args[0].span,
        )),
    };

    let client = reqwest::blocking::Client::new();
    let mut req = client.post(url);
    
    if args.len() >= 2 {
        let body = match &args[1].node {
            Expr::String(s) => s,
            _ => return Err((
                "http.post expects a string as the second argument".to_string(),
                args[1].span,
            )),
        };

        req = req.body(body.clone());
    }

    if args.len() == 3 {
        let headers = match &args[2].node {
            Expr::Object { properties } => properties,
            _ => return Err((
                "http.post expects the third argument to be an object".to_string(),
                args[2].span,
            )),
        };

        for (key, value) in headers {
            let value = match &value.node {
                Expr::String(s) => s,
                _ => return Err((
                    "http.post expects header values to be strings".to_string(),
                    args[2].span,
                )),
            };

            req = req.header(key, value);
        }
    }

    let response = req.send().map_err(|e| (
        format!("failed to perform POST request: {}", e),
        args[0].span,
    ))?;

    handle_response(response)
}

pub fn put(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    if args.len() == 0 {
        return Err((
            "http.put expects atleast 1 argument".to_string(),
            args[0].span,
        ));
    } else if args.len() > 3 {
        return Err((
            "http.put takes at most 3 arguments".to_string(),
            args[2].span,
        ));
    }

    let url = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "http.put expects a string as the first argument".to_string(),
            args[0].span,
        )),
    };

    let client = reqwest::blocking::Client::new();
    let mut req = client.put(url);

    if args.len() >= 2 {
        let body = match &args[1].node {
            Expr::String(s) => s,
            _ => return Err((
                "http.put expects a string as the second argument".to_string(),
                args[1].span,
            )),
        };

        req = req.body(body.clone());
    }

    if args.len() == 3 {
        let headers = match &args[2].node {
            Expr::Object { properties } => properties,
            _ => return Err((
                "http.put expects the third argument to be an object".to_string(),
                args[2].span,
            )),
        };

        for (key, value) in headers {
            let value = match &value.node {
                Expr::String(s) => s,
                _ => return Err((
                    "http.put expects header values to be strings".to_string(),
                    args[2].span,
                )),
            };

            req = req.header(key, value);
        }
    }

    let response = req.send().map_err(|e| (
        format!("failed to perform PUT request: {}", e),
        args[0].span,
    ))?;

    handle_response(response)
}

pub fn patch(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    if args.len() == 0 {
        return Err((
            "http.patch expects atleast 1 argument".to_string(),
            args[0].span,
        ));
    } else if args.len() > 3 {
        return Err((
            "http.patch takes at most 3 arguments".to_string(),
            args[2].span,
        ));
    }

    let url = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "http.patch expects a string as the first argument".to_string(),
            args[0].span,
        )),
    };

    let client = reqwest::blocking::Client::new();
    let mut req = client.patch(url);

    if args.len() >= 2 {
        let body = match &args[1].node {
            Expr::String(s) => s,
            _ => return Err((
                "http.patch expects a string as the second argument".to_string(),
                args[1].span,
            )),
        };

        req = req.body(body.clone());
    }

    if args.len() == 3 {
        let headers = match &args[2].node {
            Expr::Object { properties } => properties,
            _ => return Err((
                "http.patch expects the third argument to be an object".to_string(),
                args[2].span,
            )),
        };

        for (key, value) in headers {
            let value = match &value.node {
                Expr::String(s) => s,
                _ => return Err((
                    "http.patch expects header values to be strings".to_string(),
                    args[2].span,
                )),
            };

            req = req.header(key, value);
        }
    }

    let response = req.send().map_err(|e| (
        format!("failed to perform PATCH request: {}", e),
        args[0].span,
    ))?;

    handle_response(response)
}

pub fn delete(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    if args.len() == 0 {
        return Err((
            "http.delete expects atleast 1 argument".to_string(),
            args[0].span,
        ));
    } else if args.len() > 2 {
        return Err((
            "http.delete takes at most 2 arguments".to_string(),
            args[1].span,
        ));
    }

    let url = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "http.delete expects a string as the first argument".to_string(),
            args[0].span,
        )),
    };

    let client = reqwest::blocking::Client::new();
    let mut req = client.delete(url);

    if args.len() == 2 {
        let headers = match &args[1].node {
            Expr::Object { properties } => properties,
            _ => return Err((
                "http.delete expects the second argument to be an object".to_string(),
                args[1].span,
            )),
        };

        for (key, value) in headers {
            let value = match &value.node {
                Expr::String(s) => s,
                _ => return Err((
                    "http.delete expects header values to be strings".to_string(),
                    args[1].span,
                )),
            };

            req = req.header(key, value);
        }
    }

    let response = req.send().map_err(|e| (
        format!("failed to perform DELETE request: {}", e),
        args[0].span,
    ))?;

    handle_response(response)
}

pub fn get_object() -> Expr {
    let mut symbols = std::collections::HashMap::new();

    symbols.insert(
        "get".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "get".to_string(),
                args: vec!["__args__".to_string()],
                func: get,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "post".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "post".to_string(),
                args: vec!["__args__".to_string()],
                func: post,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "put".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "put".to_string(),
                args: vec!["__args__".to_string()],
                func: put,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "patch".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "patch".to_string(),
                args: vec!["__args__".to_string()],
                func: patch,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "delete".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "delete".to_string(),
                args: vec!["__args__".to_string()],
                func: delete,
            },
            span: Span::default(),
        },
    );

    Expr::Module { symbols }
}