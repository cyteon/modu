use crate::vm::value::{BuiltinFn, Value};

fn builtin(name: &str, func: fn(Vec<Value>) -> Result<(Value, Value), String>) -> BuiltinFn {
    BuiltinFn { name: name.to_string(), func }
}

pub fn get_functions() -> Vec<BuiltinFn> {
    vec![
        
    ]
}