use std::collections::HashMap;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use crate::{ast::{Expr, SpannedExpr}, evaulator, lexer::{Span, Token, lex}};

fn parser<'src>() -> impl Parser<
    'src, 
    &'src [(Token, Span)],
    Vec<SpannedExpr>,
    extra::Err<Rich<'src, (Token, Span), Span>>
> {
    let expr = recursive(|expr| {
        let atom = select! {
            (Token::Int(n), span) => SpannedExpr { node: Expr::Int(n), span },
            (Token::Float(f), span) => SpannedExpr { node: Expr::Float(f), span },
            (Token::String(name), span) => SpannedExpr { node: Expr::String(name), span },
            (Token::Identifier(name), span) => SpannedExpr { node: Expr::Identifier(name), span },
            (Token::Bool(b), span) => SpannedExpr { node: Expr::Bool(b), span },
            (Token::Null, span) => SpannedExpr { node: Expr::Null, span },
            (Token::Break, span) => SpannedExpr { node: Expr::Break, span },
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

        let primary = call.or(atom);

        let unary = select! { (Token::Minus, span) => span }
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

        let additive = unary.clone()
            .foldl(
                choice((
                    select! { (Token::Plus, span) => span }.then(unary.clone()).map(|(span, right)| (Token::Plus, span, right)),
                    select! { (Token::Minus, span) => span }.then(unary.clone()).map(|(span, right)| (Token::Minus, span, right)),
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
            );
        
        let range = additive.clone()
            .then(
                select! { (Token::Range, span) => span }
                    .then(additive.clone())
                    .or_not()
            )
            .map(|(start, range)| {
                match range {
                    Some((_, end)) => SpannedExpr {
                        node: Expr::Range {
                            start: Box::new(start.clone()),
                            end: Box::new(end.clone()),
                        },
                        span: Span::from(start.span.start..end.span.end),
                    },

                    None => start,
                }
            });
        
        let inclusive_range = range.clone()
            .then(
                select! { (Token::InclusiveRange, span) => span }
                    .then(range.clone())
                    .or_not()
            )
            .map(|(start, range)| {
                match range {
                    Some((_, end)) => SpannedExpr {
                        node: Expr::InclusiveRange {
                            start: Box::new(start.clone()),
                            end: Box::new(end.clone()),
                        },
                        span: Span::from(start.span.start..end.span.end),
                    },
                    None => start,
                }
            });
        
        inclusive_range.clone()
            .foldl(
                choice((
                    select! { (Token::DoubleEqual, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::DoubleEqual, span, right)),
                    select! { (Token::NotEqual, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::NotEqual, span, right)),
                    select! { (Token::LessThan, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::LessThan, span, right)),
                    select! { (Token::LessThanOrEqual, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::LessThanOrEqual, span, right)),
                    select! { (Token::GreaterThan, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::GreaterThan, span, right)),
                    select! { (Token::GreaterThanOrEqual, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::GreaterThanOrEqual, span, right)),
                )).repeated(),

                |left, (op, _, right)| SpannedExpr {
                    node: match op {
                        Token::DoubleEqual => Expr::Equal(Box::new(left.clone()), Box::new(right.clone())),
                        Token::NotEqual => Expr::NotEqual(Box::new(left.clone()), Box::new(right.clone())),
                        Token::LessThan => Expr::LessThan(Box::new(left.clone()), Box::new(right.clone())),
                        Token::LessThanOrEqual => Expr::LessThanOrEqual(Box::new(left.clone()), Box::new(right.clone())),
                        Token::GreaterThan => Expr::GreaterThan(Box::new(left.clone()), Box::new(right.clone())),
                        Token::GreaterThanOrEqual => Expr::GreaterThanOrEqual(Box::new(left.clone()), Box::new(right.clone())),
                        _ => unreachable!(),
                    },
                    span: Span::from(left.span.start..right.span.end),
                }
            )
    });

    let stmt = recursive(|stmt| {
        let let_stmt = select! { (Token::Let, span) => span }
            .then(select! { (Token::Identifier(name), _) => name })
            .then_ignore(select! { (Token::Assign, _) => () })
            .then(expr.clone())
            .then(select! { (Token::Semicolon, span) => span }.labelled("semicolon"))
            .map(|(((start, name), value), end): (((Span, String), SpannedExpr), Span)| SpannedExpr {
                node: Expr::Let { name, value: Box::new(value) },
                span: Span::from(start.start..end.end),
            });

        let expr_stmt = expr.clone()
            .map_with(|expr, e| (expr, e.span()))
            .then(select! { (Token::Semicolon, span) => span }.labelled("semicolon"))
            .map(|((expr, expr_span), end): ((SpannedExpr, SimpleSpan), Span)| {
                SpannedExpr {
                    node: expr.node,
                    span: Span::from(expr.span.start..end.end),
                }
            })
            .labelled("statement");
        
        let block = select! { (Token::LBrace, span) => span }
            .then(stmt.clone().repeated().collect::<Vec<_>>())
            .then(select! { (Token::RBrace, span) => span })
            .map(|((start, stmts), end): ((Span, Vec<SpannedExpr>), Span)| SpannedExpr {
                node: Expr::Block(stmts),
                span: Span::from(start.start..end.end),
            });
        
        let fn_stmt = select! { (Token::Function, span) => span }
            .then(select! { (Token::Identifier(name), _) => name })
            .then_ignore(select! { (Token::LParen, _) => () })
            .then(
                select! { (Token::Identifier(name), _) => name }
                    .separated_by(select! { (Token::Comma, _) => () })
                    .allow_trailing()
                    .collect::<Vec<_>>()
            )
            .then_ignore(select! { (Token::RParen, _) => () })
            .then(block.clone())
            .map(|(((start, name), args), body): (((Span, String), Vec<String>), SpannedExpr)| SpannedExpr {
                node: Expr::Function { name, args, body: Box::new(body.clone()) },
                span: Span::from(start.start..body.span.end),
            });
        
        let infinite_loop_stmt = select! { (Token::Loop, span) => span }
            .then(block.clone())
            .map(|(start, body): (Span, SpannedExpr)| SpannedExpr {
                node: Expr::InfiniteLoop { body: Box::new(body.clone()) },
                span: Span::from(start.start..body.span.end),
            });
        
        let for_loop_stmt = select! { (Token::For, span) => span }
            .then(select! { (Token::Identifier(name), _) => name })
            .then_ignore(select! { (Token::Assign, _) => () })
            .then(expr.clone())
            .then(block.clone())
            .map(|(((start, iterator_name), iterator_range), body): (((Span, String), SpannedExpr), SpannedExpr)| SpannedExpr {
                node: Expr::ForLoop {
                    iterator_name,
                    iterator_range: Box::new(iterator_range.clone()),
                    body: Box::new(body.clone()),
                },
                span: Span::from(start.start..body.span.end),
            });
        
        let retun_stmt = select! { (Token::Return, span) => span }
            .then(expr.clone().or_not())
            .then(select! { (Token::Semicolon, span) => span }.labelled("semicolon"))
            .map(|((start, value), end): ((Span, Option<SpannedExpr>), Span)| SpannedExpr {
                node: Expr::Return(
                    match value {
                        Some(v) => Box::new(v),
                        None => Box::new(SpannedExpr {
                            node: Expr::Null,
                            span: start.clone(),
                        }),
                    }
                ),
                span: Span::from(start.start..end.end),
            });
                
        let_stmt
            .or(fn_stmt)
            .or(infinite_loop_stmt)
            .or(for_loop_stmt)
            .or(retun_stmt)
            .or(block)
            .or(expr_stmt)
    });

    stmt.repeated().collect::<Vec<_>>().then_ignore(end())
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
                match &expr.node {
                    // should never be an return in the top-level
                    Expr::Return(_) => {
                        Report::build(ReportKind::Error, (filename, expr.span.into_range()))
                            .with_code(3)
                            .with_message("Return statement not allowed in top-level")
                            .with_label(
                                Label::new((filename, expr.span.into_range()))
                                    .with_color(Color::Red)
                                    .with_message("unexpected return statement"),
                            )
                            .with_help("Return statements can only be used inside functions")
                            .finish()
                            .print((filename, Source::from(input)))
                            .unwrap();

                        return;
                    }

                    Expr::Break => {
                        Report::build(ReportKind::Error, (filename, expr.span.into_range()))
                            .with_code(4)
                            .with_message("Break statement not allowed in top-level")
                            .with_label(
                                Label::new((filename, expr.span.into_range()))
                                    .with_color(Color::Red)
                                    .with_message("unexpected break statement"),
                            )
                            .with_help("Break statements can only be used inside loops")
                            .finish()
                            .print((filename, Source::from(input)))
                            .unwrap();

                        return;
                    }

                    _ => {}
                }

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
               let span = err.span();

               match err.reason() {
                    chumsky::error::RichReason::ExpectedFound { expected, found } => {
                        let (found_str, error_span) = match found {
                            Some(chumsky::util::MaybeRef::Val((tok, tok_span))) => {
                                (format!("{:?}", tok), tok_span.clone())
                            },
                            Some(chumsky::util::MaybeRef::Ref((tok, tok_span))) => {
                                (format!("{:?}", tok), tok_span.clone())
                            },
                            None => {
                                ("end of input".to_string(), Span::from(input.len()-1..input.len()-1))
                            }
                        };

                        Report::build(ReportKind::Error, (filename, error_span.into_range()))
                            .with_code(2)
                            .with_message(format!("I expected {:?}, but found {}", expected, found_str))
                            .with_label(
                                    Label::new((filename, error_span.into_range()))
                                        .with_color(Color::Red)
                                        .with_message(format!("expected {:?}", expected)),
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