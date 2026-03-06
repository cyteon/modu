use std::collections::HashMap;

pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,

    Array(Vec<Value>),
    Object(HashMap<String, Value>),

    Function { chunk_id: usize, arity: usize },
    InternalFunction(fn(Vec<Value>) -> Value),

    Range {
        start: i64,
        end: i64,
        inclusive: bool,
    }
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
            _ => true,
        }
    }

    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err("Invalid operands for '+'".to_string()),
        }
    }

    pub fn sub(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err("Invalid operands for '-'".to_string()),
        }
    }
}