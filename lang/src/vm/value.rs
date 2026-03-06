pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    Array(Vec<Value>),
    Function { chunk_id: usize, arity: usize },
}

impl Value {
    fn truthy(&self) -> bool {
        match self {
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Array(arr) => !arr.is_empty(),
            Value::Function { .. } => true,
        }
    }
}