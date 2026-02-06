use std::io::{Read, Seek, SeekFrom, Write};
use std::fs::OpenOptions;
use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

pub fn open(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let path = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "open expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .map_err(|e| (
            format!("Failed to open file: {}", e),
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

    Expr::Module { symbols }
}

// file functions

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

pub fn write(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let file = match &args[0].node {
        Expr::File(file) => file,
        _ => return Err((
            "write expects a file argument".to_string(),
            args[0].span,
        )),
    };

    let content = match &args[1].node {
        Expr::String(s) => s,
        _ => return Err((
            "write expects a string as the second argument".to_string(),
            args[1].span,
        )),
    };
    
    (&**file).seek(SeekFrom::Start(0)).map_err(|e| (
        format!("Failed to seek to beginning of file: {}", e),
        args[0].span,
    ))?;

    (&**file).write_all(content.as_bytes()).map_err(|e| (
        format!("Failed to write to file: {}", e),
        args[0].span,
    ))?;

    file.set_len(content.len() as u64).map_err(|e| (
        format!("Failed to truncate file: {}", e),
        args[0].span,
    ))?;

    (&**file).flush().map_err(|e| (
        format!("Failed to flush file: {}", e),
        args[0].span,
    ))?;

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: None,
    })
}

pub fn append(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let file = match &args[0].node {
        Expr::File(file) => file,
        _ => return Err((
            "append expects a file argument".to_string(),
            args[0].span,
        )),
    };

    let content = match &args[1].node {
        Expr::String(s) => s,
        _ => return Err((
            "append expects a string as the second argument".to_string(),
            args[1].span,
        )),
    };
    
    (&**file).write_all(content.as_bytes()).map_err(|e| (
        format!("Failed to write to file: {}", e),
        args[0].span,
    ))?;

    (&**file).flush().map_err(|e| (
        format!("Failed to flush file: {}", e),
        args[0].span,
    ))?;

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: None,
    })
}

pub fn close(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let file = match &args[0].node {
        Expr::File(file) => file.clone(),
        _ => return Err((
            "close expects a file argument".to_string(),
            args[0].span,
        )),
    };
    
    std::io::Write::flush(&mut &*file).map_err(|e| (
        format!("Failed to flush file: {}", e),
        args[0].span,
    ))?;

    file.sync_all().map_err(|e| (
        format!("Failed to sync file: {}", e),
        args[0].span,
    ))?;
    
    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: Some(Expr::Null),
    })
}

pub fn get_fn(name: &str) -> Option<Expr> {
    Some(Expr::InternalFunction {
        name: name.to_string(),
        args: match name {
            "read" => vec!["self".to_string()],
            "write" => vec!["self".to_string(), "content".to_string()],
            "append" => vec!["self".to_string(), "content".to_string()],
            "close" => vec!["self".to_string()],
            _ => vec![],
        },
        func: match name {
            "read" => read,
            "write" => write,
            "append" => append,
            "close" => close,
            _ => return None,
        },
    })
}