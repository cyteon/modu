#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    String(String),
    Identifier(String),

    Call {
        name: String,
        args: Vec<Expr>,
    },

    Let {
        name: String,
        value: Box<Expr>,
    }
}