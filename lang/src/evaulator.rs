use std::collections::HashMap;

use crate::ast::{Expr, SpannedExpr};
use crate::lexer::Span;

#[derive(Debug)]
pub struct EvalError {
    pub message: String,
    pub message_short: String,
    pub span: Span,
}

#[derive(Debug)]
pub enum Flow {
    Continue(Expr),
    Return(Expr),
    Break,
}

impl Flow {
    fn unwrap(self) -> Expr {
        match self {
            Flow::Continue(v) | Flow::Return(v) => v,
            Flow::Break => Expr::Null,
        }
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn eval<'src>(expr: &'src SpannedExpr, context: &mut HashMap<String, Expr>) -> Result<Flow, EvalError> {    
    match &expr.node {
        Expr::Int(n) => Ok(Flow::Continue(Expr::Int(*n))),
        Expr::Float(f) => Ok(Flow::Continue(Expr::Float(*f))),
        Expr::String(s) => Ok(Flow::Continue(Expr::String(s.clone()))),
        Expr::Bool(b) => Ok(Flow::Continue(Expr::Bool(*b))),
        Expr::Null => Ok(Flow::Continue(Expr::Null)),

        Expr::Neg(inner) => {
            let value = eval(inner, context)?.unwrap();

            match value {
                Expr::Int(n) => Ok(Flow::Continue(Expr::Int(-n))),
                Expr::Float(f) => Ok(Flow::Continue(Expr::Float(-f))),
                _ => Err(EvalError {
                    message: format!("Cannot negate value: {:?}", value),
                    message_short: "cannot negate".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Add(left, right) => {
            let left_value = eval(left, context)?.unwrap();
            let right_value = eval(right, context)?.unwrap();

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Int(l + r))),
                (Expr::Float(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Float(l + r))),
                (Expr::Int(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Float(l as f64 + r))),
                (Expr::Float(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Float(l + r as f64))),
                (Expr::String(l), Expr::String(r)) => Ok(Flow::Continue(Expr::String(l + &r))),

                _ => Err(EvalError {
                    message: format!("Cannot add values: {:?} + {:?}", left.node, right.node),
                    message_short: "cannot add".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Sub(left, right) => {
            let left_value = eval(left, context)?.unwrap();
            let right_value = eval(right, context)?.unwrap();

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Int(l - r))),
                (Expr::Float(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Float(l - r))),
                (Expr::Int(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Float(l as f64 - r))),
                (Expr::Float(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Float(l - r as f64))),
                
                _ => Err(EvalError {
                    message: format!("Cannot subtract values: {:?} - {:?}", left.node, right.node),
                    message_short: "cannot subtract".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Identifier(name) => {
            match context.get(name) {
                Some(value) => Ok(Flow::Continue(value.clone())),
                None => Err(EvalError {
                    message: format!("Undefined variable: {}", name),
                    message_short: "not defined".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Range { start, end } => {
            Ok(Flow::Continue(Expr::Range {
                start: start.clone(),
                end: end.clone(),
            }))
        }

        Expr::InclusiveRange { start, end } => {
            Ok(Flow::Continue(Expr::InclusiveRange {
                start: start.clone(),
                end: end.clone(),
            }))
        }

        Expr::Call { name, args } => {
            let evaluated_args: Result<Vec<SpannedExpr>, EvalError> = args.iter()
                .map(|arg| {
                    match eval(arg, context) {
                        Ok(v) => Ok(SpannedExpr {
                            node: v.unwrap(),
                            span: arg.span,
                        }),

                        Err(e) => Err(e),
                    }
                })
                .collect();

            match context.get(name) {
                Some(v) => {
                    match v {
                        Expr::InternalFunction { name, args, func } => {
                            if !args.contains(&"__args__".to_string()) && args.len() != evaluated_args.as_ref().map_or(0, |a| a.len()) {
                                return Err(EvalError {
                                    message: format!("Function {} expects {} arguments, got {}", name, args.len(), evaluated_args.as_ref().map_or(0, |a| a.len())),
                                    message_short: format!("got {} arguments too many", evaluated_args.as_ref().map_or(0, |a| a.len()) - args.len()),
                                    span: expr.span,
                                });
                            }

                            match func(evaluated_args?) {
                                Ok(response) => Ok(Flow::Continue(response.return_value.node)),
                                Err((msg, span)) => Err(EvalError {
                                    message: msg.clone(),
                                    message_short: msg,
                                    span,
                                }),
                            }
                        }

                        Expr::Function { name, args, body } => {
                            if args.len() != evaluated_args.as_ref().map_or(0, |a| a.len()) {
                                return Err(EvalError {
                                    message: format!("Function {} expects {} arguments, got {}", name, args.len(), evaluated_args.as_ref().map_or(0, |a| a.len())),
                                    message_short: format!("got {} arguments too many", evaluated_args.as_ref().map_or(0, |a| a.len()) - args.len()),
                                    span: expr.span,
                                });
                            }

                            let mut new_context = context.clone();

                            for (i, arg_name) in args.iter().enumerate() {
                                new_context.insert(arg_name.clone(), evaluated_args.as_ref().unwrap()[i].node.clone());
                            }

                            match eval(body, &mut new_context)? {
                                Flow::Continue(v) => Ok(Flow::Continue(v)),
                                Flow::Return(v) => Ok(Flow::Continue(v)),
                                Flow::Break => Err(EvalError {
                                    message: "Unexpected break in function".to_string(),
                                    message_short: "unexpected break".to_string(),
                                    span: expr.span,
                                }),
                            }
                        }

                        _ => Err(EvalError {
                            message: format!("{} is not a function", name),
                            message_short: "not a function".to_string(),
                            span: expr.span,
                        })
                    }
                }

                None => Err(EvalError {
                    message: format!("Undefined function: {}", name),
                    message_short: "not defined".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Let { name, value } => {
            let value = eval(value, context)?.unwrap();
            context.insert(name.clone(), value);
            
            Ok(Flow::Continue(Expr::Null))

        }

        Expr::Function { name, args, body } => {
            context.insert(name.clone(), Expr::Function {
                name: name.clone(),
                args: args.clone(),
                body: body.clone(),
            });

            Ok(Flow::Continue(Expr::Null))
        }

        Expr::Block(exprs) => {
            let preexisting_keys = context.keys().cloned().collect::<Vec<String>>();

            for e in exprs {
                match eval(e, context)? {
                    Flow::Continue(_) => {},
                    Flow::Return(v) => return Ok(Flow::Return(v)),
                    Flow::Break => return Ok(Flow::Break),
                }
            }

            for key in context.keys().cloned().collect::<Vec<String>>() {
                if !preexisting_keys.contains(&key) {
                    context.remove(&key);
                }
            }

            Ok(Flow::Continue(Expr::Null))
        }

        Expr::InfiniteLoop { body } => {
            loop {
                match eval(body, context)? {
                    Flow::Continue(_) => {},
                    Flow::Return(v) => return Ok(Flow::Return(v)),
                    Flow::Break => return Ok(Flow::Continue(Expr::Null)),
                }
            }
        }

        Expr::ForLoop { iterator_name, iterator_range, body } => {
            let range_value = eval(iterator_range, context)?.unwrap();

            match range_value {
                Expr::Range { start, end } => {
                    let start = match eval(&start, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("Range start must be an integer, got {:?}", start),
                                message_short: "invalid range start".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    let end = match eval(&end, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("Range end must be an integer, got {:?}", end),
                                message_short: "invalid range end".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    for i in start..end {
                        context.insert(iterator_name.clone(), Expr::Int(i));

                        match eval(body, context)? {
                            Flow::Continue(_) => {},
                            Flow::Return(v) => return Ok(Flow::Return(v)),
                            Flow::Break => break,
                        }
                    }

                    Ok(Flow::Continue(Expr::Null))
                }

                Expr::InclusiveRange { start, end } => {
                    let start = match eval(&start, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("Range start must be an integer, got {:?}", start),
                                message_short: "invalid range start".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    let end = match eval(&end, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("Range end must be an integer, got {:?}", end),
                                message_short: "invalid range end".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    for i in start..=end {
                        context.insert(iterator_name.clone(), Expr::Int(i));

                        match eval(body, context)? {
                            Flow::Continue(_) => {},
                            Flow::Return(v) => return Ok(Flow::Return(v)),
                            Flow::Break => break,
                        }
                    }

                    Ok(Flow::Continue(Expr::Null))
                }

                _ => Err(EvalError {
                    message: format!("Cannot iterate over value: {:?}", range_value),
                    message_short: "cannot iterate".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Return(value) => {
            let return_value = eval(value, context)?.unwrap();
            Ok(Flow::Return(return_value))
        }

        Expr::Break => {
            Ok(Flow::Break)
        },

        Expr::Equal(left, right) => {
            let left_value = eval(left, context)?.unwrap();
            let right_value = eval(right, context)?.unwrap();

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Bool(l == r))),
                (Expr::Float(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Bool(l == r))),
                (Expr::Int(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Bool((l as f64) == r))),
                (Expr::Float(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Bool(l == (r as f64)))),
                (Expr::Bool(l), Expr::Bool(r)) => Ok(Flow::Continue(Expr::Bool(l == r))),
                (Expr::String(l), Expr::String(r)) => Ok(Flow::Continue(Expr::Bool(l == r))),
                (Expr::Null, Expr::Null) => Ok(Flow::Continue(Expr::Bool(true))),

                _ => Ok(Flow::Continue(Expr::Bool(false))),
            }
        },

        Expr::NotEqual(left, right) => {
            let left_value = eval(left, context)?.unwrap();
            let right_value = eval(right, context)?.unwrap();

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Bool(l != r))),
                (Expr::Float(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Bool(l != r))),
                (Expr::Int(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Bool((l as f64) != r))),
                (Expr::Float(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Bool(l != (r as f64)))),
                (Expr::Bool(l), Expr::Bool(r)) => Ok(Flow::Continue(Expr::Bool(l != r))),
                (Expr::String(l), Expr::String(r)) => Ok(Flow::Continue(Expr::Bool(l != r))),
                (Expr::Null, Expr::Null) => Ok(Flow::Continue(Expr::Bool(false))),

                _ => Ok(Flow::Continue(Expr::Bool(true))),
            }
        },

        Expr::LessThan(left, right) => {
            let left_value = eval(left, context)?.unwrap();
            let right_value = eval(right, context)?.unwrap();

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Bool(l < r))),
                (Expr::Float(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Bool(l < r))),
                (Expr::Int(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Bool((l as f64) < r))),
                (Expr::Float(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Bool(l < (r as f64)))),

                _ => Err(EvalError {
                    message: format!("Cannot compare values: {:?} < {:?}", left.node, right.node),
                    message_short: "cannot compare".to_string(),
                    span: expr.span,
                }),
            }
        },

        Expr::LessThanOrEqual(left, right) => {
            let left_value = eval(left, context)?.unwrap(); 
            let right_value = eval(right, context)?.unwrap();

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Bool(l <= r))),
                (Expr::Float(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Bool(l <= r))),
                (Expr::Int(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Bool((l as f64) <= r))),
                (Expr::Float(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Bool(l <= (r as f64)))),

                _ => Err(EvalError {
                    message: format!("Cannot compare values: {:?} <= {:?}", left.node, right.node),
                    message_short: "cannot compare".to_string(),
                    span: expr.span,
                }),
            }
        },

        Expr::GreaterThan(left, right) => {
            let left_value = eval(left, context)?.unwrap();
            let right_value = eval(right, context)?.unwrap();

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Bool(l > r))),
                (Expr::Float(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Bool(l > r))),
                (Expr::Int(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Bool((l as f64) > r))),
                (Expr::Float(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Bool(l > (r as f64)))),

                _ => Err(EvalError {
                    message: format!("Cannot compare values: {:?} > {:?}", left.node, right.node),
                    message_short: "cannot compare".to_string(),
                    span: expr.span,
                }),
            }
        },

        Expr::GreaterThanOrEqual(left, right) => {
            let left_value = eval(left, context)?.unwrap();
            let right_value = eval(right, context)?.unwrap();

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Bool(l >= r))),
                (Expr::Float(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Bool(l >= r))),
                (Expr::Int(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Bool((l as f64) >= r))),
                (Expr::Float(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Bool(l >= (r as f64)))),

                _ => Err(EvalError {
                    message: format!("Cannot compare values: {:?} >= {:?}", left.node, right.node),
                    message_short: "cannot compare".to_string(),
                    span: expr.span,
                }),
            }
        },

        v => {
            Err(EvalError {
                message: format!("No evaluator for {:?}", v),
                message_short: "couldn't evaluate".to_string(),
                span: expr.span,
            })
        }
    }
}