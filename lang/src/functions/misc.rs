use std::collections::HashMap;

use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

pub fn print(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    for arg in args {
        print!("{}", arg.node);
    }
    println!();

    Ok(InternalFunctionResponse {
        return_value: Expr::Null,
        replace_self: None,
    })
}

pub fn input(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    use std::io::{self, Write};

    if args.len() > 1 {
        return Err((
            "input function takes at most one argument".to_string(),
            args[1].span,
        ));
    }

    if args.len() == 1 {
        print!("{}", args[0].node);
    }
    
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    Ok(InternalFunctionResponse {
        return_value: Expr::String(input.trim_end().to_string()),
        replace_self: None,
    })
}

pub fn exit(_: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    std::process::exit(0);
}

pub fn fill_context(context: &mut HashMap<String, Expr>) {
    context.insert(
        "print".to_string(),
        Expr::InternalFunction {
            name: "print".to_string(),
            args: vec!["__args__".to_string()],
            func: print,
        },
    );

    context.insert(
        "input".to_string(),
        Expr::InternalFunction {
            name: "input".to_string(),
            args: vec!["__args__".to_string()],
            func: input,
        },
    );

    context.insert(
        "exit".to_string(),
        Expr::InternalFunction {
            name: "exit".to_string(),
            args: vec!["__args__".to_string()],
            func: exit,
        },
    );
}