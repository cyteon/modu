use logos::Logos;

#[derive(Default, Debug, PartialEq, Clone)]
pub enum LexingError {
    #[default]
    UnexpectedToken,
    InvalidInteger(String)
}

impl From<std::num::ParseIntError> for LexingError {
    fn from(err: std::num::ParseIntError) -> Self {
        use std::num::IntErrorKind::*;

        match err.kind() {
            PosOverflow | NegOverflow => LexingError::InvalidInteger("Integer overflow".to_string()),
            _ => LexingError::InvalidInteger("Other error".to_string()),
        }
    }
}

impl From<std::num::ParseFloatError> for LexingError {
    fn from(_err: std::num::ParseFloatError) -> Self {
        LexingError::InvalidInteger("Float parsing error".to_string())
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\f\r]+")]
pub enum Token {
    #[regex("//[^\n]*|/\\*([^*]|\\*[^/])*\\*/", allow_greedy = true)]
    Comment,

    #[token("/*")]
    MultiLineCommentStart,

    #[token("*/")]
    MultiLineCommentEnd,

    #[token("let")]
    Let,

    #[token("fn")]
    Fn,

    #[token("import")]
    Import,

    #[token("return")]
    Return,

    #[token("as")]
    As,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("if")]
    If,

    #[token("loop")]
    Loop,

    #[token("for")]
    For,

    #[token("break")]
    Break,

    #[token(".")]
    Dot,

    #[token("..")]
    Range,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifer,

    #[token("=")]
    Assign,

    #[regex("[0-9](?:_?[0-9])*", |lex| {
        lex.slice().replace("_", "").parse::<i64>()
    })]
    Integer(i64),

    #[regex("[0-9]+\\.[0-9]+", |lex| {
        lex.slice().parse::<f64>()
    })]
    Float(f64),

    #[regex(r#""([^"\\]|\\.)*"|'([^'\\]|\\.)*'"#, |lex| {
        let slice = lex.slice();
        let len = slice.len();
        slice[1..len-1].to_string()
    })]
    String(String),

    #[regex("true|false", |lex| {
        match lex.slice() {
            "true" => true,
            "false" => false,
            _ => unreachable!(),
        }
    })]
    Boolean(bool),

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBracket,

    #[token("}")]
    RBracket,

    #[token("[")]
    LSquareBracket,

    #[token("]")]
    RSquareBracket,

    #[token("==")]
    IsEqual,

    #[token("!=")]
    IsUnequal,

    #[token("<")]
    LessThan,

    #[token(">")]
    GreaterThan,

    #[token("<=")]
    LessThanOrEqual,

    #[token(">=")]
    GreaterThanOrEqual,

    #[token("*")]
    Star,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_1() {
        let mut lexer = Token::lexer("\"Hello, world!\"");
        assert_eq!(lexer.next(), Some(Ok(Token::String("Hello, world!".to_string()))));
    }

    #[test]
    fn string_2() {
        let mut lexer = Token::lexer("'Hello, world!'");
        assert_eq!(lexer.next(), Some(Ok(Token::String("Hello, world!".to_string()))));
    }

    #[test]
    fn string_error() {
        let mut lexer = Token::lexer("\"Hello, world!'");
        assert_eq!(lexer.next(), Some(Err(LexingError::UnexpectedToken)));
    }

    #[test]
    fn asing_str() {
        let mut lexer = Token::lexer("let x = \"test\"");
        assert_eq!(lexer.next(), Some(Ok(Token::Let)));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifer)));
        assert_eq!(lexer.next(), Some(Ok(Token::Assign)));
        assert_eq!(lexer.next(), Some(Ok(Token::String("test".to_string()))));
    }

    #[test]
    fn asing_number() {
        let mut lexer = Token::lexer("let x = 10");
        assert_eq!(lexer.next(), Some(Ok(Token::Let)));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifer)));
        assert_eq!(lexer.next(), Some(Ok(Token::Assign)));
        assert_eq!(lexer.next(), Some(Ok(Token::Integer(10))));
    }

    #[test]
    fn asing_boolean() {
        let mut lexer = Token::lexer("let x = true");
        assert_eq!(lexer.next(), Some(Ok(Token::Let)));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifer)));
        assert_eq!(lexer.next(), Some(Ok(Token::Assign)));
        assert_eq!(lexer.next(), Some(Ok(Token::Boolean(true))));
    }

    #[test]
    fn expr() {
        let mut lexer = Token::lexer("print(\"Hello, world!\")");

        assert_eq!(lexer.next(), Some(Ok(Token::Identifer)));
        assert_eq!(lexer.next(), Some(Ok(Token::LParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::String("Hello, world!".to_string()))));
        assert_eq!(lexer.next(), Some(Ok(Token::RParen)));
    }

    #[test]
    fn int_overflow() {
        let mut lexer = Token::lexer("let x = 9223372036854775808");

        assert_eq!(lexer.next(), Some(Ok(Token::Let)));
        assert_eq!(lexer.next(), Some(Ok(Token::Identifer)));
        assert_eq!(lexer.next(), Some(Ok(Token::Assign)));
        assert_eq!(lexer.next(), Some(Err(LexingError::InvalidInteger("Integer overflow".to_string()))));
    }
}
