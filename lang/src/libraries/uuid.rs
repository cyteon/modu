use crate::ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr};

pub fn v4(_: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, crate::lexer::Span)> {
    let uuid = uuid::Uuid::new_v4();

    Ok(InternalFunctionResponse {
        return_value: SpannedExpr {
            node: Expr::String(uuid.to_string()),
            span: crate::lexer::Span::default(),
        },
        replace_self: None,
    })
}

pub fn v7(_: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, crate::lexer::Span)> {
    let uuid = uuid::Uuid::now_v7();

    Ok(InternalFunctionResponse {
        return_value: SpannedExpr {
            node: Expr::String(uuid.to_string()),
            span: crate::lexer::Span::default(),
        },
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
            span: crate::lexer::Span::default(),
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
            span: crate::lexer::Span::default(),
        },
    );

    Expr::Module { symbols }
}