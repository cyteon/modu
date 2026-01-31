use std::{collections::HashMap, time};
use chrono::{DateTime, Local};

use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

pub fn now_unix(_: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| (format!("System time error: {}", e), Span::default()))?
        .as_secs() as i64;

    Ok(InternalFunctionResponse {
        return_value: Expr::Int(now),
        replace_self: None,
    })
}

pub fn now_utc(_: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let now = time::SystemTime::now();
    let datetime: DateTime<chrono::Utc> = now.into();

    Ok(InternalFunctionResponse {
        return_value: Expr::String(format!("{}", datetime.format("%c"))),
        replace_self: None,
    })
}

pub fn now_local(_: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let now = time::SystemTime::now();
    let datetime: DateTime<chrono::Local> = now.into();

    Ok(InternalFunctionResponse {
        return_value: Expr::String(format!("{}", datetime.format("%c"))),
        replace_self: None,
    })
}

pub fn to_iso_8601(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let time = match args[0].node {
        Expr::Int(n) => n,
        _ => return Err(("to_iso_8601 expects an integer unix timestamp".to_string(), args[0].span)),
    };

    let time = time::UNIX_EPOCH + time::Duration::from_secs(time as u64);
    let time: DateTime<Local> = time.into();
    
    Ok(InternalFunctionResponse {
        return_value: Expr::String(time.to_rfc3339()),
        replace_self: None,
    })
}

pub fn to_rfc_2822(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let time = match args[0].node {
        Expr::Int(n) => n,
        _ => return Err(("to_rfc_2822 expects an integer unix timestamp".to_string(), args[0].span)),
    };

    let time = time::UNIX_EPOCH + time::Duration::from_secs(time as u64);
    let time: DateTime<Local> = time.into();
    
    Ok(InternalFunctionResponse {
        return_value: Expr::String(time.to_rfc2822()),
        replace_self: None,
    })
}

pub fn to_local_date_time(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let time = match args[0].node {
        Expr::Int(n) => n,
        _ => return Err(("to_local_date_time expects an integer unix timestamp".to_string(), args[0].span)),
    };

    let time = time::UNIX_EPOCH + time::Duration::from_secs(time as u64);
    let time: DateTime<Local> = time.into();
    
    Ok(InternalFunctionResponse {
        return_value: Expr::String(format!("{}", time.format("%c"))),
        replace_self: None,
    })
}

pub fn to_utc_date_time(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let time = match args[0].node {
        Expr::Int(n) => n,
        _ => return Err(("to_utc_date_time expects an integer unix timestamp".to_string(), args[0].span)),
    };

    let time = time::UNIX_EPOCH + time::Duration::from_secs(time as u64);
    let time: DateTime<chrono::Utc> = time.into();
    
    Ok(InternalFunctionResponse {
        return_value: Expr::String(format!("{}", time.format("%c"))),
        replace_self: None,
    })
}

pub fn get_object() -> Expr {
    let mut symbols = HashMap::new();

    symbols.insert(
        "now_unix".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "now_unix".to_string(),
                args: vec![],
                func: now_unix,
            },

            span: Span::default(),
        },
    );

    symbols.insert(
        "now_utc".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "now_utc".to_string(),
                args: vec![],
                func: now_utc,
            },

            span: Span::default(),
        },
    );

    symbols.insert(
        "now_local".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "now_local".to_string(),
                args: vec![],
                func: now_local,
            },

            span: Span::default(),
        },
    );

    symbols.insert(
        "to_iso_8601".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "to_iso_8601".to_string(),
                args: vec!["unix_timestamp".to_string()],
                func: to_iso_8601,
            },

            span: Span::default(),
        },
    );

    symbols.insert(
        "to_rfc_2822".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "to_rfc_2822".to_string(),
                args: vec!["unix_timestamp".to_string()],
                func: to_rfc_2822,
            },

            span: Span::default(),
        },
    );

    symbols.insert(
        "to_local_date_time".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "to_local_date_time".to_string(),
                args: vec!["unix_timestamp".to_string()],
                func: to_local_date_time,
            },

            span: Span::default(),
        },
    );

    symbols.insert(
        "to_utc_date_time".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "to_utc_date_time".to_string(),
                args: vec!["unix_timestamp".to_string()],
                func: to_utc_date_time,
            },

            span: Span::default(),
        },
    );

    Expr::Module { symbols }
}