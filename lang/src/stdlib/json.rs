use std::collections::HashMap;
use crate::vm::value::{BuiltinFn, Value};

pub fn object() -> Value {
    let mut methods = HashMap::new();

    methods.insert("parse".to_string(), Value::BuiltinFn(BuiltinFn::new("parse", parse)));

    Value::Object(methods)
}

fn parse_obj(obj: &mut HashMap<String, serde_json::Value>) -> Result<HashMap<String, Value>, String> {
    let mut map = HashMap::new();

    for (k, v) in obj.iter_mut() {
        let value = match v {
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
            
            serde_json::Value::Array(arr) => {
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
                            let mut nested_obj = HashMap::new();
                            nested_obj.insert("value".to_string(), item.clone());
                            let properties = parse_obj(&mut nested_obj)?;
                            properties.get("value").cloned().unwrap_or(Value::Null)
                        }
                    };
                    vec.push(item_value);
                }
                Value::Array(vec)
            }

            serde_json::Value::Object(obj) => {
                let mut hashmap: HashMap<String, serde_json::Value> = obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                let properties = parse_obj(&mut hashmap)?;

                Value::Object(properties)
            }
        };

        map.insert(k.clone(), value);
    } 

    Ok(map)
}

pub fn parse(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("json.parse() takes exactly one argument ({} given)", args.len()));
    }

    let json_str = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("json.parse() argument must be a string, got {}", args[0].type_name())),
    };

    let mut parsed = match serde_json::from_str::<HashMap<String, serde_json::Value>>(json_str) {
        Ok(val) => val,
        Err(e) => return Err(format!("Failed to parse JSON: {}", e)),
    };

    let properties = parse_obj(&mut parsed)?;
    Ok(Value::Object(properties))
}