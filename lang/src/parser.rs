use std::collections::HashMap;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use crate::{ast::{Expr, SpannedExpr, AssignOp}, lexer::{Span, Token, lex}};

enum Postfix {
    Property(String, Span),
    Call(Vec<SpannedExpr>, Span),
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
            (Token::Super, span) => SpannedExpr { node: Expr::Identifier("super".to_string()), span },

            (Token::Null, span) => SpannedExpr { node: Expr::Null, span },
            (Token::Break, span) => SpannedExpr { node: Expr::Break, span },
            (Token::Continue, span) => SpannedExpr { node: Expr::Continue, span },
        }.labelled("atom");

        let array = select! { (Token::LBracket, span) => span }
            .then(
                expr.clone().labelled("entries")
                    .separated_by(select! { (Token::Comma, _) => () })
                    .allow_trailing()
                    .collect::<Vec<_>>()
            )
            .then(select! { (Token::RBracket, span) => span })
            .map(|((start, elements), end): ((Span, Vec<SpannedExpr>), Span)| SpannedExpr {
                node: Expr::Array(elements),
                span: Span::from(start.start..end.end),
            })
            .labelled("array");
        
        let object = select! { (Token::LBrace, start) => start }
            .then(
                select! { (Token::String(key), _) => key }.labelled("key")
                .then_ignore(select! { (Token::Colon, _) => () })
                .then(expr.clone().labelled("value"))
                .separated_by(select! { (Token::Comma, _) => () })
                .allow_trailing()
                .collect::<Vec<_>>()
            )
            .then(select! { (Token::RBrace, end) => end })
            .map(|((start, entries), end): ((Span, Vec<(String, SpannedExpr)>), Span)| {
                let mut map = HashMap::new();

                for (key, value) in entries {
                    map.insert(key, value);
                }

                SpannedExpr {
                    node: Expr::Object { properties: map },
                    span: Span::from(start.start..end.end),
                }
            });

        let primary = choice((
            atom,
            array,
            object,
            select! { (Token::LParen, _) => () }
                .ignore_then(expr.clone())
                .then_ignore(select! { (Token::RParen, _) => () })
        )).labelled("primary expression");

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
                        .then(select! { (Token::RParen, span) => span })
                        .map(|(args, span)| Postfix::Call(args, span)),

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

                    Postfix::Call(args, span) => SpannedExpr {
                        span: Span::from(obj.span.start..span.end),
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
            )
            .labelled("postfix expression")
            .boxed();

        let unary = choice((
            select! { (Token::Minus, span) => (Token::Minus, span) },
            select! { (Token::Not, span) => (Token::Not, span) },
            select! { (Token::BitNot, span) => (Token::BitNot, span) }
        ))
            .repeated()
            .collect::<Vec<(Token, Span)>>()
            .then(postfix)
            .map(|(ops, mut expr): (Vec<(Token, Span)>, SpannedExpr)| {
                for (op, op_span) in ops.into_iter().rev() {
                    expr = SpannedExpr {
                        node: match op {
                            Token::Minus => Expr::Neg(Box::new(expr.clone())),
                            Token::Not => Expr::Not(Box::new(expr.clone())),
                            Token::BitNot => Expr::BitNot(Box::new(expr.clone())),
                            _ => unreachable!(),
                        },
                        span: Span::from(op_span.start..expr.span.end),
                    };
                }

                expr
            })
            .boxed()
            .labelled("unary expression");
        
        let shift = unary.clone()
            .foldl(
                choice((
                    select! { (Token::BitShl, span) => span }.then(unary.clone()).map(|(span, right)| (Token::BitShl, span, right)),
                    select! { (Token::BitShr, span) => span }.then(unary.clone()).map(|(span, right)| (Token::BitShr, span, right)),
                )).repeated(),
                |left: SpannedExpr, (op, _span, right): (Token, Span, SpannedExpr)| SpannedExpr {
                    node: match op {
                        Token::BitShl => Expr::BitShl(Box::new(left.clone()), Box::new(right.clone())),
                        Token::BitShr => Expr::BitShr(Box::new(left.clone()), Box::new(right.clone())),
                        _ => unreachable!(),
                    },
                    span: Span::from(left.span.start..right.span.end),
                }
            )
            .boxed()
            .labelled("shift expression");
        
        let and = shift.clone()
            .foldl(
                select! { (Token::BitAnd, span) => span }.then(shift.clone()).repeated(),
                |left: SpannedExpr, (_span, right): (Span, SpannedExpr)| SpannedExpr {
                    node: Expr::BitAnd(Box::new(left.clone()), Box::new(right.clone())),
                    span: Span::from(left.span.start..right.span.end),
                }
            )
            .boxed()
            .labelled("bitwise AND expression");
        
        let or = and.clone()
            .foldl(
                select! { (Token::BitOr, span) => span }.then(and.clone()).repeated(),
                |left: SpannedExpr, (_span, right): (Span, SpannedExpr)| SpannedExpr {
                    node: Expr::BitOr(Box::new(left.clone()), Box::new(right.clone())),
                    span: Span::from(left.span.start..right.span.end),
                }
            )
            .boxed()
            .labelled("bitwise OR expression");
        
        let xor = or.clone()
            .foldl(
                select! { (Token::BitXor, span) => span }.then(or.clone()).repeated(),
                |left: SpannedExpr, (_span, right): (Span, SpannedExpr)| SpannedExpr {
                    node: Expr::BitXor(Box::new(left.clone()), Box::new(right.clone())),
                    span: Span::from(left.span.start..right.span.end),
                }
            )
            .boxed()
            .labelled("bitwise XOR expression");

        let power = xor.clone()
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
            .boxed()
            .labelled("power expression");
        
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
            .boxed()
            .labelled("multiplicative expression");

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
            .boxed()
            .labelled("additive expression");
        
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
            .boxed()
            .labelled("range expression");
        
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
            .boxed()
            .labelled("inclusive range expression");
        
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
                    select! { (Token::And, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::And, span, right)),
                    select! { (Token::Or, span) => span }.then(inclusive_range.clone()).map(|(span, right)| (Token::Or, span, right)),
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
                        Token::And => Expr::And(Box::new(left.clone()), Box::new(right.clone())),
                        Token::Or => Expr::Or(Box::new(left.clone()), Box::new(right.clone())),
                        _ => unreachable!(),
                    },
                    span: Span::from(left.span.start..right.span.end),
                }
            )
            .boxed();
        
        comparison
    });

    let stmt = recursive(|stmt| {
        let let_stmt = select! { (Token::Let, span) => span }
            .then(select! { (Token::Identifier(name), _) => name }.labelled("variable name"))
            .then_ignore(select! { (Token::Assign, _) => () })
            .then(expr.clone().labelled("an expression after '='"))
            .then(select! { (Token::Semicolon, span) => span }.labelled("semicolon"))
            .map(|(((start, name), value), end): (((Span, String), SpannedExpr), Span)| SpannedExpr {
                node: Expr::Let { name, value: Box::new(value) },
                span: Span::from(start.start..end.end),
            })
            .labelled("let statement");
        
        let const_stmt = select! { (Token::Const, span) => span }
            .then(select! { (Token::Identifier(name), _) => name }.labelled("constant name"))
            .then_ignore(select! { (Token::Assign, _) => () })
            .then(expr.clone().labelled("an expression after '='"))
            .then(select! { (Token::Semicolon, span) => span }.labelled("semicolon"))
            .map(|(((start, name), value), end): (((Span, String), SpannedExpr), Span)| SpannedExpr {
                node: Expr::Const { name, value: Box::new(value) },
                span: Span::from(start.start..end.end),
            })
            .labelled("const statement");
        
        let assign_stmt = expr.clone().labelled("target")
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
            .then(expr.clone().labelled("an expression after '='"))
            .then(select! { (Token::Semicolon, span) => span }.labelled("semicolon"))
            .map(|(((target, op), value), end): (((SpannedExpr, Option<AssignOp>), SpannedExpr), Span)| {
                let start = target.span.start;

                SpannedExpr {
                    node: Expr::Assign { target: Box::new(target), value: Box::new(value), operator: op },
                    span: Span::from(start..end.end),
                }
            })
            .labelled("assignment");

        let expr_stmt = expr.clone()
            .map_with(|expr, e| (expr, e.span()))
            .then(select! { (Token::Semicolon, span) => span }.labelled("semicolon"))
            .map(|((expr, _), end): ((SpannedExpr, SimpleSpan), Span)| {
                SpannedExpr {
                    node: expr.node,
                    span: Span::from(expr.span.start..end.end),
                }
            })
            .labelled("expression");
        
        let block = select! { (Token::LBrace, span) => span }
            .then(stmt.clone().repeated().collect::<Vec<_>>())
            .then(select! { (Token::RBrace, span) => span })
            .map(|((start, stmts), end): ((Span, Vec<SpannedExpr>), Span)| SpannedExpr {
                node: Expr::Block(stmts),
                span: Span::from(start.start..end.end),
            })
            .labelled("block");
        
        let fn_stmt = select! { (Token::Function, span) => span }
            .then(select! { (Token::Identifier(name), _) => name }.labelled("function name"))
            .then_ignore(select! { (Token::LParen, _) => () })
            .then(
                select! { (Token::Identifier(name), _) => name }
                    .separated_by(select! { (Token::Comma, _) => () })
                    .allow_trailing()
                    .collect::<Vec<_>>()
            )
            .then_ignore(select! { (Token::RParen, _) => () })
            .then(block.clone().labelled("function body"))
            .map(|(((start, name), args), body): (((Span, String), Vec<String>), SpannedExpr)| SpannedExpr {
                node: Expr::Function { name, args, body: Box::new(body.clone()) },
                span: Span::from(start.start..body.span.end),
            })
            .labelled("function declaration");
        
        let class_stmt = select! { (Token::Class, span) => span }
            .then(select! { (Token::Identifier(name), _) => name }.labelled("class name"))
            .then(
                select! { (Token::Extends, _) }
                .ignore_then(
                    select! { (Token::Identifier(name), _) => name }
                )
                .or_not()
            )
            .then(
                select! { (Token::LBrace, span) => span }
                    .then(fn_stmt.clone().repeated().collect::<Vec<_>>())
                    .then(select! { (Token::RBrace, span) => span })
            )
            .map(|(((start, name), parent), ((_lbrace, methods), end)): (((Span, String), Option<String>), ((Span, Vec<SpannedExpr>), Span))| SpannedExpr {
                node: Expr::Class { name, methods, parent },
                span: Span::from(start.start..end.end),
            })
            .labelled("class declaration");
        
        let infinite_loop_stmt = select! { (Token::Loop, span) => span }
            .then(block.clone().labelled("loop body"))
            .map(|(start, body): (Span, SpannedExpr)| SpannedExpr {
                node: Expr::InfiniteLoop { body: Box::new(body.clone()) },
                span: Span::from(start.start..body.span.end),
            })
            .labelled("infinite loop");
        
        let for_loop_stmt = select! { (Token::For, span) => span }
            .then(select! { (Token::Identifier(name), _) => name })
            .then_ignore(select! { (Token::In, _) => () })
            .then(expr.clone().labelled("iterable"))
            .then(block.clone().labelled("loop body"))
            .map(|(((start, iterator_name), iterator_range), body): (((Span, String), SpannedExpr), SpannedExpr)| {
                SpannedExpr {
                    node: Expr::ForLoop {
                        iterator_name,
                        iterator_range: Box::new(iterator_range.clone()),
                        body: Box::new(body.clone()),
                    },
                    span: Span::from(start.start..body.span.end),
                }
            })
            .labelled("for loop");
        
        let while_loop_stmt = select! { (Token::While, span) => span }
            .then(expr.clone().labelled("condition"))
            .then(block.clone().labelled("loop body"))
            .map(|((start, condition), body): ((Span, SpannedExpr), SpannedExpr)| SpannedExpr {
                node: Expr::WhileLoop {
                    condition: Box::new(condition.clone()),
                    body: Box::new(body.clone()),
                },
                span: Span::from(start.start..body.span.end),
            })
            .labelled("while loop");
        
        let if_stmt = select! { (Token::If, span) => span }
            .then(expr.clone().labelled("'if' condition"))
            .then(block.clone().labelled("body"))
            .then(
                select! { (Token::ElseIf, span) => span }
                    .then(expr.clone().labelled("else if condition"))
                    .then(block.clone().labelled("'else if' body"))
                    .repeated()
                    .collect::<Vec<_>>()
            )
            .then(
                select! { (Token::Else, span) => span }
                    .then(block.clone().labelled("'else' body"))
                    .or_not()
            )
            .map(|((((start, condition), then_branch), else_if_branches), else_branch): ((((Span, SpannedExpr), SpannedExpr), Vec<((Span, SpannedExpr), SpannedExpr)>), Option<(Span, SpannedExpr)>)| {
                let mut branches = Vec::new();

                branches.push((Some(condition.clone()), then_branch.clone()));
                branches.extend(else_if_branches.into_iter().map(|((_, cond), block)| (Some(cond), block)));
                
                if let Some((_, else_block)) = else_branch.clone() {
                    branches.push((None, else_block));
                }
                
                SpannedExpr {
                    node: Expr::If(branches),
                    span: Span::from(start.start..then_branch.span.end),
                }
            })
            .labelled("if statement");
        
        let return_stmt = select! { (Token::Return, span) => span }
            .then(expr.clone().or_not().labelled("an expression after 'return'"))
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
            })
            .labelled("return statement");
        
        let import_stmt = select! { (Token::Import, span) => span }
            .then(expr.clone().labelled("module name"))
            .then(
                select! { (Token::As, span) => span }
                    .then(
                        select! { (Token::Identifier(name), _) => name }
                            .or(select! { (Token::Star, _) => "*".to_string() })
                    )
                    .or_not()
                    .labelled("optional 'as' clause")
            )
            .then(select! { (Token::Semicolon, span) => span }.labelled("semicolon"))
            .map(|(((start, name_expr), alias), end): (((Span, SpannedExpr), Option<(Span, String)>), Span)| {
                let import_name = match name_expr.node {
                    Expr::String(s) => s,
                    _ => "".to_string(),
                };

                SpannedExpr {
                    node: Expr::Import {
                        name: import_name,
                        alias: alias.map(|(_, n)| n),
                    },
                    span: Span::from(start.start..end.end),
                }
            })
            .labelled("import statement");
        
        let try_catch_stmt = select! { (Token::Try, span) => span }
            .then(block.clone().labelled("try block"))
            .then(
                select! { (Token::Catch, span) => span }
                    .then(
                        select! { (Token::Identifier(name), _) => name }.or_not()
                    )
                    .then(block.clone().labelled("catch block"))
                    .or_not()
            )
            .map(|((start, try_block), catch): ((Span, SpannedExpr), Option<((Span, Option<String>), SpannedExpr)>)| {
                let (catch_var, catch_block) = match catch {
                    Some(((_, var), block)) => (var, block),
                    None => (None, SpannedExpr {
                        node: Expr::Block(vec![]),
                        span: start.clone(),
                    }),
                };

                SpannedExpr {
                    node: Expr::Try {
                        try_block: Box::new(try_block.clone()),
                        catch_block: Box::new(catch_block.clone()),
                        catch_var,
                    },
                    span: Span::from(start.start..try_block.span.end),
                }
            })
            .labelled("try-catch statement");
                
        choice((
            let_stmt,
            const_stmt,
            assign_stmt,
            fn_stmt,
            class_stmt,
            infinite_loop_stmt,
            for_loop_stmt,
            while_loop_stmt,
            if_stmt,
            import_stmt,
            return_stmt,
            block,
            expr_stmt,
            try_catch_stmt,
        )).boxed().labelled("statement")
    });

    stmt.repeated().collect::<Vec<_>>().then_ignore(end()).labelled("program")
}

pub fn parse(input: &str, filename: &str) -> Result<Vec<SpannedExpr>, ()> {
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

            return Err(());
        }
    };

    match parser().parse(&tokens).into_result() {
        Ok(ast) => {
            if let Err(()) = crate::validator::validate_ast(&ast, filename, input) {
                return Err(());
            } else {
                return Ok(ast);
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

            return Err(());
        }
    }
}