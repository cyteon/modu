use std::collections::HashMap;

use crate::ast::{Expr, SpannedExpr};
use crate::lexer::Span;

#[derive(Debug)]
pub struct EvalError {
    pub message: String,
    pub span: Span,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn eval<'src>(expr: &'src SpannedExpr, context: &mut HashMap<String, Expr>) -> Result<Expr, EvalError> {    
    match &expr.node {
        Expr::Int(n) => Ok(Expr::Int(*n)),
        Expr::Float(f) => Ok(Expr::Float(*f)),
        Expr::String(s) => Ok(Expr::String(s.clone())),
        Expr::Bool(b) => Ok(Expr::Bool(*b)),
        Expr::Null => Ok(Expr::Null),

        Expr::Identifier(name) => {
            Err(EvalError {
                message: format!("identifier kaboom: {}", name),
                span: expr.span,
            })
        }

        Expr::Call { name, args } => {
            Err(EvalError {
                message: format!("kaboom: {}", name),
                span: expr.span,
            })
        }

        Expr::Let { name, value } => {
            context.insert(name.clone(), (*value).node.clone());

            Ok(Expr::Null)
        }
    }
}