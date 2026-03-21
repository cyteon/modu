use std::collections::HashMap;
use crate::vm::value::{BuiltinFn, Value};

pub fn object() -> Value {
    let mut methods = HashMap::new();

    methods.insert("parse".to_string(), Value::BuiltinFn(BuiltinFn::new("parse", parse)));

    Value::Object(methods)
}

fn parse_obj(value: serde_json::Value) -> Result<Value, String> {
    match value {
        serde_json::Value::Null => Ok(Value::Null),
        serde_json::Value::Bool(b) => Ok(Value::Bool(b)),
        serde_json::Value::Number(n) => {
            if n.is_i64() {
                Ok(Value::Int(n.as_i64().unwrap()))
            } else if n.is_f64() {
                Ok(Value::Float(n.as_f64().unwrap()))
            } else {
                return Err(format!("Unsupported number type in JSON: {}", n));
            }
        }
        
        serde_json::Value::String(s) => Ok(Value::String(s.clone())),
        
        serde_json::Value::Array(mut arr) => {
            let mut vec = Vec::new();

            for item in arr.iter_mut() {
                let item_value = match item {
                    serde_json::Value::Null => Value::Null,
                    serde_json::Value::Bool(b) => Value::Bool(*b),
                    serde_json::Value::Number(n) => {
                        if n.is_i64() {
                            Value::Int(n.as_i64().unwrap())
                        } else if n.is_f64() {
                            Value::Float(n.as_f64().unwrap())
                        } else {
                            return Err(format!("Unsupported number type in JSON: {}", n));
                        }
                    }
                    
                    serde_json::Value::String(s) => Value::String(s.clone()),
                    
                    serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
                        parse_obj(item.clone())?
                    }
                };

                vec.push(item_value);
            }

            Ok(Value::Array(vec))
        }

        serde_json::Value::Object(obj) => {
            let mut properties = HashMap::new();
            
            for (k, v) in obj.into_iter() {
                properties.insert(k.clone(), parse_obj(v)?);
            }

            Ok(Value::Object(properties))
        }
    }
}

pub fn parse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("json.parse() takes exactly one argument ({} given)", args.len()));
    }

    let json_str = match &args[0] {
        Value::String(s) => Value::process_escape_sequences(s),
        _ => return Err(format!("json.parse() argument must be a string, got {}", args[0].type_name())),
    };

    let value = match serde_json::from_str::<serde_json::Value>(&json_str) {
        Ok(val) => val,
        Err(e) => return Err(format!("Failed to parse JSON: {}", e)),
    };

    Ok(parse_obj(value)?)
}