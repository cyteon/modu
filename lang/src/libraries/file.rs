use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

pub fn open(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let path = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "open expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let file = std::fs::File::open(path).map_err(|e| (
        format!("Failed to open file '{}': {}", path, e),
        args[0].span,
    ))?;

    Ok(InternalFunctionResponse {
        return_value: Expr::File(std::sync::Arc::new(file)),
        replace_self: None,
    })
}

pub fn create(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let path = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "create expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let file = std::fs::File::create(path).map_err(|e| (
        format!("Failed to create file '{}': {}", path, e),
        args[0].span,
    ))?;

    Ok(InternalFunctionResponse {
        return_value: Expr::File(std::sync::Arc::new(file)),
        replace_self: None,
    })
}

pub fn get_object() -> Expr {
    let mut symbols = std::collections::HashMap::new();

    symbols.insert(
        "open".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "open".to_string(),
                args: vec!["path".to_string()],
                func: open,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "create".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "create".to_string(),
                args: vec!["path".to_string()],
                func: create,
            },
            span: Span::default(),
        },
    );

    Expr::Module { symbols }
}