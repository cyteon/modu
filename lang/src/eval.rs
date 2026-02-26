use std::collections::HashMap;
use chumsky::span::SimpleSpan;

use crate::ast::{Expr, SpannedExpr, AssignOp};
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
    Skip,
}

impl Flow {
    fn unwrap(self) -> Expr {
        match self {
            Flow::Continue(v) | Flow::Return(v) => v,
            Flow::Break | Flow::Skip => Expr::Null,
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

        Expr::PropertyAccess { object, property } => {
            let object = eval(object, context)?.unwrap();

            match object {
                Expr::Module { symbols } => {
                    match symbols.get(property) {
                        Some(value) => Ok(Flow::Continue(value.node.clone())),
                        None => Err(EvalError {
                            message: format!("module has no method named '{}'", property),
                            message_short: "no such property".to_string(),
                            span: expr.span,
                        }),
                    }
                }
                
                Expr::Object { properties } => {
                    match properties.get(property) {
                        Some(value) => Ok(Flow::Continue(value.clone())),
                        None => {
                            match crate::builtins::object::get_fn(property) {
                                Some(value) => Ok(Flow::Continue(value)),
                                None => Err(EvalError {
                                    message: format!("object has no method named '{}'", property),
                                    message_short: "no such property".to_string(),
                                    span: expr.span,
                                }),
                            }
                        }
                    }
                }

                Expr::Array(_) => {
                    match crate::builtins::array::get_fn(property) {
                        Some(value) => Ok(Flow::Continue(value)),
                        None => Err(EvalError {
                            message: format!("array has no method named '{}'", property),
                            message_short: "no such property".to_string(),
                            span: expr.span,
                        }),
                    }
                }

                Expr::String(_) => {
                    match crate::builtins::string::get_fn(property) {
                        Some(value) => Ok(Flow::Continue(value)),
                        None => Err(EvalError {
                            message: format!("string has no method named '{}'", property),
                            message_short: "no such property".to_string(),
                            span: expr.span,
                        }),
                    }
                }

                Expr::Int(_) => {
                    match crate::builtins::int::get_fn(property) {
                        Some(value) => Ok(Flow::Continue(value)),
                        None => Err(EvalError {
                            message: format!("int has no method named '{}'", property),
                            message_short: "no such property".to_string(),
                            span: expr.span,
                        }),
                    }
                }

                Expr::Float(_) => {
                    match crate::builtins::float::get_fn(property) {
                        Some(value) => Ok(Flow::Continue(value)),
                        None => Err(EvalError {
                            message: format!("float has no method named '{}'", property),
                            message_short: "no such property".to_string(),
                            span: expr.span,
                        }),
                    }
                }

                #[cfg(not(target_arch = "wasm32"))]
                Expr::File(_) => {
                    match crate::libraries::fs::get_fn(property) {
                        Some(value) => Ok(Flow::Continue(value)),
                        None => Err(EvalError {
                            message: format!("file has no method named '{}'", property),
                            message_short: "no such property".to_string(),
                            span: expr.span,
                        }),
                    }
                }

                #[cfg(not(target_arch = "wasm32"))]
                Expr::FFILibrary(library) => {
                    Ok(Flow::Continue(Expr::FFILibrary(library)))
                }

                _ => Err(EvalError {
                    message: format!("cannot access property '{}' of '{}'", property, object),
                    message_short: "cannot access property".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Neg(inner) => {
            let value = eval(inner, context)?.unwrap();

            match value {
                Expr::Int(n) => Ok(Flow::Continue(Expr::Int(-n))),
                Expr::Float(f) => Ok(Flow::Continue(Expr::Float(-f))),
                _ => Err(EvalError {
                    message: format!("cannot negate value '{}'", value),
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
                    message: format!("cannot add values '{}' and '{}'", left.node, right.node),
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
                    message: format!("cannot subtract values  '{}' and '{}'", left.node, right.node),
                    message_short: "cannot subtract".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Mul(left, right) => {
            let left_value = eval(left, context)?.unwrap();
            let right_value = eval(right, context)?.unwrap();

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Int(l * r))),
                (Expr::Float(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Float(l * r))),
                (Expr::Int(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Float(l as f64 * r))),
                (Expr::Float(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Float(l * r as f64))),
                (Expr::String(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::String(l.repeat(r as usize)))),
                
                _ => Err(EvalError {
                    message: format!("cannot multiply values '{}' and '{}'", left.node, right.node),
                    message_short: "cannot multiply".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Div(left, right) => {
            let left_value = eval(left, context)?.unwrap();
            let right_value = eval(right, context)?.unwrap();

            if matches!(right_value, Expr::Int(0) | Expr::Float(0.0)) {
                return Err(EvalError {
                    message: "division by zero".to_string(),
                    message_short: "division by zero".to_string(),
                    span: expr.span,
                });
            }

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Float(l as f64 / r as f64))),
                (Expr::Float(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Float(l / r))),
                (Expr::Int(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Float(l as f64 / r))),
                (Expr::Float(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Float(l / r as f64))),
                
                _ => Err(EvalError {
                    message: format!("cannot divide values '{}' and '{}'", left.node, right.node),
                    message_short: "cannot divide".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Pow(left, right) => {
            let left_value = eval(left, context)?.unwrap();
            let right_value = eval(right, context)?.unwrap();

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Int(l.pow(r as u32)))),
                (Expr::Float(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Float(l.powf(r)))),
                (Expr::Int(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Float((l as f64).powf(r)))),
                (Expr::Float(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Float(l.powi(r as i32)))),
                
                _ => Err(EvalError {
                    message: format!("cannot exponentiate values '{}' and '{}'", left.node, right.node),
                    message_short: "cannot exponentiate".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Mod(left, right) => {
            let left_value = eval(left, context)?.unwrap();
            let right_value = eval(right, context)?.unwrap();

            if matches!(right_value, Expr::Int(0) | Expr::Float(0.0)) {
                return Err(EvalError {
                    message: "modulo by zero".to_string(),
                    message_short: "modulo by zero".to_string(),
                    span: expr.span,
                });
            }

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Int(l % r))),
                (Expr::Float(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Float(l % r))),
                (Expr::Int(l), Expr::Float(r)) => Ok(Flow::Continue(Expr::Float((l as f64) % r))),
                (Expr::Float(l), Expr::Int(r)) => Ok(Flow::Continue(Expr::Float(l % (r as f64)))),
                
                _ => Err(EvalError {
                    message: format!("cannot modulo values '{}' and '{}'", left.node, right.node),
                    message_short: "cannot modulo".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Identifier(name) => {
            match context.get(name) {
                Some(value) => Ok(Flow::Continue(value.clone())),
                None => Err(EvalError {
                    message: format!("undefined variable '{}'", name),
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

        Expr::Call { callee, args } => {
            let mut evaluated_args: Vec<SpannedExpr> = args.iter()
                .map(|arg| {
                    match eval(arg, context) {
                        Ok(v) => Ok(SpannedExpr {
                            node: v.unwrap(),
                            span: arg.span,
                        }),

                        Err(e) => Err(e),
                    }
                })
                .collect::<Result<Vec<SpannedExpr>, EvalError>>()?;

            match eval(callee, context)?.unwrap() {
                Expr::InternalFunction { name, args, func } => {
                    if args.contains(&"self".to_string()) {
                        match &callee.node {
                            Expr::PropertyAccess { object, .. } => {
                                evaluated_args.insert(0, SpannedExpr {
                                    node: eval(object, context)?.unwrap(),
                                    span: object.span,
                                });
                            }

                            _ => { }
                        }
                    }

                    if !args.contains(&"__args__".to_string()) && args.len() != evaluated_args.len() {
                        let error_span = if evaluated_args.len() > args.len() {
                            SimpleSpan::from(
                                evaluated_args[args.len()].span.start..evaluated_args[evaluated_args.len() - 1].span.end
                            )
                        } else {
                            expr.span
                        };
                        
                        if evaluated_args.len() > args.len() {
                            return Err(EvalError {
                                message: format!("function '{}' expects {} arguments, got {}", name, args.len(), evaluated_args.len()),
                                message_short: format!("{} arguments too many", evaluated_args.len() - args.len()),
                                span: error_span,
                            });
                        } else {
                            return Err(EvalError {
                                message: format!("function '{}' expects {} arguments, got {}", name, args.len(), evaluated_args.len()),
                                message_short: format!("{} arguments too few", args.len() - evaluated_args.len()),
                                span: error_span,
                            });
                        }
                    }

                    match func(evaluated_args) {
                        Ok(response) => {
                            if let Some(replace_self) = response.replace_self {
                                match &callee.node {
                                    Expr::PropertyAccess { object, property: _ } => {
                                        if let Expr::Identifier(obj_name) = &object.node {
                                            context.insert(obj_name.clone(), replace_self);
                                        }
                                    }
                                    
                                    _ => {}
                                }
                            }

                            Ok(Flow::Continue(response.return_value))
                        },
                        Err((msg, span)) => Err(EvalError {
                            message: msg.clone(),
                            message_short: msg,
                            span,
                        }),
                    }
                }

                Expr::Function { name, args, body } => {
                    if args.len() != evaluated_args.len() {
                        let error_span = if evaluated_args.len() > args.len() {
                            SimpleSpan::from(
                                evaluated_args[args.len()].span.start..evaluated_args[evaluated_args.len() - 1].span.end
                            )
                        } else {
                            expr.span
                        };

                        if evaluated_args.len() > args.len() {
                            return Err(EvalError {
                                message: format!("function '{}' expects {} arguments, got {}", name, args.len(), evaluated_args.len()),
                                message_short: format!("{} arguments too many", evaluated_args.len() - args.len()),
                                span: error_span,
                            });
                        } else {
                            return Err(EvalError {
                                message: format!("function '{}' expects {} arguments, got {}", name, args.len(), evaluated_args.len()),
                                message_short: format!("{} arguments too few", args.len() - evaluated_args.len()),
                                span: error_span,
                            });
                        }
                    }

                    let mut new_context = context.clone();

                    for (i, arg_name) in args.iter().enumerate() {
                        new_context.insert(arg_name.clone(), evaluated_args[i].node.clone());
                    }

                    match eval(&*body, &mut new_context)? {
                        Flow::Continue(v) => Ok(Flow::Continue(v)),
                        Flow::Return(v) => Ok(Flow::Continue(v)),
                        Flow::Break => Err(EvalError {
                            message: "unexpected break statement in function".to_string(),
                            message_short: "unexpected break".to_string(),
                            span: expr.span,
                        }),
                        Flow::Skip => Err(EvalError {
                            message: "unexpected continue statement in function".to_string(),
                            message_short: "unexpected continue".to_string(),
                            span: expr.span,
                        }),
                    }
                }

                #[cfg(not(target_arch = "wasm32"))]
                Expr::FFILibrary(library) => {
                    let result = crate::libraries::ffi::execute_ffi_call(
                        &library,
                        match &callee.node {
                            Expr::PropertyAccess { property, .. } => property,
                            _ => unreachable!(),
                        },
                        evaluated_args,
                    );

                    match result {
                        Ok(value) => Ok(Flow::Continue(value)),
                        Err(msg) => Err(EvalError {
                            message: msg.clone(),
                            message_short: msg,
                            span: expr.span,
                        }),
                    }
                }

                v => Err(EvalError {
                    message: format!("'{}' is not a function", v),
                    message_short: "not a function".to_string(),
                    span: expr.span,
                })
            }
        }

        Expr::Let { name, value } => {
            let value = eval(value, context)?.unwrap();
            context.insert(name.clone(), value);
            
            Ok(Flow::Continue(Expr::Null))
        }

        Expr::Assign { name, value, operator } => {
            if !context.contains_key(name) {
                return Err(EvalError {
                    message: format!("undefined variable '{}'", name),
                    message_short: "define with let".to_string(),
                    span: expr.span,
                });
            }

            let value = eval(value, context)?.unwrap();

            match operator {
                None => {
                    context.insert(name.clone(), value);
                    Ok(Flow::Continue(Expr::Null))
                },

                Some(op) => {
                    match op {
                        AssignOp::Add => {
                            let current_value = context.get(name).unwrap().clone();
                            let new_value = match (current_value, value) {
                                (Expr::Int(l), Expr::Int(r)) => Expr::Int(l + r),
                                (Expr::Float(l), Expr::Float(r)) => Expr::Float(l + r),
                                (Expr::Int(l), Expr::Float(r)) => Expr::Float(l as f64 + r),
                                (Expr::Float(l), Expr::Int(r)) => Expr::Float(l + r as f64),
                                (Expr::String(l), Expr::String(r)) => Expr::String(l + &r),

                                (l, r) => return Err(EvalError {
                                    message: format!("cannot add values '{}' and '{}'", l, r),
                                    message_short: "cannot add".to_string(),
                                    span: expr.span,
                                }),
                            };

                            context.insert(name.clone(), new_value);
                            Ok(Flow::Continue(Expr::Null))
                        },

                        AssignOp::Sub => {
                            let current_value = context.get(name).unwrap().clone();
                            let new_value = match (current_value, value) {
                                (Expr::Int(l), Expr::Int(r)) => Expr::Int(l - r),
                                (Expr::Float(l), Expr::Float(r)) => Expr::Float(l - r),
                                (Expr::Int(l), Expr::Float(r)) => Expr::Float(l as f64 - r),
                                (Expr::Float(l), Expr::Int(r)) => Expr::Float(l - r as f64),
                                
                                (l, r) => return Err(EvalError {
                                    message: format!("cannot subtract values '{}' and '{}'", l, r),
                                    message_short: "cannot subtract".to_string(),
                                    span: expr.span,
                                }),
                            };

                            context.insert(name.clone(), new_value);
                            Ok(Flow::Continue(Expr::Null))
                        },

                        AssignOp::Mul => {
                            let current_value = context.get(name).unwrap().clone();
                            let new_value = match (current_value, value) {
                                (Expr::Int(l), Expr::Int(r)) => Expr::Int(l * r),
                                (Expr::Float(l), Expr::Float(r)) => Expr::Float(l * r),
                                (Expr::Int(l), Expr::Float(r)) => Expr::Float(l as f64 * r),
                                (Expr::Float(l), Expr::Int(r)) => Expr::Float(l * r as f64),
                                (Expr::String(l), Expr::Int(r)) => Expr::String(l.repeat(r as usize)),
                                
                                (l, r) => return Err(EvalError {
                                    message: format!("cannot multiply values '{}' and '{}'", l, r),
                                    message_short: "cannot multiply".to_string(),
                                    span: expr.span,
                                }),
                            };

                            context.insert(name.clone(), new_value);
                            Ok(Flow::Continue(Expr::Null))
                        },

                        AssignOp::Div => {
                            let current_value = context.get(name).unwrap().clone();

                            if matches!(value, Expr::Int(0) | Expr::Float(0.0)) {
                                return Err(EvalError {
                                    message: "division by zero".to_string(),
                                    message_short: "division by zero".to_string(),
                                    span: expr.span,
                                });
                            }

                            let new_value = match (current_value, value) {
                                (Expr::Int(l), Expr::Int(r)) => Expr::Float(l as f64 / r as f64),
                                (Expr::Float(l), Expr::Float(r)) => Expr::Float(l / r),
                                (Expr::Int(l), Expr::Float(r)) => Expr::Float(l as f64 / r),
                                (Expr::Float(l), Expr::Int(r)) => Expr::Float(l / r as f64),
                                
                                (l, r) => return Err(EvalError {
                                    message: format!("cannot divide values '{}' and '{}'", l, r),
                                    message_short: "cannot divide".to_string(),
                                    span: expr.span,
                                }),
                            };

                            context.insert(name.clone(), new_value);
                            Ok(Flow::Continue(Expr::Null))
                        },

                        AssignOp::Mod => {
                            let current_value = context.get(name).unwrap().clone();

                            if matches!(value, Expr::Int(0) | Expr::Float(0.0)) {
                                return Err(EvalError {
                                    message: "modulo by zero".to_string(),
                                    message_short: "modulo by zero".to_string(),
                                    span: expr.span,
                                });
                            }

                            let new_value = match (current_value, value) {
                                (Expr::Int(l), Expr::Int(r)) => Expr::Int(l % r),
                                (Expr::Float(l), Expr::Float(r)) => Expr::Float(l % r),
                                (Expr::Int(l), Expr::Float(r)) => Expr::Float((l as f64) % r),
                                (Expr::Float(l), Expr::Int(r)) => Expr::Float(l % (r as f64)),
                                
                                (l, r) => return Err(EvalError {
                                    message: format!("cannot modulo values '{}' and '{}'", l, r),
                                    message_short: "cannot modulo".to_string(),
                                    span: expr.span,
                                }),
                            };

                            context.insert(name.clone(), new_value);
                            Ok(Flow::Continue(Expr::Null))
                        }
                    }
                }
            }
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
                    Flow::Skip => return Ok(Flow::Skip),
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
                    Flow::Skip => continue,
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
                                message: format!("range start must be an integer, got '{}'", start.node),
                                message_short: "invalid range start".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    let end = match eval(&end, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("range end must be an integer, got '{}'", end.node),
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
                            Flow::Skip => continue,
                        }
                    }

                    Ok(Flow::Continue(Expr::Null))
                }

                Expr::InclusiveRange { start, end } => {
                    let start = match eval(&start, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("range start must be an integer, got '{}'", start.node),
                                message_short: "invalid range start".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    let end = match eval(&end, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("range end must be an integer, got '{}'", end.node),
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
                            Flow::Skip => continue,
                        }
                    }

                    Ok(Flow::Continue(Expr::Null))
                }

                Expr::Array(elements) => {
                    for element in elements {
                        let value = eval(&element, context)?.unwrap();
                        context.insert(iterator_name.clone(), value);

                        match eval(body, context)? {
                            Flow::Continue(_) => {},
                            Flow::Return(v) => return Ok(Flow::Return(v)),
                            Flow::Break => break,
                            Flow::Skip => continue,
                        }
                    }

                    Ok(Flow::Continue(Expr::Null))
                }

                _ => Err(EvalError {
                    message: format!("cannot iterate over value '{}'", range_value),
                    message_short: "cannot iterate".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::WhileLoop { condition, body } => {
            loop {
                let condition_value = eval(condition, context)?.unwrap();

                match condition_value {
                    Expr::Bool(true) => {
                        match eval(body, context)? {
                            Flow::Continue(_) => {},
                            Flow::Return(v) => return Ok(Flow::Return(v)),
                            Flow::Break => break,
                            Flow::Skip => continue,
                        }
                    },

                    Expr::Bool(false) => break,

                    _ => return Err(EvalError {
                        message: format!("while loop condition must be a boolean, got '{}'", condition_value),
                        message_short: "invalid while condition".to_string(),
                        span: expr.span,
                    }),
                }
            }

            Ok(Flow::Continue(Expr::Null))
        }

        Expr::Return(value) => {
            let return_value = eval(value, context)?.unwrap();
            Ok(Flow::Return(return_value))
        }

        Expr::Break => {
            Ok(Flow::Break)
        },

        Expr::Continue => {
            Ok(Flow::Skip)
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
                    message: format!("cannot compare values '{}' and '{}'", left.node, right.node),
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
                    message: format!("cannot compare values '{}' and '{}'", left.node, right.node),
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
                    message: format!("cannot compare values '{}' and '{}'", left.node, right.node),
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
                    message: format!("cannot compare values '{}' and '{}'", left.node, right.node),
                    message_short: "cannot compare".to_string(),
                    span: expr.span,
                }),
            }
        },

        Expr::If { condition, then_branch, else_if_branches, else_branch } => {
            let condition_value = eval(condition, context)?.unwrap();

            match condition_value {
                Expr::Bool(true) => eval(then_branch, context),
                
                Expr::Bool(false) | Expr::Null => {
                    if !else_if_branches.is_empty() {
                        for (else_if_condition, else_if_branch) in else_if_branches {
                            let else_if_condition_value = eval(else_if_condition, context)?.unwrap();

                            match else_if_condition_value {
                                Expr::Bool(true) => return eval(else_if_branch, context),
                                Expr::Bool(false) | Expr::Null => continue,

                                _ => return Err(EvalError {
                                    message: format!("else if condition must be a boolean, got '{}'", else_if_condition_value),
                                    message_short: "invalid else if condition".to_string(),
                                    span: expr.span,
                                }),
                            }
                        }

                        if let Some(else_branch) = else_branch {
                            eval(else_branch, context)
                        } else {
                            Ok(Flow::Continue(Expr::Null))
                        }
                    } else if let Some(else_branch) = else_branch {
                        eval(else_branch, context)
                    } else {
                        Ok(Flow::Continue(Expr::Null))
                    }
                },

                Expr::Int(n) => {
                    if n != 0 {
                        eval(then_branch, context)
                    } else {
                        if !else_if_branches.is_empty() {
                            for (else_if_condition, else_if_branch) in else_if_branches {
                                let else_if_condition_value = eval(else_if_condition, context)?.unwrap();

                                match else_if_condition_value {
                                    Expr::Bool(true) => return eval(else_if_branch, context),
                                    Expr::Bool(false) | Expr::Null => continue,

                                    _ => return Err(EvalError {
                                        message: format!("else if condition must be a boolean, got '{}'", else_if_condition_value),
                                        message_short: "invalid else if condition".to_string(),
                                        span: expr.span,
                                    }),
                                }
                            }

                            if let Some(else_branch) = else_branch {
                                eval(else_branch, context)
                            } else {
                                Ok(Flow::Continue(Expr::Null))
                            }
                        } else if let Some(else_branch) = else_branch {
                            eval(else_branch, context)
                        } else {
                            Ok(Flow::Continue(Expr::Null))
                        }
                    }
                }

                // so u can do like "if var", and it will run if var is not null
                _ => {
                    eval(then_branch, context)
                }
            }
        }

        Expr::Import { name, import_as } => {
            let import_as = match import_as {
                Some(as_name) => as_name.clone(),
                None => name.clone(),
            };

            let mut path: std::path::PathBuf = std::path::PathBuf::new();

            #[cfg(not(target_arch = "wasm32"))]
            {
                path = std::env::current_dir().unwrap();
                
                let sys_args = std::env::args().collect::<Vec<String>>();
                if sys_args.len() > 2 && sys_args[1] == "run" {
                    path.push(&sys_args[2]);
                    path.pop();
                }

                if context.contains_key("CURRENTLY_PARSING_MODULE_PATH") {
                    if let Expr::String(current_module_path) = context.get("CURRENTLY_PARSING_MODULE_PATH").unwrap() {
                        let mut module_path = std::path::PathBuf::from(current_module_path);
                        module_path.pop();
                        path = module_path;
                    }
                }

                if context.contains_key("CURRENTLY_PARSING_PACKAGE_NAME") {
                    if let Expr::String(current_package_name) = context.get("CURRENTLY_PARSING_PACKAGE_NAME").unwrap() {
                        path.push(".modu");
                        path.push("packages");
                        path.push(current_package_name);
                    }
                }
            }

            if name.ends_with(".modu") {
                path.push(name);
                
                let source = std::fs::read_to_string(path.clone()).map_err(|e| EvalError {
                    message: format!("Failed to read module file {}: {}", name, e),
                    message_short: "failed to read module".to_string(),
                    span: expr.span,
                })?;

                let mut new_context = crate::utils::create_context();
                new_context.insert(
                    "CURRENTLY_PARSING_MODULE_PATH".to_string(),
                    Expr::String(path.to_str().unwrap().to_string())
                );

                crate::parser::parse(&source, path.to_str().unwrap(), &mut new_context);

                if import_as == "*" {
                    for (k, v) in new_context {
                        context.insert(k, v);
                    }
                } else {
                    let mut symbols = HashMap::new();

                    for (k, v) in new_context.iter().filter(|(k, _)| !crate::utils::create_context().contains_key(*k)) {
                        symbols.insert(k.clone(), SpannedExpr {
                            node: v.clone(),
                            span: expr.span,
                        });
                    }

                    context.insert(import_as.clone().replace(".modu", ""), Expr::Module {
                        symbols,
                    });
                }
            } else {
                match crate::libraries::get_package(name) {
                    Some(module) => {
                        if import_as == "*" {
                            if let Expr::Module { symbols } = module {
                                for (k, v) in symbols {
                                    context.insert(k, v.node);
                                }
                            } else {
                                return Err(EvalError {
                                    message: format!("package '{}' is not a module", name),
                                    message_short: "not a module".to_string(),
                                    span: expr.span,
                                });
                            }
                        } else {
                            context.insert(import_as.clone(), module);
                        }
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    None => {
                        path.push(".modu");
                        path.push("packages");
                        path.push(name);
                        path.push("lib.modu");

                        if !path.exists() {
                            return Err(EvalError {
                                message: format!("package '{}' does not exist or is not installed", name),
                                message_short: "package not found".to_string(),
                                span: expr.span,
                            });
                        }

                        let source = std::fs::read_to_string(path.clone()).map_err(|e| EvalError {
                            message: format!("failed to read module file for package '{}': {}", name, e),
                            message_short: "failed to read module".to_string(),
                            span: expr.span,
                        })?;

                        let mut new_context = crate::utils::create_context();
                        new_context.insert(
                            "CURRENTLY_PARSING_PACKAGE_PATH".to_string(),
                            Expr::String(path.to_str().unwrap().to_string())
                        );
                        new_context.insert(
                            "CURRENTLY_PARSING_PACKAGE_NAME".to_string(),
                            Expr::String(name.clone()),
                        );

                        crate::parser::parse(&source, path.to_str().unwrap(), &mut new_context);

                        if import_as == "*" {
                            for (k, v) in new_context {
                                context.insert(k, v);
                            }
                        } else {
                            let mut symbols = HashMap::new();

                            for (k, v) in new_context.iter().filter(|(k, _)| !crate::utils::create_context().contains_key(*k)) {
                                symbols.insert(k.clone(), SpannedExpr {
                                    node: v.clone(),
                                    span: expr.span,
                                });
                            }

                            context.insert(import_as.clone().replace(".modu", ""), Expr::Module {
                                symbols,
                            });
                        }
                    }

                    #[cfg(target_arch = "wasm32")]
                    None => {
                        return Err(EvalError {
                            message: format!("could not find package '{}'", name),
                            message_short: "package not found".to_string(),
                            span: expr.span,
                        });
                    }
                }

               
            }

            Ok(Flow::Continue(Expr::Null))
        }

        Expr::Array(elements) => {
            let mut evaluated_elements = Vec::new();

            for element in elements {
                let value = eval(element, context)?.unwrap();
                evaluated_elements.push(SpannedExpr {
                    node: value,
                    span: element.span,
                });
            }

            Ok(Flow::Continue(Expr::Array(evaluated_elements)))
        }

        Expr::IndexAccess { object, index } => {
            let object_value = eval(object, context)?.unwrap();
            let index_value = eval(index, context)?.unwrap();
            
            match (object_value, index_value) {
                (Expr::Array(elements), Expr::Int(i)) => {
                    let idx = if i < 0 {
                        elements.len() as i64 + i
                    } else {
                        i
                    };

                    if idx < 0 || idx >= elements.len() as i64 {
                        return Err(EvalError {
                            message: format!("index {} is out of bounds", i),
                            message_short: "index out of bounds".to_string(),
                            span: expr.span,
                        });
                    }

                    Ok(Flow::Continue(elements[idx as usize].node.clone()))
                }

                (Expr::Array(elements), Expr::Range { start, end }) => {
                    let start = match eval(&start, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("range start must be an integer, got '{}'", start.node),
                                message_short: "invalid range start".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    let end = match eval(&end, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("range end must be an integer, got '{}'", end.node),
                                message_short: "invalid range end".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    let len = elements.len() as i64;

                    let start_idx = if start < 0 {
                        len + start
                    } else {
                        start
                    };

                    let end_idx = if end < 0 {
                        len + end
                    } else {
                        end
                    };

                    if start_idx < 0 || start_idx > len || end_idx < 0 || end_idx > len || start_idx > end_idx {
                        return Err(EvalError {
                            message: format!("range {}..{} is out of bounds", start, end),
                            message_short: "range out of bounds".to_string(),
                            span: expr.span,
                        });
                    }

                    Ok(Flow::Continue(Expr::Array(elements.iter().skip(start_idx as usize).take((end_idx - start_idx) as usize).cloned().collect())))
                }

                (Expr::Array(elements), Expr::InclusiveRange { start, end }) => {
                    let start = match eval(&start, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("range start must be an integer, got '{}'", start.node),
                                message_short: "invalid range start".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    let end = match eval(&end, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("range end must be an integer, got '{}'", end.node),
                                message_short: "invalid range end".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    let len = elements.len() as i64;

                    let start_idx = if start < 0 {
                        len + start
                    } else {
                        start
                    };

                    let end_idx = if end < 0 {
                        len + end
                    } else {
                        end
                    };

                    if start_idx < 0 || start_idx > len || end_idx < 0 || end_idx > len || start_idx > end_idx {
                        return Err(EvalError {
                            message: format!("range {}..={} is out of bounds", start, end),
                            message_short: "range out of bounds".to_string(),
                            span: expr.span,
                        });
                    }

                    Ok(Flow::Continue(Expr::Array(elements.iter().skip(start_idx as usize).take((end_idx - start_idx + 1) as usize).cloned().collect())))
                }

                (Expr::Object { properties }, Expr::String(key)) => {
                    match properties.get(&key) {
                        Some(value) => Ok(Flow::Continue(value.clone())),
                        None => Err(EvalError {
                            message: format!("object has no property named '{}'", key),
                            message_short: "no such property".to_string(),
                            span: expr.span,
                        }),
                    }
                }

                (Expr::String(s), Expr::Int(i)) => {
                    let idx = if i < 0 {
                        s.len() as i64 + i
                    } else {
                        i
                    };

                    if idx < 0 || idx >= s.len() as i64 {
                        return Err(EvalError {
                            message: format!("index {} is out of bounds", i),
                            message_short: "index out of bounds".to_string(),
                            span: expr.span,
                        });
                    }

                    if let Some(char) = s.chars().nth(idx as usize) {
                        Ok(Flow::Continue(Expr::String(char.to_string())))
                    } else {
                        return Err(EvalError {
                            message: format!("could not get char with index {}", i),
                            message_short: "could not get char".to_string(),
                            span: expr.span,
                        });
                    }
                }

                (Expr::String(s), Expr::Range { start, end }) => {
                    let start = match eval(&start, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("range start must be an integer, got '{}'", start.node),
                                message_short: "invalid range start".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    let end = match eval(&end, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("range end must be an integer, got '{}'", end.node),
                                message_short: "invalid range end".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    let len = s.len() as i64;

                    let start_idx = if start < 0 {
                        len + start
                    } else {
                        start
                    };

                    let end_idx = if end < 0 {
                        len + end
                    } else {
                        end
                    };

                    if start_idx < 0 || start_idx > len || end_idx < 0 || end_idx > len || start_idx > end_idx {
                        return Err(EvalError {
                            message: format!("range {}..{} is out of bounds", start, end),
                            message_short: "range out of bounds".to_string(),
                            span: expr.span,
                        });
                    }

                    Ok(Flow::Continue(Expr::String(s.chars().skip(start_idx as usize).take((end_idx - start_idx) as usize).collect())))
                }
                
                (Expr::String(s), Expr::InclusiveRange { start, end }) => {
                    let start = match eval(&start, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("range start must be an integer, got '{}'", start.node),
                                message_short: "invalid range start".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    let end = match eval(&end, context)?.unwrap() {
                        Expr::Int(n) => n,

                        _ => {
                            return Err(EvalError {
                                message: format!("range end must be an integer, got '{}'", end.node),
                                message_short: "invalid range end".to_string(),
                                span: expr.span,
                            });
                        }
                    };

                    let len = s.len() as i64;

                    let start_idx = if start < 0 {
                        len + start
                    } else {
                        start
                    };

                    let end_idx = if end < 0 {
                        len + end
                    } else {
                        end
                    };

                    if start_idx < 0 || start_idx > len || end_idx < 0 || end_idx > len || start_idx > end_idx {
                        return Err(EvalError {
                            message: format!("range {}..={} is out of bounds", start, end),
                            message_short: "range out of bounds".to_string(),
                            span: expr.span,
                        });
                    }

                    Ok(Flow::Continue(Expr::String(s.chars().skip(start_idx as usize).take((end_idx - start_idx + 1) as usize).collect())))
                }

                (_, i) => Err(EvalError {
                    message: format!("'{}' cannot be used as a index", i),
                    message_short: "invalid index".to_string(),
                    span: expr.span,
                }),
            }
        }

        v => {
            Err(EvalError {
                message: format!("no evaluator for '{}', please report this issue", v),
                message_short: "couldn't evaluate".to_string(),
                span: expr.span,
            })
        }
    }
}