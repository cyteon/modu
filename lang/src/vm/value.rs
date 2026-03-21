use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,

    Array(Vec<Value>),
    Object(HashMap<String, Value>),

    Function { chunk_id: usize, arity: usize },
    NativeFn(NativeFn),
    BuiltinFn(BuiltinFn),

    FFILib(usize),
    FFIFunc(usize, String), // (lib, fn name)

    Range {
        start: i64,
        end: i64,
        inclusive: bool,
    },

    Class {
        name: String,
        methods: HashMap<String, Value>,
    },

    Instance {
        class_name: String,
        properties: HashMap<String, Value>,
    },

    InstanceFn {
        instance: Box<Value>,
        chunk_id: usize,
        arity: usize,
    }
}

#[derive(Clone)]
pub struct BuiltinFn {
    pub name: String,
    pub func: fn(Vec<Value>) -> Result<Value, String>,
}

impl BuiltinFn {
    pub fn new(name: &str, func: fn(Vec<Value>) -> Result<Value, String>) -> Self {
        Self { name: name.to_string(), func }
    }
}

#[derive(Clone)]
pub struct NativeFn {
    pub name: String,
    pub func: fn(Value, Vec<Value>) -> Result<(Value, Option<Value>), String>, // (return value, value to replace self with)
}

impl NativeFn {
    pub fn new(name: &str, func: fn(Value, Vec<Value>) -> Result<(Value, Option<Value>), String>) -> Self {
        Self { name: name.to_string(), func }
    }
}

impl std::fmt::Debug for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fn {}>", self.name)
    }
}

impl std::fmt::Debug for BuiltinFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin fn {}>", self.name)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,

            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => a == b,
            (Value::FFILib(a), Value::FFILib(b)) => a == b,
            (Value::FFIFunc(lib_a, name_a), Value::FFIFunc(lib_b, name_b)) => lib_a == lib_b && name_a == name_b,

            (Value::Function { chunk_id: a_id, arity: a_arity }, Value::Function { chunk_id: b_id, arity: b_arity }) => a_id == b_id && a_arity == b_arity,
            (Value::NativeFn(a), Value::NativeFn(b)) => a.name == b.name,
            (Value::BuiltinFn(a), Value::BuiltinFn(b)) => a.name == b.name,
            (Value::Class { name: a_name, methods: a_methods }, Value::Class { name: b_name, methods: b_methods }) => a_name == b_name && a_methods == b_methods,

            (
                Value::Instance { class_name: a_class, properties: a_props },
                Value::Instance { class_name: b_class, properties: b_props }
            ) => a_class == b_class && a_props == b_props,

            (
                Value::InstanceFn { instance: a_instance, chunk_id: a_id, arity: a_arity },
                Value::InstanceFn { instance: b_instance, chunk_id: b_id, arity: b_arity }
            ) => a_instance == b_instance && a_id == b_id && a_arity == b_arity,

            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Int(a), Value::Float(b)) => (*a as f64).partial_cmp(b),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Int(b)) => a.partial_cmp(&(*b as f64)),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => {
                let text = Self::process_escape_sequences(s);
                write!(f, "{}", text)
            }
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),

            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| match v {
                    Value::String(s) => format!("\"{}\"", s),
                    _ => format!("{}", v),
                }).collect();

                write!(f, "[{}]", elements.join(", "))
            }

            Value::Object(obj) => {
                let properties: Vec<String> = obj.iter().map(|(k, v)| {
                    let value_str = match v {
                        Value::String(s) => format!("\"{}\"", s),
                        _ => format!("{}", v),
                    };
                    format!("\"{}\": {}", k, value_str)
                }).collect();

                write!(f, "{{ {} }}", properties.join(", "))
            }

            Value::Function { chunk_id, arity } => write!(f, "<fn {}:{}>", chunk_id, arity),
            Value::NativeFn(native_fn) => write!(f, "{:?}", native_fn),
            Value::BuiltinFn(builtin_fn) => write!(f, "{:?}", builtin_fn),
            Value::FFILib(lib_id) => write!(f, "<ffi lib {}>", lib_id),
            Value::FFIFunc(lib_id, name) => write!(f, "<ffi func {} from lib {}>", name, lib_id),
            
            Value::Range { start, end, inclusive } => {
                if *inclusive {
                    write!(f, "{}..={}", start, end)
                } else {
                    write!(f, "{}..{}", start, end)
                }
            }

            Value::Class { name, .. } => write!(f, "<class {}>", name),
            Value::Instance { class_name, .. } => write!(f, "<instance of class {}>", class_name),
            Value::InstanceFn { instance, chunk_id, arity } => write!(f, "<fn {}:{} of {}>", chunk_id, arity, instance),
        }
    }
}

impl Value {
    pub fn process_escape_sequences(s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars();
        
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                if let Some(next) = chars.next() {
                    match next {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        'x' => {
                            let hex: String = chars.by_ref().take(2).collect();
                            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                                result.push(byte as char);
                            } else {
                                result.push('\\');
                                result.push('x');
                                result.push_str(&hex);
                            }
                        }
                        _ => {
                            result.push('\\');
                            result.push(next);
                        }
                    }
                } else {
                    result.push('\\');
                }
            } else {
                result.push(ch);
            }
        }
        
        result
    }

    pub fn truthy(&self) -> bool {
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


    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Null => "null",
            Value::Array(_) => "array",
            Value::Object { .. } => "object",
            Value::Function { .. } => "function",
            Value::NativeFn(_) => "native_fn",
            Value::BuiltinFn(_) => "builtin_fn",
            Value::Range { .. } => "range",
            Value::FFILib(_) => "ffi_lib",
            Value::FFIFunc(_, _) => "ffi_function",
            Value::Class { .. } => "class",
            Value::Instance { .. } => "instance",
            Value::InstanceFn { .. } => "instance_fn",
        }
    }

    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(format!("cannot add {} and {}", self.type_name(), other.type_name())),
        }
    }

    pub fn sub(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err(format!("cannot subtract {} and {}", self.type_name(), other.type_name())),
        }
    }

    pub fn mul(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),

            (Value::String(s), Value::Int(n)) => {
                if *n < 0 {
                    Err("cannot multiply string by negative integer".to_string())
                } else {
                    Ok(Value::String(s.repeat(*n as usize)))
                }
            }

            _ => Err(format!("cannot multiply {} and {}", self.type_name(), other.type_name())),
        }
    }

    pub fn div(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("division by zero".to_string())
                } else {
                    Ok(Value::Float((*a as f64) / (*b as f64)))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err("division by zero".to_string())
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err("division by zero".to_string())
                } else {
                    Ok(Value::Float(*a as f64 / b))
                }
            }
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("division by zero".to_string())
                } else {
                    Ok(Value::Float(a / *b as f64))
                }
            }
            _ => Err(format!("cannot divide {} and {}", self.type_name(), other.type_name())),
        }
    }

    pub fn r#mod(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("modulo by zero".to_string())
                } else {
                    Ok(Value::Int(a % b))
                }
            }

            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err("modulo by zero".to_string())
                } else {
                    Ok(Value::Float(a % b))
                }
            }

            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err("modulo by zero".to_string())
                } else {
                    Ok(Value::Float(*a as f64 % b))
                }
            }

            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 {
                    Err("modulo by zero".to_string())
                } else {
                    Ok(Value::Float(a % *b as f64))
                }
            }

            _ => Err(format!("cannot modulo {} and {}", self.type_name(), other.type_name())),
        }
    }

    pub fn pow(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => {
                if *b < 0 {
                    Ok(Value::Float((*a as f64).powf(*b as f64)))
                } else {
                    Ok(Value::Int(a.pow(*b as u32)))
                }
            },

            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f64).powf(*b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.powf(*b as f64))),

            _ => Err(format!("cannot exponentiate {} and {}", self.type_name(), other.type_name())),
        }
    }

    pub fn neg(&self) -> Result<Value, String> {
        match self {
            Value::Int(n) => Ok(Value::Int(-n)),
            Value::Float(f) => Ok(Value::Float(-f)),
            _ => Err(format!("cannot negate {}", self.type_name())),
        }
    }

    pub fn contains(&self, item: &Value) -> Result<bool, String> {
        match (self, item){
            (Value::String(s), Value::String(sub)) => Ok(s.contains(sub)),
            (Value::Array(arr), item) => Ok(arr.contains(item)),
            _ => Err(format!("cannot check if {} contains {}", self.type_name(), item.type_name())),
        }
    }
}