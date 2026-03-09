use std::collections::HashMap;
use chrono::{DateTime, Local, TimeZone};
use crate::vm::value::{BuiltinFn, Value};

pub fn object() -> Value {
    let mut methods = HashMap::new();

    methods.insert("now_unix".to_string(), Value::BuiltinFn(BuiltinFn::new("now_unix", now_unix)));
    methods.insert("now_unix_ms".to_string(), Value::BuiltinFn(BuiltinFn::new("now_unix_ms", now_unix_ms)));
    methods.insert("now_utc".to_string(), Value::BuiltinFn(BuiltinFn::new("now_utc", now_utc)));
    methods.insert("now_local".to_string(), Value::BuiltinFn(BuiltinFn::new("now_local", now_local)));
    methods.insert("to_iso_8601".to_string(), Value::BuiltinFn(BuiltinFn::new("to_iso_8601", to_iso_8601)));
    methods.insert("to_rfc_2822".to_string(), Value::BuiltinFn(BuiltinFn::new("to_rfc_2822", to_rfc_2822)));
    methods.insert("to_local_date_time".to_string(), Value::BuiltinFn(BuiltinFn::new("to_local_date_time", to_local_date_time)));
    methods.insert("to_utc_date_time".to_string(), Value::BuiltinFn(BuiltinFn::new("to_utc_date_time", to_utc_date_time)));

    Value::Object(methods)
}

pub fn list_fns() -> Vec<String> {
    vec![
        "now_unix".to_string(),
        "now_unix_ms".to_string(),
        "now_utc".to_string(),
        "now_local".to_string(),
        "to_iso_8601".to_string(),
        "to_rfc_2822".to_string(),
        "to_local_date_time".to_string(),
        "to_utc_date_time".to_string(),
    ]
}

fn now_unix(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(format!("time.now_unix() takes no arguments ({} given)", args.len()));
    }

    let unix_time = Local::now().timestamp();

    Ok(Value::Int(unix_time))
}

fn now_unix_ms(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(format!("time.now_unix_ms() takes no arguments ({} given)", args.len()));
    }

    let unix_time_ms = Local::now().timestamp_millis();

    Ok(Value::Int(unix_time_ms))
}

fn now_utc(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(format!("time.now_utc() takes no arguments ({} given)", args.len()));
    }

    let utc_time: DateTime<chrono::Utc> = chrono::Utc::now();

    Ok(Value::String(utc_time.to_rfc3339()))
}

fn now_local(args: Vec<Value>) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(format!("time.now_local() takes no arguments ({} given)", args.len()));
    }

    let local_time: DateTime<Local> = Local::now();

    Ok(Value::String(local_time.to_rfc3339()))
}

fn to_iso_8601(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("time.to_iso_8601() takes exactly one argument ({} given)", args.len()));
    }

    let timestamp = match &args[0] {
        Value::Int(i) => *i,
        _ => return Err(format!("time.to_iso_8601() argument must be an integer timestamp, got {}", args[0].type_name())),
    };

    let dt = DateTime::<Local>::from(Local.timestamp(timestamp, 0));

    Ok(Value::String(dt.to_rfc3339()))
}

fn to_rfc_2822(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("time.to_rfc_2822() takes exactly one argument ({} given)", args.len()));
    }

    let timestamp = match &args[0] {
        Value::Int(i) => *i,
        _ => return Err(format!("time.to_rfc_2822() argument must be an integer timestamp, got {}", args[0].type_name())),
    };

    let dt = DateTime::<Local>::from(Local.timestamp(timestamp, 0));

    Ok(Value::String(dt.to_rfc2822()))
}

fn to_local_date_time(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("time.to_local_date_time() takes exactly one argument ({} given)", args.len()));
    }

    let timestamp = match &args[0] {
        Value::Int(i) => *i,
        _ => return Err(format!("time.to_local_date_time() argument must be an integer timestamp, got {}", args[0].type_name())),
    };

    let dt = DateTime::<Local>::from(Local.timestamp(timestamp, 0));

    Ok(Value::String(dt.to_string()))
}

fn to_utc_date_time(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("time.to_utc_date_time() takes exactly one argument ({} given)", args.len()));
    }

    let timestamp = match &args[0] {
        Value::Int(i) => *i,
        _ => return Err(format!("time.to_utc_date_time() argument must be an integer timestamp, got {}", args[0].type_name())),
    };

    let dt = DateTime::<chrono::Utc>::from(chrono::Utc.timestamp(timestamp, 0));

    Ok(Value::String(dt.to_string()))
}