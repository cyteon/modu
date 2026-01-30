use std::collections::HashMap;
use crate::lexer::Span;

pub type SpannedExpr = Spanned<Expr>;
#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct InternalFunctionResponse {
    pub return_value: Expr,
    pub replace_self: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    String(String),
    Identifier(String),
    Bool(bool),
    Return(Box<Spanned<Expr>>),
    Null,
    Break,
    Continue,
    Star,

    Neg(Box<Spanned<Expr>>),
    Add(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Sub(Box<Spanned<Expr>>, Box<Spanned<Expr>>),

    Let {
        name: String,
        value: Box<Spanned<Expr>>,
    },

    Call {
        callee: Box<Spanned<Expr>>,
        args: Vec<Spanned<Expr>>,
    },

    PropertyAccess {
        object: Box<Spanned<Expr>>,
        property: String,
    },

    Block(Vec<Spanned<Expr>>),

    InternalFunction {
        name: String,
        args: Vec<String>, // or __args__ for an optional amount
        func: fn(Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)>,
    },

    Function {
        name: String,
        args: Vec<String>,
        body: Box<Spanned<Expr>>,
    },

    // import "module" as module;
    // or import "module" as *; // you can use like function() instead of module.function()
    // or import "module"; // will import as the module name
    Import {
        name: String,
        import_as: Option<String>,
    },

    Module {
        symbols: HashMap<String, Spanned<Expr>>,
    },

    Object {
        properties: HashMap<String, Expr>,
    },

    If {
        condition: Box<Spanned<Expr>>,
        then_branch: Box<Spanned<Expr>>,
        else_branch: Option<Box<Spanned<Expr>>>,
    },

    InfiniteLoop {
        body: Box<Spanned<Expr>>,
    },

    ForLoop {
        iterator_name: String,
        iterator_range: Box<Spanned<Expr>>,
        body: Box<Spanned<Expr>>,
    },

    Range {
        start: Box<Spanned<Expr>>,
        end: Box<Spanned<Expr>>,
    },

    InclusiveRange {
        start: Box<Spanned<Expr>>,
        end: Box<Spanned<Expr>>,
    },
    
    Equal(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    NotEqual(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    LessThan(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    LessThanOrEqual(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    GreaterThan(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    GreaterThanOrEqual(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Int(n) => write!(f, "{}", n),
            Expr::Float(fl) => write!(f, "{}", fl),
            Expr::String(s) => write!(
                f, "{}", 
                s.replace("\\n", "\n")
                    .replace("\\t", "\t")
                    .replace("\\\"", "\"")
                    .replace("\\\\", "\\")
            ),
            Expr::Identifier(name) => write!(f, "{}", name),
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::Null => write!(f, "null"),

            _ => write!(f, "{:?}", self),
        }
    }
}