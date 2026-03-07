use std::collections::HashMap;

pub type Context = Vec<HashMap<String, crate::ast::Expr>>;

pub fn create_context() -> Vec<HashMap<String, crate::ast::Expr>> {
    let mut global_scope = HashMap::new();
    //crate::functions::fill_context(&mut global_scope);

    vec![global_scope]
}