use logos::Logos;
use chumsky::span::SimpleSpan;

pub type Span = SimpleSpan;

#[derive(Default, Debug, PartialEq, Clone)]
pub enum LexingError {
    #[default]
    UnexpectedToken,
    InvalidInteger(String)
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
        LexingError::InvalidInteger("Invalid float literal".to_string())
    }
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(error = LexingError)]
pub enum Token {
    #[token("let")]
    Let,

    #[token("=")]
    Equals,

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

    #[token(",")]
    Comma,
    
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBracket,

    #[token("}")]
    RBracket,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    #[regex(r"//[^\n]*", logos::skip, allow_greedy = true)]
    Comment,
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
        }
    }
}