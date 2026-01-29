use std::collections::HashMap;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use crate::{ast::{Expr, SpannedExpr}, evaulator, lexer::{Span, Token, lex}};

fn parser<'src>() -> impl Parser<
    'src, 
    &'src [(Token, Span)],
    Vec<SpannedExpr>,
    extra::Err<Rich<'src, (Token, Span), Span>>> 
{
    let expr = recursive(|expr| {
        let atom = select! {
            (Token::Int(n), span) => SpannedExpr { node: Expr::Int(n), span },
            (Token::Float(f), span) => SpannedExpr { node: Expr::Float(f), span },
            (Token::String(name), span) => SpannedExpr { node: Expr::String(name), span },
            (Token::Identifier(name), span) => SpannedExpr { node: Expr::Identifier(name), span },
            (Token::Bool(b), span) => SpannedExpr { node: Expr::Bool(b), span },
            (Token::Null, span) => SpannedExpr { node: Expr::Null, span },
        };

        let call = select! { (Token::Identifier(name), span) => (name, span) }
            .then_ignore(select! { (Token::LParen, _) => () })
            .then(
                expr.clone()
                    .separated_by(select! { (Token::Comma, _) => () })
                    .allow_trailing()
                    .collect::<Vec<_>>()
            )
            .then(select! { (Token::RParen, span) => span })
            .map(|(((name, start), args), end): (((String, Span), Vec<SpannedExpr>), Span)| SpannedExpr {
                node: Expr::Call { name, args },
                span: Span::from(start.start..end.end),
            });
        
        let block = select! { (Token::LBracket, span) => span }
            .then(expr.clone().repeated().collect::<Vec<SpannedExpr>>())
            .then(select! { (Token::RBracket, span) => span })
            .map(|((start, exprs), end): ((Span, Vec<SpannedExpr>), Span)| SpannedExpr {
                node: Expr::Block(exprs),
                span: Span::from(start.start..end.end),
            });

        let primary = call
            .or(atom)
            .or(block);

        let primary = select! { (Token::Minus, span) => span }
            .repeated()
            .collect::<Vec<Span>>()
            .then(primary)
            .map(|(neg, mut expr)| {
                for neg_span in neg.into_iter().rev() {
                    expr = SpannedExpr {
                        node: Expr::Neg(Box::new(expr.clone())),
                        span: Span::from(neg_span.start..expr.span.end),
                    };
                }

                expr
            });

        primary.clone()
            .foldl(
                choice((
                    select! { (Token::Plus, span) => span }.then(primary.clone()).map(|(span, right)| (Token::Plus, span, right)),
                    select! { (Token::Minus, span) => span }.then(primary.clone()).map(|(span, right)| (Token::Minus, span, right)),
                ))
                .repeated(),

                |left, (op, span, right)| SpannedExpr {
                    node: match op {
                        Token::Plus => Expr::Add(Box::new(left.clone()), Box::new(right.clone())),
                        Token::Minus => Expr::Sub(Box::new(left.clone()), Box::new(right.clone())),
                        _ => unreachable!(),
                    },
                    span: Span::from(left.span.start..right.span.end),
                }
            )
    });

    let let_stmt = select! { (Token::Let, span) => span }
        .then(select! { (Token::Identifier(name), _) => name })
        .then_ignore(select! { (Token::Equals, _) => () })
        .then(expr.clone())
        .then(select! { (Token::Semicolon, span) => span })
        .map(|(((start, name), value), end): (((Span, String), SpannedExpr), Span)| SpannedExpr {
            node: Expr::Let { name, value: Box::new(value) },
            span: Span::from(start.start..end.end),
        });

    let expr_stmt = expr.clone()
        .then(select! { (Token::Semicolon, span) => span })
        .map(|(mut expr, end)| {
            expr.span = Span::from(expr.span);
            expr
        });

    let_stmt.or(expr_stmt).repeated().collect().then_ignore(end())
}

pub fn parse(input: &str, filename: &str, context: &mut HashMap<String, Expr>) {
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
            let sys_args = std::env::args().collect::<Vec<String>>();

            if sys_args.contains(&"--debug".to_string()) {
                println!("AST: {:#?}", ast);
            }

            for expr in ast {
                match evaulator::eval(&expr, context) {
                    Ok(_) => {
                        
                    }

                    Err(e) => {
                        Report::build(ReportKind::Error, (filename, e.span.into_range()))
                            .with_code(1)
                            .with_message(format!("Evaluation error: {}", e.message))
                            .with_label(
                                Label::new((filename, e.span.into_range()))
                                    .with_color(Color::Red)
                                    .with_message(format!("{}", e.message_short)),
                            )
                            .finish()
                            .print((filename, Source::from(input)))
                            .unwrap();

                        return;
                    }
                }
            }
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
                            .with_code(2)
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