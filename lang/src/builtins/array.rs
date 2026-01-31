use crate::{ast::{Expr, InternalFunctionResponse, Spanned}, lexer::Span};

pub fn len(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let array = match &args[0].node {
        Expr::Array(elements) => elements,
        _ => unreachable!(),
    };

    let length = array.len() as i64;
    
    Ok(InternalFunctionResponse {
        return_value: Expr::Int(length),
        replace_self: None,
    })
}

pub fn get_fn(name: &str) -> Option<Expr> {
    Some(Expr::InternalFunction {
        name: name.to_string(),
        args: match name {
            "len" => vec!["self".to_string()],
            _ => vec![],
        },
        func: match name {
            "len" => len,
            _ => return None,
        },
    })
}