use std::io::{Read, Seek, SeekFrom, Write};
use std::fs::OpenOptions;
use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

pub fn open(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let path = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "file.open expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .map_err(|e| (
            format!("failed to open file: {}", e),
            args[0].span,
        ))?;
    
    Ok(InternalFunctionResponse {
        return_value: Expr::File(std::sync::Arc::new(file)),
        replace_self: None,
    })
}

pub fn exists(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let path = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "fs.exists expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let exists = std::path::Path::new(path).exists();

    Ok(InternalFunctionResponse {
        return_value: Expr::Bool(exists),
        replace_self: None,
    })
}

pub fn mkdir(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let path = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "fs.mkdir expects a string argument".to_string(),
            args[0].span,
        )),
    };

    match std::fs::create_dir(path) {
        Ok(_) => (),
        Err(e) => {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                return Err((
                    format!("failed to create directory: {}", e),
                    args[0].span,
                ))
            }
        }
    }

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: None,
    })
}

pub fn rmdir(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let path = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "fs.rmdir expects a string argument".to_string(),
            args[0].span,
        )),
    };

    std::fs::remove_dir(path).map_err(|e| (
        format!("failed to remove directory: {}", e),
        args[0].span,
    ))?;

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: None,
    })
}

pub fn remove(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let path = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "fs.remove expects a string argument".to_string(),
            args[0].span,
        )),
    };

    std::fs::remove_file(path).map_err(|e| (
        format!("failed to delete file: {}", e),
        args[0].span,
    ))?;

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
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
        "exists".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "exists".to_string(),
                args: vec!["path".to_string()],
                func: exists,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "mkdir".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "mkdir".to_string(),
                args: vec!["path".to_string()],
                func: mkdir,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "rmdir".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "rmdir".to_string(),
                args: vec!["path".to_string()],
                func: rmdir,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "remove".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "remove".to_string(),
                args: vec!["path".to_string()],
                func: remove,
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
            "file.read expects a file argument".to_string(),
            args[0].span,
        )),
    };

    let mut buf = String::new();
    std::io::BufReader::new(&**file)
        .read_to_string(&mut buf)
        .map_err(|e| (
            format!("failed to read from file: {}", e),
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
            "file.write expects a file argument".to_string(),
            args[0].span,
        )),
    };

    let content = match &args[1].node {
        Expr::String(s) => s,
        _ => return Err((
            "file.write expects a string as the second argument".to_string(),
            args[1].span,
        )),
    };
    
    (&**file).seek(SeekFrom::Start(0)).map_err(|e| (
        format!("failed to seek to beginning of file: {}", e),
        args[0].span,
    ))?;

    (&**file).write_all(content.as_bytes()).map_err(|e| (
        format!("failed to write to file: {}", e),
        args[0].span,
    ))?;

    file.set_len(content.len() as u64).map_err(|e| (
        format!("failed to truncate file: {}", e),
        args[0].span,
    ))?;

    (&**file).flush().map_err(|e| (
        format!("failed to flush file: {}", e),
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
            "file.append expects a file argument".to_string(),
            args[0].span,
        )),
    };

    let content = match &args[1].node {
        Expr::String(s) => s,
        _ => return Err((
            "file.append expects a string as the second argument".to_string(),
            args[1].span,
        )),
    };
    
    (&**file).write_all(content.as_bytes()).map_err(|e| (
        format!("failed to write to file: {}", e),
        args[0].span,
    ))?;

    (&**file).flush().map_err(|e| (
        format!("failed to flush file: {}", e),
        args[0].span,
    ))?;

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: None,
    })
}

pub fn stat(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let file = match &args[0].node {
        Expr::File(file) => file,
        _ => return Err((
            "file.stat expects a file argument".to_string(),
            args[0].span,
        )),
    };

    let metadata = (&**file).metadata().map_err(|e| (
        format!("failed to get file metadata: {}", e),
        args[0].span,
    ))?;

    let file_type = if metadata.is_file() {
        "file"
    } else if metadata.is_dir() {
        "directory"
    } else {
        "other"
    };

    let obj = Expr::Object {
        properties: {
            let mut map = std::collections::HashMap::new();

            map.insert("type".to_string(), SpannedExpr {
                node: Expr::String(file_type.to_string()),
                span: Span::default(),
            });
            
            map.insert("size".to_string(), SpannedExpr {
                node: Expr::Int(metadata.len() as i64),
                span: Span::default(),
            });

            map.insert("readonly".to_string(), SpannedExpr {
                node: Expr::Bool(metadata.permissions().readonly()),
                span: Span::default(),
            });

            map.insert("created".to_string(), match metadata.created() {
                Ok(time) => SpannedExpr {
                    node: Expr::Int(time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64),
                    span: Span::default(),
                },
                Err(_) => SpannedExpr {
                    node: Expr::Null,
                    span: Span::default(),
                },
            });

            map.insert("modified".to_string(), match metadata.modified() {
                Ok(time) => SpannedExpr {
                    node: Expr::Int(time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64),
                    span: Span::default(),
                },
                Err(_) => SpannedExpr {
                    node: Expr::Null,
                    span: Span::default(),
                },
            });

            map
        }
    };

    Ok(InternalFunctionResponse {
        return_value: obj,
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
        format!("failed to flush file: {}", e),
        args[0].span,
    ))?;

    file.sync_all().map_err(|e| (
        format!("failed to sync file: {}", e),
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
            "stat" => vec!["self".to_string()],
            _ => vec![],
        },
        func: match name {
            "read" => read,
            "write" => write,
            "append" => append,
            "close" => close,
            "stat" => stat,
            _ => return None,
        },
    })
}