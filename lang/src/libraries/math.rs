use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

pub fn mul(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    if args.len() != 2 {
        return Err((
            "mul takes exactly two arguments".to_string(),
            args[0].span,
        ));
    }

    match (&args[0].node, &args[1].node) {
        (Expr::Int(a), Expr::Int(b)) => {
            let result = a * b;
            Ok(InternalFunctionResponse {
                return_value: Expr::Int(result),
                replace_self: None,
            })
        }

        (Expr::Float(a), Expr::Float(b)) => {
            let result = a * b;
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        (Expr::Int(a), Expr::Float(b)) => {
            let result = (*a as f64) * b;
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        (Expr::Float(a), Expr::Int(b)) => {
            let result = a * (*b as f64);
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        _ => Err((
            "mul expects number arguments".to_string(),
            args[0].span,
        )),
    }
}

pub fn div(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    if args.len() != 2 {
        return Err((
            "div takes exactly two arguments".to_string(),
            args[0].span,
        ));
    }

    match (&args[0].node, &args[1].node) {
        (Expr::Int(a), Expr::Int(b)) => {
            if *b == 0 {
                return Err((
                    "division by zero".to_string(),
                    args[1].span,
                ));
            }
            let result = (*a as f64) / (*b as f64);
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        (Expr::Float(a), Expr::Float(b)) => {
            if *b == 0.0 {
                return Err((
                    "division by zero".to_string(),
                    args[1].span,
                ));
            }
            let result = a / b;
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        (Expr::Int(a), Expr::Float(b)) => {
            if *b == 0.0 {
                return Err((
                    "division by zero".to_string(),
                    args[1].span,
                ));
            }
            let result = (*a as f64) / b;
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        (Expr::Float(a), Expr::Int(b)) => {
            if *b == 0 {
                return Err((
                    "division by zero".to_string(),
                    args[1].span,
                ));
            }
            let result = a / (*b as f64);
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        v => Err((
            "div expects number arguments".to_string(),
            args[0].span,
        )),
    }
}

pub fn abs(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    if args.len() != 1 {
        return Err((
            "abs takes exactly one argument".to_string(),
            args[0].span,
        ));
    }

    match &args[0].node {
        Expr::Int(n) => {
            let abs_value = n.abs();
            Ok(InternalFunctionResponse {
                return_value: Expr::Int(abs_value),
                replace_self: None,
            })
        }

        Expr::Float(f) => {
            let abs_value = f.abs();
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(abs_value),
                replace_self: None,
            })
        }

        _ => Err((
            "abs expects a number argument".to_string(),
            args[0].span,
        )),
    }
}

pub fn get_object() -> Expr {
    let mut symbols = std::collections::HashMap::new();

    symbols.insert(
        "mul".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "mul".to_string(),
                args: vec!["a".to_string(), "b".to_string()],
                func: mul,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "div".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "div".to_string(),
                args: vec!["a".to_string(), "b".to_string()],
                func: div,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "abs".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "abs".to_string(),
                args: vec!["x".to_string()],
                func: abs,
            },
            span: Span::default(),
        },
    );

    Expr::Module { symbols }
}