use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

pub fn mul(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
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
            "math.mul expects number arguments".to_string(),
            args[0].span,
        )),
    }
}

pub fn div(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
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

        _ => Err((
            "math.div expects number arguments".to_string(),
            args[0].span,
        )),
    }
}

pub fn abs(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
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
            "math.abs expects a number argument".to_string(),
            args[0].span,
        )),
    }
}

pub fn pow(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    match (&args[0].node, &args[1].node) {
        (Expr::Int(a), Expr::Int(b)) => {
            let result = a.pow(*b as u32);
            Ok(InternalFunctionResponse {
                return_value: Expr::Int(result),
                replace_self: None,
            })
        }

        (Expr::Float(a), Expr::Float(b)) => {
            let result = a.powf(*b);
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        (Expr::Int(a), Expr::Float(b)) => {
            let result = (*a as f64).powf(*b);
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        (Expr::Float(a), Expr::Int(b)) => {
            let result = a.powf(*b as f64);
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        _ => Err((
            "math.pow expects number arguments".to_string(),
            args[0].span,
        )),
    }
}

pub fn floor(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    match &args[0].node {
        Expr::Float(f) => {
            let result = f.floor();
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        _ => Err((
            "math.floor expects a float argument".to_string(),
            args[0].span,
        )),
    }
}

pub fn ceil(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    match &args[0].node {
        Expr::Float(f) => {
            let result = f.ceil();
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        _ => Err((
            "math.ceil expects a float argument".to_string(),
            args[0].span,
        )),
    }
}

pub fn round(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    match &args[0].node {
        Expr::Float(f) => {
            let result = f.round();
            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        _ => Err((
            "math.round expects a float argument".to_string(),
            args[0].span,
        )),
    }
}

pub fn sqrt(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    match &args[0].node {
        Expr::Float(f) => {
            if *f < 0.0 {
                return Err((
                    "cannot take square root of a negative number".to_string(),
                    args[0].span,
                ));
            }

            let result = f.sqrt();

            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        Expr::Int(n) => {
            if *n < 0 {
                return Err((
                    "cannot take square root of a negative number".to_string(),
                    args[0].span,
                ));
            }

            let result = (*n as f64).sqrt();

            Ok(InternalFunctionResponse {
                return_value: Expr::Float(result),
                replace_self: None,
            })
        }

        _ => Err((
            "math.sqrt expects a number argument".to_string(),
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

    symbols.insert(
        "pow".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "pow".to_string(),
                args: vec!["base".to_string(), "exponent".to_string()],
                func: pow,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "floor".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "floor".to_string(),
                args: vec!["x".to_string()],
                func: floor,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "ceil".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "ceil".to_string(),
                args: vec!["x".to_string()],
                func: ceil,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "round".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "round".to_string(),
                args: vec!["x".to_string()],
                func: round,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "sqrt".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "sqrt".to_string(),
                args: vec!["x".to_string()],
                func: sqrt,
            },
            span: Span::default(),
        },
    );

    // constants
    symbols.insert(
        "PI".to_string(),
        SpannedExpr {
            node: Expr::Float(std::f64::consts::PI),
            span: Span::default(),
        },
    );

    symbols.insert(
        "E".to_string(),
        SpannedExpr {
            node: Expr::Float(std::f64::consts::E),
            span: Span::default(),
        },
    );

    Expr::Module { symbols }
}