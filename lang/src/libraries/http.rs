use std::collections::HashMap;

use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

fn handle_response(response: reqwest::blocking::Response) -> Result<InternalFunctionResponse, (String, Span)> {
    let status = response.status();

    let mut properties = HashMap::new();

    properties.insert(
        "status".to_string(),
        Expr::Int(status.as_u16() as i64)
    );

    properties.insert(
        "status_text".to_string(),
        Expr::String(status.canonical_reason().unwrap_or("").to_string())
    );

    let headers = Expr::Object {
        properties: response.headers().iter().map(|(k, v)| {
            (
                k.to_string(),
                Expr::String(v.to_str().unwrap_or("").to_string())
            )
        }).collect(),
    };

    properties.insert(
        "headers".to_string(),
        headers
    );

    let body = response.text().map_err(|e| (
        format!("failed to read response body: {}", e),
        Span::default(),
    ))?;

    properties.insert(
        "body".to_string(),
        Expr::String(body)
    );

    properties.insert(
        "ok".to_string(),
        Expr::Bool(status.is_success())
    );

    Ok(InternalFunctionResponse {
        return_value: Expr::Object { properties },
        replace_self: None,
    })
}

pub fn get(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let url = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "http.get expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let response = reqwest::blocking::get(url).map_err(|e| (
        format!("failed to perform GET request: {}", e),
        args[0].span,
    ))?;

    handle_response(response)
}

pub fn post(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let url = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "http.post expects a string as the first argument".to_string(),
            args[0].span,
        )),
    };

    let body = match &args[1].node {
        Expr::String(s) => s,
        _ => return Err((
            "http.post expects a string as the second argument".to_string(),
            args[1].span,
        )),
    };

    let client = reqwest::blocking::Client::new();
    let response = client.post(url)
        .body(body.clone())
        .send()
        .map_err(|e| (
            format!("failed to perform POST request: {}", e),
            args[0].span,
        ))?;

    handle_response(response)
}

pub fn put(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let url = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "http.put expects a string as the first argument".to_string(),
            args[0].span,
        )),
    };

    let body = match &args[1].node {
        Expr::String(s) => s,
        _ => return Err((
            "http.put expects a string as the second argument".to_string(),
            args[1].span,
        )),
    };

    let client = reqwest::blocking::Client::new();
    let response = client.put(url)
        .body(body.clone())
        .send()
        .map_err(|e| (
            format!("failed to perform PUT request: {}", e),
            args[0].span,
        ))?;

    handle_response(response)
}

pub fn patch(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let url = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "http.patch expects a string as the first argument".to_string(),
            args[0].span,
        )),
    };

    let body = match &args[1].node {
        Expr::String(s) => s,
        _ => return Err((
            "http.patch expects a string as the second argument".to_string(),
            args[1].span,
        )),
    };

    let client = reqwest::blocking::Client::new();
    let response = client.patch(url)
        .body(body.clone())
        .send()
        .map_err(|e| (
            format!("failed to perform PATCH request: {}", e),
            args[0].span,
        ))?;

    handle_response(response)
}

pub fn delete(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let url = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "http.delete expects a string as the first argument".to_string(),
            args[0].span,
        )),
    };

    let client = reqwest::blocking::Client::new();
    let response = client.delete(url)
        .send()
        .map_err(|e| (
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
                args: vec!["url".to_string()],
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
                args: vec!["url".to_string(), "body".to_string()],
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
                args: vec!["url".to_string(), "body".to_string()],
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
                args: vec!["url".to_string(), "body".to_string()],
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
                args: vec!["url".to_string()],
                func: delete,
            },
            span: Span::default(),
        },
    );

    Expr::Module { symbols }
}