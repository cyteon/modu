use std::process::Command;
use std::collections::HashMap;
use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

pub fn exec(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, crate::lexer::Span)> {
    let command_str = match &args[0].node {
        Expr::String(s) => s.clone(),
        _ => return Err((
            "os.exec expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let output = {
        #[cfg(windows)]
        {
            Command::new("cmd")
                .args(&["/C", &command_str])
                .creation_flags(0x08000000)
                .output()
        }

        #[cfg(not(windows))]
        {
            Command::new("sh")
                .arg("-c")
                .arg(&command_str)
                .output()
        }
    }.map_err(|e| (
        format!("failed to execute command: {}", e),
        args[0].span,
    ))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let status_code = output.status.code().unwrap_or(-1);

    let obj = Expr::Object {
        properties: {
            let mut map = HashMap::new();

            map.insert("stdout".to_string(), SpannedExpr { node: Expr::String(stdout), span: args[0].span, });
            map.insert("stderr".to_string(), SpannedExpr { node: Expr::String(stderr), span: args[0].span, });
            map.insert("status_code".to_string(), SpannedExpr { node: Expr::Int(status_code as i64), span: args[0].span, });
            map.insert("success".to_string(), SpannedExpr { node: Expr::Bool(output.status.success()), span: args[0].span, });

            map
        },
    };

    Ok(InternalFunctionResponse {
        return_value: obj,
        replace_self: None,
    })
}

pub fn pid(_args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, crate::lexer::Span)> {
    let pid = std::process::id() as i64;

    Ok(InternalFunctionResponse {
        return_value: Expr::Int(pid),
        replace_self: None,
    })
}

pub fn uid(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, crate::lexer::Span)> {
    #[cfg(unix)]
    {
        use libc::getuid;
        let uid = unsafe { getuid() } as i64;

        Ok(InternalFunctionResponse {
            return_value: Expr::Int(uid),
            replace_self: None,
        })
    }

    #[cfg(not(unix))]
    {
        Err((
            "os.uid is not supported on this platform".to_string(),
            args[0].span,
        ))
    }
}

pub fn gid(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, crate::lexer::Span)> {
    #[cfg(unix)]
    {
        use libc::getgid;
        let gid = unsafe { getgid() } as i64;

        Ok(InternalFunctionResponse {
            return_value: Expr::Int(gid),
            replace_self: None,
        })
    }

    #[cfg(not(unix))]
    {
        Err((
            "os.gid is not supported on this platform".to_string(),
            args[0].span,
        ))
    }
}

pub fn getenv(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, crate::lexer::Span)> {
    let var_name = match &args[0].node {
        Expr::String(s) => s.clone(),

        _ => return Err((
            "os.getenv expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let value = match std::env::var(&var_name) {
        Ok(val) => Expr::String(val),
        Err(_) => Expr::Null,
    };

    Ok(InternalFunctionResponse {
        return_value: value,
        replace_self: None,
    })
}

pub fn setenv(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, crate::lexer::Span)> {
    let var_name = match &args[0].node {
        Expr::String(s) => s.clone(),

        _ => return Err((
            "os.setenv expects the first argument to be a string".to_string(),
            args[0].span,
        )),
    };

    let var_value = match &args[1].node {
        Expr::String(s) => s.clone(),

        _ => return Err((
            "os.setenv expects the second argument to be a string".to_string(),
            args[1].span,
        )),
    };

    unsafe {
        std::env::set_var(var_name, var_value);
    }

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: None,
    })
}

pub fn unsetenv(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, crate::lexer::Span)> {
    let var_name = match &args[0].node {
        Expr::String(s) => s.clone(),

        _ => return Err((
            "os.unsetenv expects a string argument".to_string(),
            args[0].span,
        )),
    };

    unsafe {
        std::env::remove_var(var_name);
    }

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: None,
    })
}

pub fn get_object() -> Expr {
    let mut symbols = std::collections::HashMap::new();

    symbols.insert(
        "exec".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "exec".to_string(),
                args: vec!["cmd".to_string()],
                func: exec,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "pid".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "pid".to_string(),
                args: vec![],
                func: pid,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "uid".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "uid".to_string(),
                args: vec![],
                func: uid,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "gid".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "gid".to_string(),
                args: vec![],
                func: gid,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "getenv".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "getenv".to_string(),
                args: vec!["var_name".to_string()],
                func: getenv,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "setenv".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "setenv".to_string(),
                args: vec!["var_name".to_string(), "var_value".to_string()],
                func: setenv,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "unsetenv".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "unsetenv".to_string(),
                args: vec!["var_name".to_string()],
                func: unsetenv,
            },
            span: Span::default(),
        },
    );

    // vars

    symbols.insert(
        "name".to_string(),
        SpannedExpr {
            node: Expr::String({
                if cfg!(target_os = "windows") {
                    "windows"
                } else if cfg!(target_os = "linux") {
                    "linux"
                } else if cfg!(target_os = "macos") {
                    "macos"
                } else {
                    "unknown"
                }.to_string()
            }),
            span: Span::default(),
        },
    );

    Expr::Module { symbols }
}