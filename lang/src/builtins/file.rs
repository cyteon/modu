use std::io::Read;
use crate::{ast::{Expr, InternalFunctionResponse, Spanned}, lexer::Span};

pub fn read(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let file = match &args[0].node {
        Expr::File(file) => file,
        _ => return Err((
            "read expects a file argument".to_string(),
            args[0].span,
        )),
    };

    let mut buf = String::new();
    std::io::BufReader::new(&**file)
        .read_to_string(&mut buf)
        .map_err(|e| (
            format!("Failed to read from file: {}", e),
            args[0].span,
        ))?;

    Ok(InternalFunctionResponse {
        return_value: Expr::String(buf),
        replace_self: None,
    })
}

pub fn get_fn(name: &str) -> Option<Expr> {
    Some(Expr::InternalFunction {
        name: name.to_string(),
        args: match name {
            "read" => vec!["self".to_string()],
            _ => vec![],
        },
        func: match name {
            "read" => read,
            _ => return None,
        },
    })
}