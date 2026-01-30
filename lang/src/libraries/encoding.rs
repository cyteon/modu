use std::collections::HashMap;

use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

pub fn encode_base64(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let input = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "encode_base64 expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let encoded = base64::encode(input);

    Ok(InternalFunctionResponse {
        return_value: Expr::String(encoded),
        replace_self: None,
    })
}

pub fn decode_base64(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let input = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "decode_base64 expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let decoded_bytes = base64::decode(input).map_err(|e| (
        format!("Failed to decode base64 string: {}", e),
        args[0].span,
    ))?;

    let decoded = String::from_utf8(decoded_bytes).map_err(|e| (
        format!("Decoded base64 is not valid UTF-8: {}", e),
        args[0].span,
    ))?;

    Ok(InternalFunctionResponse {
        return_value: Expr::String(decoded),
        replace_self: None,
    })
}

pub fn encode_base16(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let input = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "encode_base16 expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let encoded = base16::encode_lower(input.as_bytes());

    Ok(InternalFunctionResponse {
        return_value: Expr::String(encoded),
        replace_self: None,
    })
}

pub fn decode_base16(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let input = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "decode_base16 expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let decoded_bytes = base16::decode(input).map_err(|e| (
        format!("Failed to decode base16 string: {}", e),
        args[0].span,
    ))?;

    let decoded = String::from_utf8(decoded_bytes).map_err(|e| (
        format!("Decoded base16 is not valid UTF-8: {}", e),
        args[0].span,
    ))?;

    Ok(InternalFunctionResponse {
        return_value: Expr::String(decoded),
        replace_self: None,
    })
}

pub fn get_object() -> Expr {
    let mut symbols = HashMap::new();

    symbols.insert(
        "encode_base64".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "encode_base64".to_string(),
                args: vec!["str".to_string()],
                func: encode_base64,
            },

            span: Span::default(),
        },
    );

    symbols.insert(
        "decode_base64".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "decode_base64".to_string(),
                args: vec!["str".to_string()],
                func: decode_base64,
            },

            span: Span::default(),
        },
    );

    symbols.insert(
        "encode_base16".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "encode_base16".to_string(),
                args: vec!["str".to_string()],
                func: encode_base16,
            },

            span: Span::default(),
        },
    );

    symbols.insert(
        "decode_base16".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "decode_base16".to_string(),
                args: vec!["str".to_string()],
                func: decode_base16,
            },

            span: Span::default(),
        },
    );

    Expr::Module { symbols }
}