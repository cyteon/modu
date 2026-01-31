use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

pub fn v4(_: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let uuid = uuid::Uuid::new_v4();

    Ok(InternalFunctionResponse {
        return_value: Expr::String(uuid.to_string()),
        replace_self: None,
    })
}

pub fn v7(_: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let uuid = uuid::Uuid::now_v7();

    Ok(InternalFunctionResponse {
        return_value: Expr::String(uuid.to_string()),
        replace_self: None,
    })
}

pub fn get_object() -> Expr {
    let mut symbols = std::collections::HashMap::new();

    symbols.insert(
        "v4".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "v4".to_string(),
                args: vec![],
                func: v4,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "v7".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "v7".to_string(),
                args: vec![],
                func: v7,
            },
            span: Span::default(),
        },
    );

    Expr::Module { symbols }
}