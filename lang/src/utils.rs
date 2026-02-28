use std::collections::HashMap;

pub fn create_context() -> HashMap<String, crate::ast::Expr> {
    let mut context = HashMap::new();
    crate::functions::fill_context(&mut context);

    return context;
}