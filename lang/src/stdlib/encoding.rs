use std::collections::HashMap;
use base64::prelude::*;
use crate::vm::value::{BuiltinFn, Value};

pub fn object() -> Value {
    let mut methods = HashMap::new();

    methods.insert("encode_base64".to_string(), Value::BuiltinFn(BuiltinFn::new("encode_base64", encode_base64)));
    methods.insert("decode_base64".to_string(), Value::BuiltinFn(BuiltinFn::new("decode_base64", decode_base64)));
    methods.insert("encode_base16".to_string(), Value::BuiltinFn(BuiltinFn::new("encode_base16", encode_base16)));
    methods.insert("decode_base16".to_string(), Value::BuiltinFn(BuiltinFn::new("decode_base16", decode_base16)));

    Value::Object(methods)
}

fn encode_base64(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("encoding.encode_base64() takes exactly one argument ({} given)", args.len()));
    }

    let input = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("encoding.encode_base64() argument must be a string, got {}", args[0].type_name())),
    };

    Ok(Value::String(BASE64_STANDARD.encode(input.as_bytes())))
}

fn decode_base64(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("encoding.decode_base64() takes exactly one argument ({} given)", args.len()));
    }

    let input = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("encoding.decode_base64() argument must be a string, got {}", args[0].type_name())),
    };

    match BASE64_STANDARD.decode(input.as_bytes()) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => Ok(Value::String(s)),
            Err(_) => Err("encoding.decode_base64() decoded bytes are not valid UTF-8".to_string()),
        },
        Err(_) => Err("encoding.decode_base64() invalid base64 string".to_string()),
    }
}

fn encode_base16(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("encoding.encode_base16() takes exactly one argument ({} given)", args.len()));
    }

    let input = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("encoding.encode_base16() argument must be a string, got {}", args[0].type_name())),
    };

    Ok(Value::String(base16::encode_lower(input.as_bytes())))
}

fn decode_base16(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("encoding.decode_base16() takes exactly one argument ({} given)", args.len()));
    }

    let input = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("encoding.decode_base16() argument must be a string, got {}", args[0].type_name())),
    };

    match base16::decode(input) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => Ok(Value::String(s)),
            Err(_) => Err("encoding.decode_base16() decoded bytes are not valid UTF-8".to_string()),
        },
        Err(_) => Err("encoding.decode_base16() invalid base16 string".to_string()),
    }
}