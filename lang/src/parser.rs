use std::collections::HashMap;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use crate::{ast::Expr, lexer::{Span, Token, lex}};

fn parser<'src>() -> impl Parser<
    'src, 
    &'src [(Token, Span)],
    Vec<Expr>,
    extra::Err<Rich<'src, (Token, Span), Span>>> 
{
    let expr = recursive(|expr| {
        let atom = select! {
            (Token::Int(n), _) => Expr::Int(n),
            (Token::Float(f), _) => Expr::Float(f),
            (Token::String(name), _) => Expr::String(name),
            (Token::Identifier(name), _) => Expr::Identifier(name),
        };

        let call = select! { (Token::Identifier(name), _) => name }
            .then_ignore(select! { (Token::LParen, _) => () })
            .then(
                expr.clone()
                    .separated_by(select! { (Token::Comma, _) => () })
                    .allow_trailing()
                    .collect::<Vec<_>>()
            )
            .then_ignore(select! { (Token::RParen, _) => () })
            .map(|(name, args)| Expr::Call { name, args });

        call.or(atom)
    });

    let let_stmt = select! { (Token::Let, _) => () }
        .ignore_then(select! { (Token::Identifier(name), _) => name })
        .then_ignore(select! { (Token::Equals, _) => () })
        .then(expr.clone())
        .then_ignore(select! { (Token::Semicolon, _) => () })
        .map(|(name, value)| Expr::Let { name, value: Box::new(value) });

    let expr_stmt = expr.clone().then_ignore(select! { (Token::Semicolon, _) => () });

    let_stmt.or(expr_stmt).repeated().collect().then_ignore(end())
}

pub fn parse(input: &str, filename: &str, _context: &mut HashMap<String, Expr>) {
    let tokens = match lex(input) {
        Ok(toks) => toks,
        Err(e) => {
            Report::build(ReportKind::Error, (filename, e.1.into_range()))
                .with_code(0)
                .with_message(format!("Lexing error: {:?}", e.0))
                .with_label(
                    Label::new((filename, e.1.into_range()))
                        .with_color(Color::Red)
                        .with_message(format!("{}", e.0)),
                )
                .finish()
                .print((filename, Source::from(input)))
                .unwrap();

            return;
        }
    };

    match parser().parse(&tokens).into_result() {
        Ok(ast) => {
            println!("Parsed AST:\n{:#?}", ast);
        }

        Err(e) => {
            for err in e {
               let mut span = err.span();

               match err.reason() {
                    chumsky::error::RichReason::ExpectedFound { expected, found } => {
                        if let Some(found) = found {
                            span = &found.1;
                        }

                        Report::build(ReportKind::Error, (filename, span.into_range()))
                            .with_code(1)
                            .with_message(format!("{:?}", err.reason()))
                            .with_label(
                                    Label::new((filename, span.into_range()))
                                        .with_color(Color::Red)
                                        .with_message(format!("expected {:?}, found {:?}", expected, found)),
                            )
                            .finish()
                            .print((filename, Source::from(input)))
                            .unwrap();
                    }
    
                    _ => {
                        Report::build(ReportKind::Error, (filename, span.into_range()))
                            .with_message(format!("{:?}", err.reason()))
                            .with_label(
                                    Label::new((filename, span.into_range()))
                                        .with_color(Color::Red)
                                        .with_message("error occurred here"),
                            )
                            .finish()
                            .print((filename, Source::from(input)))
                            .unwrap();
                    }
               } 
            }
        }
    }
}