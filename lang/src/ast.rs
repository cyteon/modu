use std::collections::HashMap;
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use libloading::Library;

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

    Neg(Box<Spanned<Expr>>),
    Add(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Sub(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Mul(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Div(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Mod(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Pow(Box<Spanned<Expr>>, Box<Spanned<Expr>>),

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

    IndexAccess {
        object: Box<Spanned<Expr>>,
        index: Box<Spanned<Expr>>, // either abc[0] or abc["key"]
    },

    Block(Vec<Spanned<Expr>>),
    Array(Vec<Spanned<Expr>>),

    InternalFunction {
        name: String,
        args: Vec<String>, // Vec<"__args__"> for an optional amount
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

    #[cfg(not(target_arch = "wasm32"))]
    FFILibrary(Arc<Library>),

    #[cfg(not(target_arch = "wasm32"))]
    File(Arc<std::fs::File>),

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
            Expr::String(s) => {
                let processed = Self::process_escape_sequences(s);
                write!(f, "{}", processed)
            },
            Expr::Identifier(name) => write!(f, "{}", name),
            Expr::Bool(b) => write!(f, "{}", b),
            Expr::Null => write!(f, "null"),

            Expr::Array(elements) => {
                write!(f, "[")?;

                for (i, element) in elements.iter().enumerate() {
                    if let Expr::String(s) = &element.node {
                        let processed = Self::process_escape_sequences(s);
                        write!(f, "\"{}\"", processed)?;
                    } else {
                        write!(f, "{}", element.node)?;
                    }

                    if i != elements.len() - 1 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "]")
            }

            Expr::Object { properties } => {
                write!(f, "{{")?;

                let mut first = true;
                for (key, value) in properties {
                    if !first {
                        write!(f, ", ")?;
                    }

                    first = false;

                    let value_str = match value {
                        Expr::String(s) => format!("\"{}\"", Self::process_escape_sequences(s)),
                        Expr::Int(n) => n.to_string(),
                        Expr::Float(fl) => fl.to_string(),
                        Expr::Bool(b) => b.to_string(),
                        Expr::Array(_) => format!("{}", value),
                        Expr::Null => "null".to_string(),
                        _ => "\"<complex value>\"".to_string(),
                    };

                    write!(f, "\"{}\": {}", key, value_str)?;
                }

                write!(f, "}}")
            }

            _ => write!(f, "{:?}", self),
        }
    }
}

impl Expr {
    fn process_escape_sequences(s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars();
        
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                if let Some(next) = chars.next() {
                    match next {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        'x' => {
                            let hex: String = chars.by_ref().take(2).collect();
                            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                                result.push(byte as char);
                            } else {
                                result.push('\\');
                                result.push('x');
                                result.push_str(&hex);
                            }
                        }
                        _ => {
                            result.push('\\');
                            result.push(next);
                        }
                    }
                } else {
                    result.push('\\');
                }
            } else {
                result.push(ch);
            }
        }
        
        result
    }
}