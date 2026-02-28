use ariadne::{Color, Label, Report, ReportKind, Source};
use crate::ast::{Expr, SpannedExpr};
use crate::lexer::Span;

struct ValidationContext {
    inside_function: usize,
    inside_loop: usize,
}

pub struct ValidationError {
    pub span: Span,
    pub message: String,
}

pub fn validate_ast(ast: &[SpannedExpr], filename: &str, source: &str) -> Result<(), ()> {
    let mut context = ValidationContext {
        inside_function: 0,
        inside_loop: 0,
    };

    for expr in ast {
        if let Err(err) = validate_expr(expr, &mut context) {
            report_error(&err, filename, source);
            return Err(());
        }
    }

    Ok(())
}

fn validate_expr(expr: &SpannedExpr, ctx: &mut ValidationContext) -> Result<(), ValidationError> {
    match &expr.node {
        Expr::Return(_) => {
            if ctx.inside_function == 0 {
                return Err(ValidationError {
                    span: expr.span,
                    message: "Return statement not allowed outside of a function".to_string(),
                });
            }
        }

        Expr::Break => {
            if ctx.inside_loop == 0 {
                return Err(ValidationError {
                    span: expr.span,
                    message: "Break statement not allowed outside of a loop".to_string(),
                });
            }
        }

        Expr::Continue => {
            if ctx.inside_loop == 0 {
                return Err(ValidationError {
                    span: expr.span,
                    message: "Continue statement not allowed outside of a loop".to_string(),
                });
            }
        }

        Expr::Function { body, .. } => {
            validate_expr(&body, ctx)?;
        }

        Expr::WhileLoop { body, .. } | Expr::ForLoop { body, .. } | Expr::InfiniteLoop { body, .. } => {
            ctx.inside_loop += 1;
            validate_expr(&body, ctx)?;
            ctx.inside_loop -= 1;
        }

        Expr::Block(body) => {
            ctx.inside_function += 1;
            for expr in body {
                validate_expr(&expr, ctx)?;
            }
            ctx.inside_function -= 1;
        }

        _ => {}
    }

    Ok(())
}

fn report_error(err: &ValidationError, source_name: &str, code: &str) {
    let report = Report::build(ReportKind::Error, (source_name, err.span.into_range()))
        .with_message(&err.message)
        .with_label(
            Label::new((source_name, err.span.into_range()))
                .with_message(&err.message)
                .with_color(Color::Red),
        )
        .finish();

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