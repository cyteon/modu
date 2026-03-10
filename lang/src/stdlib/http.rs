use std::collections::HashMap;
use crate::vm::value::{BuiltinFn, Value};

pub fn object() -> Value {
    let mut methods = HashMap::new();

    methods.insert("get".to_string(), Value::BuiltinFn(BuiltinFn::new("get", get)));
    methods.insert("post".to_string(), Value::BuiltinFn(BuiltinFn::new("post", post)));
    methods.insert("put".to_string(), Value::BuiltinFn(BuiltinFn::new("put", put)));
    methods.insert("patch".to_string(), Value::BuiltinFn(BuiltinFn::new("patch", patch)));
    methods.insert("delete".to_string(), Value::BuiltinFn(BuiltinFn::new("delete", delete)));

    Value::Object(methods)
}

fn create_response_obj(response: reqwest::blocking::Response) -> Value {
    let mut properties = HashMap::new();

    properties.insert("status".to_string(), Value::Int(response.status().as_u16() as i64));
    properties.insert("status_text".to_string(), Value::String(response.status().canonical_reason().unwrap_or("").to_string()));
    properties.insert("ok".to_string(), Value::Bool(response.status().is_success()));

    let headers = response.headers().iter().map(|(k, v)| {
        let value_str = match v.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => format!("{:?}", v),
        };
        (k.to_string(), Value::String(value_str))
    }).collect();
    properties.insert("headers".to_string(), Value::Object(headers));

    let body = response.text().unwrap_or_default();
    properties.insert("body".to_string(), Value::String(body));

    Value::Object(properties)
}

fn get(args: Vec<Value>) -> Result<Value, String> {
    if args.len() == 0 {
        return Err(format!("http.get() needs at least one argument ({} given)", args.len()));
    } else if args.len() > 2 {
        return Err(format!("http.get() takes at most two arguments ({} given)", args.len()));
    }

    let url = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("http.get() first argument must be a string, got {}", args[0].type_name())),
    };

    let client = reqwest::blocking::Client::new();
    let mut request = client.get(url);

    if args.len() == 2 {
        let headers = match &args[1] {
            Value::Object(obj) => obj,
            _ => return Err(format!("http.get() second argument must be an object, got {}", args[1].type_name())),
        };

        for (key, value) in headers {
            let value_str = match value {
                Value::String(s) => s,
                _ => return Err(format!("http.get() header values must be strings, got {}", value.type_name())),
            };

            request = request.header(key, value_str);
        }
    }

    let response = match request.send() {
        Ok(resp) => resp,
        Err(e) => return Err(format!("http.get() request failed: {}", e)),
    };

    Ok(create_response_obj(response))
}

fn post(args: Vec<Value>) -> Result<Value, String> {
    if args.len() == 0 {
        return Err(format!("http.post() needs at least one argument ({} given)", args.len()));
    } else if args.len() > 3 {
        return Err(format!("http.post() takes at most three arguments ({} given)", args.len()));
    }

    let url = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("http.post() first argument must be a string, got {}", args[0].type_name())),
    };

    let client = reqwest::blocking::Client::new();
    let mut request = client.post(url);

    if args.len() >= 2 {
        let body = match &args[1] {
            Value::String(s) => s,
            _ => return Err(format!("http.post() second argument must be a string, got {}", args[1].type_name())),
        };

        request = request.body(body.clone());
    }

    if args.len() == 3 {
        let headers = match &args[2] {
            Value::Object(obj) => obj,
            _ => return Err(format!("http.post() third argument must be an object, got {}", args[2].type_name())),
        };

        for (key, value) in headers {
            let value_str = match value {
                Value::String(s) => s,
                _ => return Err(format!("http.post() header values must be strings, got {}", value.type_name())),
            };

            request = request.header(key, value_str);
        }
    }

    let response = match request.send() {
        Ok(resp) => resp,
        Err(e) => return Err(format!("http.post() request failed: {}", e)),
    };

    Ok(create_response_obj(response))
}

fn put(args: Vec<Value>) -> Result<Value, String> {
    if args.len() == 0 {
        return Err(format!("http.put() needs at least one argument ({} given)", args.len()));
    } else if args.len() > 3 {
        return Err(format!("http.put() takes at most three arguments ({} given)", args.len()));
    }

    let url = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("http.put() first argument must be a string, got {}", args[0].type_name())),
    };

    let client = reqwest::blocking::Client::new();
    let mut request = client.put(url);  

    if args.len() >= 2 {
        let body = match &args[1] {
            Value::String(s) => s,
            _ => return Err(format!("http.put() second argument must be a string, got {}", args[1].type_name())),
        };

        request = request.body(body.clone());
    }

    if args.len() == 3 {
        let headers = match &args[2] {
            Value::Object(obj) => obj,
            _ => return Err(format!("http.put() third argument must be an object, got {}", args[2].type_name())),
        };

        for (key, value) in headers {
            let value_str = match value {
                Value::String(s) => s,
                _ => return Err(format!("http.put() header values must be strings, got {}", value.type_name())),
            };

            request = request.header(key, value_str);
        }
    }

    let response = match request.send() {
        Ok(resp) => resp,
        Err(e) => return Err(format!("http.put() request failed: {}", e)),
    };

    Ok(create_response_obj(response))
}

fn patch(args: Vec<Value>) -> Result<Value, String> {
    if args.len() == 0 {
        return Err(format!("http.patch() needs at least one argument ({} given)", args.len()));
    } else if args.len() > 3 {
        return Err(format!("http.patch() takes at most three arguments ({} given)", args.len()));
    }

    let url = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("http.patch() first argument must be a string, got {}", args[0].type_name())),
    };

    let client = reqwest::blocking::Client::new();
    let mut request = client.patch(url);  

    if args.len() >= 2 {
        let body = match &args[1] {
            Value::String(s) => s,
            _ => return Err(format!("http.patch() second argument must be a string, got {}", args[1].type_name())),
        };

        request = request.body(body.clone());
    }

    if args.len() == 3 {
        let headers = match &args[2] {
            Value::Object(obj) => obj,
            _ => return Err(format!("http.patch() third argument must be an object, got {}", args[2].type_name())),
        };

        for (key, value) in headers {
            let value_str = match value {
                Value::String(s) => s,
                _ => return Err(format!("http.patch() header values must be strings, got {}", value.type_name())),
            };

            request = request.header(key, value_str);
        }
    }

    let response = match request.send() {
        Ok(resp) => resp,
        Err(e) => return Err(format!("http.patch() request failed: {}", e)),
    };

    Ok(create_response_obj(response))
}

fn delete(args: Vec<Value>) -> Result<Value, String> {
    if args.len() == 0 {
        return Err(format!("http.delete() needs at least one argument ({} given)", args.len()));
    } else if args.len() > 2 {
        return Err(format!("http.delete() takes at most two arguments ({} given)", args.len()));
    }

    let url = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("http.delete() first argument must be a string, got {}", args[0].type_name())),
    };

    let client = reqwest::blocking::Client::new();
    let mut request = client.delete(url);

    if args.len() == 2 {
        let headers = match &args[1] {
            Value::Object(obj) => obj,
            _ => return Err(format!("http.delete() second argument must be an object, got {}", args[1].type_name())),
        };

        for (key, value) in headers {
            let value_str = match value {
                Value::String(s) => s,
                _ => return Err(format!("http.delete() header values must be strings, got {}", value.type_name())),
            };

            request = request.header(key, value_str);
        }
    }

    let response = match request.send() {
        Ok(resp) => resp,
        Err(e) => return Err(format!("http.delete() request failed: {}", e)),
    };

    Ok(create_response_obj(response))
}