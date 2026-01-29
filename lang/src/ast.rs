use crate::lexer::Span;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    String(String),
    Identifier(String),
    Bool(bool),
    Null,

    Call {
        name: String,
        args: Vec<Spanned<Expr>>,
    },

    Let {
        name: String,
        value: Box<Spanned<Expr>>,
    }
}

pub type SpannedExpr = Spanned<Expr>;