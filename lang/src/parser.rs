use std::collections::HashMap;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use colored::Colorize;
use crate::{ast::{Expr, SpannedExpr, AssignOp}, eval, lexer::{Span, Token, lex}};

enum Postfix {
    Property(String, Span),
    Call(Vec<SpannedExpr>),
    Index(SpannedExpr),
}

fn report_error(report: Report<'_, (&str, std::ops::Range<usize>)>, source_name: &str, code: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        let mut writer = crate::WasmWriter;
        let _ = report.write((source_name, Source::from(code)), &mut writer);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = report.eprint((source_name, Source::from(code)));
    }
}

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
            (Token::Continue, span) => SpannedExpr { node: Expr::Continue, span },
        };

        let array = select! { (Token::LBracket, span) => span }
            .then(
                expr.clone()
                    .separated_by(select! { (Token::Comma, _) => () })
                    .allow_trailing()
                    .collect::<Vec<_>>()
            )
            .then(select! { (Token::RBracket, span) => span })
            .map(|((start, elements), end): ((Span, Vec<SpannedExpr>), Span)| SpannedExpr {
                node: Expr::Array(elements),
                span: Span::from(start.start..end.end),
            });

        let primary = choice((
            atom,
            array,
            select! { (Token::LParen, _) => () }
                .ignore_then(expr.clone())
                .then_ignore(select! { (Token::RParen, _) => () })
        ));

        let postfix = primary
            .foldl(
                choice((
                    select! { (Token::Dot, _) => () }
                        .then(select! { (Token::Identifier(name), span) => (name, span) })
                        .map(|(_, (name, span))| Postfix::Property(name, span)),
                    
                    select! { (Token::LParen, _) => () }
                        .ignore_then(
                            expr.clone()
                                .separated_by(select! { (Token::Comma, _) => () })
                                .allow_trailing()
                                .collect::<Vec<_>>()
                        )
                        .then_ignore(select! { (Token::RParen, span) => span })
                        .map(Postfix::Call),

                    select! { (Token::LBracket, _) => () }
                        .ignore_then(expr.clone())
                        .then_ignore(select! { (Token::RBracket, span) => span })
                        .map(Postfix::Index),
                )).repeated(),
                |obj, postfix| match postfix {
                    Postfix::Property(name, span) => SpannedExpr {
                        node: Expr::PropertyAccess {
                            object: Box::new(obj.clone()),
                            property: name,
                        },
                        span: Span::from(obj.span.start..span.end),
                    },

                    Postfix::Call(args) => SpannedExpr {
                        span: obj.span.clone(),
                        node: Expr::Call {
                            callee: Box::new(obj.clone()),
                            args,
                        },
                    },

                    Postfix::Index(index) => SpannedExpr {
                        span: Span::from(obj.span.start..(index.span.end + 1)), // + 1 to get the ] too
                        node: Expr::IndexAccess {
                            object: Box::new(obj.clone()),
                            index: Box::new(index),
                        },
                    },
                },
            );

        let unary = select! { (Token::Minus, span) => span }
            .repeated()
            .collect::<Vec<Span>>()
            .then(postfix)
            .map(|(neg, mut expr): (Vec<Span>, SpannedExpr)| {
                for neg_span in neg.into_iter().rev() {
                    expr = SpannedExpr {
                        node: Expr::Neg(Box::new(expr.clone())),
                        span: Span::from(neg_span.start..expr.span.end),
                    };
                }

                expr
            });
        
        let power = unary.clone()
           .separated_by(select! { (Token::Pow, _span) => () })
            .at_least(1)
            .collect::<Vec<SpannedExpr>>()
            .map(|mut exprs: Vec<SpannedExpr>| {
                let mut result = exprs.pop().unwrap();
                while let Some(left) = exprs.pop() {
                    result = SpannedExpr {
                        node: Expr::Pow(Box::new(left.clone()), Box::new(result.clone())),
                        span: Span::from(left.span.start..result.span.end),
                    };
                }
                result
            })
            .boxed();
        
        let multiplicative = power.clone()
            .foldl(
                choice((
                    select! { (Token::Star, span) => span }.then(power.clone()).map(|(span, right)| (Token::Star, span, right)),
                    select! { (Token::Slash, span) => span }.then(power.clone()).map(|(span, right)| (Token::Slash, span, right)),
                    select! { (Token::Mod, span) => span }.then(power.clone()).map(|(span, right)| (Token::Mod, span, right)),
                ))
                .repeated(),
 
                |left: SpannedExpr, (op, _span, right): (Token, Span, SpannedExpr)| SpannedExpr {
                    node: match op {
                        Token::Star => Expr::Mul(Box::new(left.clone()), Box::new(right.clone())),
                        Token::Slash => Expr::Div(Box::new(left.clone()), Box::new(right.clone())),
                        Token::Mod => Expr::Mod(Box::new(left.clone()), Box::new(right.clone())),
                        _ => unreachable!(),
                    },
                    span: Span::from(left.span.start..right.span.end),
                }
            )
            .boxed();

        let additive = multiplicative.clone()
            .foldl(
                choice((
                    select! { (Token::Plus, span) => span }.then(multiplicative.clone()).map(|(span, right)| (Token::Plus, span, right)),
                    select! { (Token::Minus, span) => span }.then(multiplicative.clone()).map(|(span, right)| (Token::Minus, span, right)),
                ))
                .repeated(),

                |left: SpannedExpr, (op, _span, right): (Token, Span, SpannedExpr)| SpannedExpr {
                    node: match op {
                        Token::Plus => Expr::Add(Box::new(left.clone()), Box::new(right.clone())),
                        Token::Minus => Expr::Sub(Box::new(left.clone()), Box::new(right.clone())),
                        _ => unreachable!(),
                    },
                    span: Span::from(left.span.start..right.span.end),
                }
            )
            .boxed();
        
        let range = additive.clone()
            .then(
                select! { (Token::Range, span) => span }
                    .then(additive.clone())
                    .or_not()
            )
            .map(|(start, range): (SpannedExpr, Option<(Span, SpannedExpr)>)| {
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
            })
            .boxed();
        
        let inclusive_range = range.clone()
            .then(
                select! { (Token::InclusiveRange, span) => span }
                    .then(range.clone())
                    .or_not()
            )
            .map(|(start, range): (SpannedExpr, Option<(Span, SpannedExpr)>)| {
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
            })
            .boxed();
        
        let comparison = inclusive_range.clone()
            .foldl(
                choice((
                    select! { (Token::Equal, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::Equal, span, right)),
                    select! { (Token::NotEqual, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::NotEqual, span, right)),
                    select! { (Token::LessThan, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::LessThan, span, right)),
                    select! { (Token::LessThanOrEqual, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::LessThanOrEqual, span, right)),
                    select! { (Token::GreaterThan, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::GreaterThan, span, right)),
                    select! { (Token::GreaterThanOrEqual, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::GreaterThanOrEqual, span, right)),
                    select! { (Token::In, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::In, span, right)),
                    select! { (Token::NotIn, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::NotIn, span, right)),
                )).repeated(),

                |left: SpannedExpr, (op, _span, right): (Token, Span, SpannedExpr)| SpannedExpr {
                    node: match op {
                        Token::Equal => Expr::Equal(Box::new(left.clone()), Box::new(right.clone())),
                        Token::NotEqual => Expr::NotEqual(Box::new(left.clone()), Box::new(right.clone())),
                        Token::LessThan => Expr::LessThan(Box::new(left.clone()), Box::new(right.clone())),
                        Token::LessThanOrEqual => Expr::LessThanOrEqual(Box::new(left.clone()), Box::new(right.clone())),
                        Token::GreaterThan => Expr::GreaterThan(Box::new(left.clone()), Box::new(right.clone())),
                        Token::GreaterThanOrEqual => Expr::GreaterThanOrEqual(Box::new(left.clone()), Box::new(right.clone())),
                        Token::In => Expr::In(Box::new(left.clone()), Box::new(right.clone())),
                        Token::NotIn => Expr::NotIn(Box::new(left.clone()), Box::new(right.clone())),
                        _ => unreachable!(),
                    },
                    span: Span::from(left.span.start..right.span.end),
                }
            );
        
        comparison
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
        
        let assign_stmt = select! { (Token::Identifier(name), span) => (name, span) }
            .then(
                choice((
                    select! { (Token::Assign, _) => None },
                    select! { (Token::AddAssign, _) => Some(AssignOp::Add) },
                    select! { (Token::SubAssign, _) => Some(AssignOp::Sub) },
                    select! { (Token::MulAssign, _) => Some(AssignOp::Mul) },
                    select! { (Token::DivAssign, _) => Some(AssignOp::Div) },
                    select! { (Token::ModAssign, _) => Some(AssignOp::Mod) },
                ))
            )
            .then(expr.clone())
            .then(select! { (Token::Semicolon, span) => span }.labelled("semicolon"))
            .map(|((((name, name_span), op), value), end): ((((String, Span), Option<AssignOp>), SpannedExpr), Span)| {
                SpannedExpr {
                    node: Expr::Assign { name, value: Box::new(value), operator: op },
                    span: Span::from(name_span.start..end.end),
                }
            });

        let expr_stmt = expr.clone()
            .map_with(|expr, e| (expr, e.span()))
            .then(select! { (Token::Semicolon, span) => span }.labelled("semicolon"))
            .map(|((expr, _), end): ((SpannedExpr, SimpleSpan), Span)| {
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
            .then(
                select! { (Token::Assign, _) => true }
                    .or(select! { (Token::In, _) => false })
            )
            .then(expr.clone())
            .then(block.clone())
            .map(|((((start, iterator_name), is_deprecated), iterator_range), body): ((((Span, String), bool), SpannedExpr), SpannedExpr)| {
                if is_deprecated {
                    println!("{}", format!("Warning: using '=' in for loops is deprecated and will be removed in an future version. Use 'in' instead. ").dimmed());
                }   
                
                SpannedExpr {
                    node: Expr::ForLoop {
                        iterator_name,
                        iterator_range: Box::new(iterator_range.clone()),
                        body: Box::new(body.clone()),
                    },
                    span: Span::from(start.start..body.span.end),
                }
            });
        
        let while_loop_stmt = select! { (Token::While, span) => span }
            .then(expr.clone())
            .then(block.clone())
            .map(|((start, condition), body): ((Span, SpannedExpr), SpannedExpr)| SpannedExpr {
                node: Expr::WhileLoop {
                    condition: Box::new(condition.clone()),
                    body: Box::new(body.clone()),
                },
                span: Span::from(start.start..body.span.end),
            });
        
        let if_stmt = select! { (Token::If, span) => span }
            .then(expr.clone())
            .then(block.clone())
            .then(
                select! { (Token::ElseIf, span) => span }
                    .then(expr.clone())
                    .then(block.clone())
                    .repeated()
                    .collect::<Vec<_>>()
            )
            .then(
                select! { (Token::Else, span) => span }
                    .then(block.clone())
                    .or_not()
            )
            .map(|((((start, condition), then_branch), else_if_branches), else_branch): ((((Span, SpannedExpr), SpannedExpr), Vec<((Span, SpannedExpr), SpannedExpr)>), Option<(Span, SpannedExpr)>)| {
                let else_if_branches = else_if_branches.into_iter().map(|((_, cond), block)| (cond, block)).collect();
                let else_branch = else_branch.map(|(_, block)| Box::new(block.clone()));
                
                SpannedExpr {
                    node: Expr::If {
                        condition: Box::new(condition.clone()),
                        then_branch: Box::new(then_branch.clone()),
                        else_if_branches,
                        else_branch,
                    },
                    span: Span::from(start.start..then_branch.span.end),
                }
            });
        
        let return_stmt = select! { (Token::Return, span) => span }
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
        
        let import_stmt = select! { (Token::Import, span) => span }
            .then(expr.clone())
            .then(
                select! { (Token::As, span) => span }
                    .then(
                        select! { (Token::Identifier(name), _) => name }
                            .or(select! { (Token::Star, _) => "*".to_string() })
                    )
                    .or_not()
            )
            .then(select! { (Token::Semicolon, span) => span }.labelled("semicolon"))
            .map(|(((start, name_expr), import_as), end): (((Span, SpannedExpr), Option<(Span, String)>), Span)| {
                let import_name = match name_expr.node {
                    Expr::String(s) => s,
                    _ => "".to_string(),
                };

                SpannedExpr {
                    node: Expr::Import {
                        name: import_name,
                        import_as: import_as.map(|(_, n)| n),
                    },
                    span: Span::from(start.start..end.end),
                }
            });
                
        choice((
            let_stmt,
            assign_stmt,
            fn_stmt,
            infinite_loop_stmt,
            for_loop_stmt,
            while_loop_stmt,
            if_stmt,
            import_stmt,
            return_stmt,
            block,
            expr_stmt,
        )).boxed()
    });

    stmt.repeated().collect::<Vec<_>>().then_ignore(end())
}

pub fn parse(input: &str, filename: &str, context: &mut HashMap<String, Expr>) {
    let tokens = match lex(input) {
        Ok(toks) => toks,
        Err(e) => {
            let report = Report::build(ReportKind::Error, (filename, e.1.into_range()))
                .with_message(format!("Lexing error: {:?}", e.0))
                .with_label(
                    Label::new((filename, e.1.into_range()))
                        .with_color(Color::Red)
                        .with_message(format!("{}", e.0)),
                )
                .finish();
            
            report_error(report, filename, input);

            return;
        }
    };

    match parser().parse(&tokens).into_result() {
        Ok(ast) => {
            let sys_args = std::env::args().collect::<Vec<String>>();

            if sys_args.contains(&"--debug".to_string()) {
                println!("AST: {:#?}", ast);
            }

            if let Err(()) = crate::validator::validate_ast(&ast, filename, input) {
                return;
            }

            for expr in ast {
                match eval::eval(&expr, context) {
                    Ok(_) => {}

                    Err(e) => {
                        let report = Report::build(ReportKind::Error, (filename, e.span.into_range()))
                            .with_message(format!("Evaluation error: {}", e.message))
                            .with_label(
                                Label::new((filename, e.span.into_range()))
                                    .with_color(Color::Red)
                                    .with_message(format!("{}", e.message_short)),
                            )
                            .finish();
                        
                        report_error(report, filename, input);

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

                        let report = Report::build(ReportKind::Error, (filename, error_span.into_range()))
                            .with_message(format!("I expected {:?}, but found {}", expected, found_str))
                            .with_label(
                                    Label::new((filename, error_span.into_range()))
                                        .with_color(Color::Red)
                                        .with_message(format!("expected {:?}", expected)),
                            )
                            .finish();
                        
                        report_error(report, filename, input);
                    }
    
                    _ => {
                        let report = Report::build(ReportKind::Error, (filename, span.into_range()))
                            .with_message(format!("{:?}", err.reason()))
                            .with_label(
                                    Label::new((filename, span.into_range()))
                                        .with_color(Color::Red)
                                        .with_message("error occurred here"),
                            )
                            .finish();
                        
                        report_error(report, filename, input);
                    }
               } 
            }
        }
    }
}