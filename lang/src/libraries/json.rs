use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

pub fn new(_: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    Ok(InternalFunctionResponse {
        return_value: Expr::Object { properties: std::collections::HashMap::new() },
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


    Expr::Module { symbols }
}