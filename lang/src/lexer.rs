use logos::Logos;
use chumsky::span::SimpleSpan;

pub type Span = SimpleSpan;

#[derive(Default, Debug, PartialEq, Clone)]
pub enum LexingError {
    #[default]
    UnexpectedToken,
    InvalidInteger(String),
    InvalidFloat(String),
}

impl From<std::num::ParseIntError> for LexingError {
    fn from(err: std::num::ParseIntError) -> Self {
        use std::num::IntErrorKind;

        match err.kind() {
            IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                LexingError::InvalidInteger("Integer literal out of range".to_string())
            }
            _ => LexingError::InvalidInteger("Invalid integer literal".to_string()),
        }
    }
}

impl From<std::num::ParseFloatError> for LexingError {
    fn from(_err: std::num::ParseFloatError) -> Self {
        LexingError::InvalidFloat("Invalid float literal".to_string())
    }
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(error = LexingError)]
pub enum Token {
    #[token("let")]
    Let,

    #[token("=")]
    Assign,

    #[token(";")]
    Semicolon,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex("[0-9]+", |lex| lex.slice().parse::<i64>())]
    Int(i64),

    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>())]
    Float(f64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        lex.slice()[1..lex.slice().len()-1].to_string()
    })]
    String(String),

    #[regex("true|false", |lex| lex.slice() == "true")]
    Bool(bool),

    #[token("null")]
    Null,

    #[token("return")]
    Return,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("fn")]
    Function,

    #[token("import")]
    Import,

    #[token("as")]
    As,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("loop")]
    Loop,

    #[token("for")]
    For,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token("*")]
    Star,
    
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("..")]
    Range,

    #[token("..=")]
    InclusiveRange,

    #[token("==")]
    DoubleEqual,

    #[token("!=")]
    NotEqual,

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanOrEqual,

    #[token(">")]
    GreaterThan,

    #[token(">=")]
    GreaterThanOrEqual,

    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    Whitespace,

    #[regex(r"//[^\n]*", logos::skip, allow_greedy = true)]
    Comment,

    #[regex(r"/\*([^*]|\*+[^*/])*\*+/", logos::skip, allow_greedy = true)]
    MultiLineComment,
}

pub fn lex(input: &str) -> Result<Vec<(Token, Span)>, (LexingError, Span)> {
    let mut lexer = Token::lexer(input);
    let mut tokens: Vec<(Token, Span)> = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Err(e) => return Err((e, SimpleSpan::from(lexer.span()))),

            Ok(v) => {
                tokens.push((v, SimpleSpan::from(lexer.span())));
            }
        }
    }

    Ok(tokens)
}

impl std::fmt::Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexingError::UnexpectedToken => write!(f, "Unexpected token"),
            LexingError::InvalidInteger(msg) => write!(f, "Invalid integer: {}", msg),
            LexingError::InvalidFloat(msg) => write!(f, "Invalid float: {}", msg),
        }
    }
}