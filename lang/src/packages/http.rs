use std::collections::HashMap;
use crate::{ast::AST, eval::eval, packages::json};

fn handle_response(response: reqwest::blocking::Response) -> Result<(AST, AST), String> {
    let status = response.status();

    let mut properties = HashMap::new();
    properties = json::insert_functions(&mut properties);

    properties.insert(
        "status".to_string(),
        AST::Integer(status.as_u16() as i64)
    );

    properties.insert(
        "status_text".to_string(),
        AST::String(status.canonical_reason().unwrap_or("").to_string())
    );

    let headers = AST::Object {
        properties: response.headers().iter().map(|(k, v)| {
            (
                k.to_string(),
                AST::String(v.to_str().unwrap_or("").to_string())
            )
        }).collect(),
        line: 0,
    };

    properties.insert(
        "headers".to_string(),
        headers
    );

    let body = response.text().map_err(|e| e.to_string())?;

    properties.insert(
        "body".to_string(),
        AST::String(body)
    );

    properties.insert(
        "ok".to_string(),
        AST::Boolean(status.is_success())
    );

    Ok((AST::Object { properties, line: 0, }, AST::Null))
}

pub fn get(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let url = eval(args[0].clone(), context)?;

    match url {
        AST::String(val) => {
            let resp = reqwest::blocking::get(&val);

            match resp {
                Ok(r) => {
                    return handle_response(r);
                },

                Err(e) => return Err(e.to_string()),
            };
        }

        _ => Err("http.get() expects a string".to_string())
    }
}

pub fn post(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    if args.len() < 1 {
        return Err("http.post() expects at least 1 argument".to_string());
    }

    let url = eval(args[0].clone(), context)?;

    let body = if args.len() >= 2 {
        match eval(args[1].clone(), context)? {
            AST::String(s) => s,
            _ => "".to_string()
        }
    } else {
        "".to_string()
    };

    match url {
        AST::String(val) => {
            let resp = reqwest::blocking::Client::new()
                .post(&val)
                .body(body)
                .send();

            match resp {
                Ok(r) => {
                    return handle_response(r);
                },

                Err(e) => return Err(e.to_string()),
            };
        }
        
        _ => Err("http.post() expects a string".to_string())
    }
}

pub fn put(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    if args.len() < 1 {
        return Err("http.put() expects at least 1 argument".to_string());
    }

    let url = eval(args[0].clone(), context)?;

    let body = if args.len() >= 2 {
        match eval(args[1].clone(), context)? {
            AST::String(s) => s,
            _ => "".to_string()
        }
    } else {
        "".to_string()
    };

    match url {
        AST::String(val) => {
            let resp = reqwest::blocking::Client::new()
                .put(&val)
                .body(body)
                .send();

            match resp {
                Ok(r) => {
                    return handle_response(r);
                },

                Err(e) => return Err(e.to_string()),
            };
        }

        _ => Err("http.get() expects a string".to_string())
    }
}

pub fn patch(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    if args.len() < 1 {
        return Err("http.patch() expects at least 1 argument".to_string());
    }

    let url = eval(args[0].clone(), context)?;

    let body = if args.len() >= 2 {
        match eval(args[1].clone(), context)? {
            AST::String(s) => s,
            _ => "".to_string()
        }
    } else {
        "".to_string()
    };

    match url {
        AST::String(val) => {
            let resp = reqwest::blocking::Client::new()
                .patch(&val)
                .body(body)
                .send();

            match resp {
                Ok(r) => {
                    return handle_response(r);
                },

                Err(e) => return Err(e.to_string()),
            };
        }

        _ => Err("http.patch() expects a string".to_string())
    }
}

pub fn delete(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    if args.len() < 1 {
        return Err("http.delete() expects at least 1 argument".to_string());
    }

    let url = eval(args[0].clone(), context)?;

    let body = if args.len() >= 2 {
        match eval(args[1].clone(), context)? {
            AST::String(s) => s,
            _ => "".to_string()
        }
    } else {
        "".to_string()
    };

    match url {
        AST::String(val) => {
            let resp = reqwest::blocking::Client::new()
                .delete(&val)
                .body(body)
                .send();

            match resp {
                Ok(r) => {
                    return handle_response(r);
                },

                Err(e) => return Err(e.to_string()),
            };
        }

        _ => Err("http.delete() expects a string".to_string())
    }
}

pub fn get_object() -> HashMap<String, AST> {
    let mut object = HashMap::new();

    object.insert(
        "get".to_string(),
        AST::InternalFunction {
            name: "get".to_string(),
            args: vec!["url".to_string()],
            call_fn: get
        }
    );

    object.insert(
        "post".to_string(),
        AST::InternalFunction {
            name: "post".to_string(),
            args: vec!["__args__".to_string()],
            call_fn: post
        }
    );

    object.insert(
        "put".to_string(),
        AST::InternalFunction {
            name: "put".to_string(),
            args: vec!["__args__".to_string()],
            call_fn: put
        }
    );

    object.insert(
        "patch".to_string(),
        AST::InternalFunction {
            name: "patch".to_string(),
            args: vec!["__args__".to_string()],
            call_fn: patch
        }
    );
    
    object.insert(
        "delete".to_string(),
        AST::InternalFunction {
            name: "delete".to_string(),
            args: vec!["__args__".to_string()],
            call_fn: delete
        }
    );

    object
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_object_test() {
        let object = get_object();

        assert_eq!(object.len(), 2);
    }
}