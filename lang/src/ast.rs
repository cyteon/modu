use std::collections::HashMap;
use crate::lexer::Span;

pub type SpannedExpr = Spanned<Expr>;
#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum AssignOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
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

    Assign {
        target: Box<Spanned<Expr>>,
        value: Box<Spanned<Expr>>,
        operator: Option<AssignOp>,
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
        alias: Option<String>,
    },

    Object {
        properties: HashMap<String, Spanned<Expr>>,
    },

    If(Vec<(Option<Spanned<Expr>>, Spanned<Expr>)>), // (condition, block)

    InfiniteLoop {
        body: Box<Spanned<Expr>>,
    },

    ForLoop {
        iterator_name: String,
        iterator_range: Box<Spanned<Expr>>,
        body: Box<Spanned<Expr>>,
    },

    WhileLoop {
        condition: Box<Spanned<Expr>>,
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
    In(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    NotIn(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    And(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Or(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Not(Box<Spanned<Expr>>),

    Class {
        name: String,
        methods: Vec<Spanned<Expr>>,
    }
}
