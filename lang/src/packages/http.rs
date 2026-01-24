use std::collections::HashMap;
use crate::{ast::AST, eval::eval, packages::json};

pub fn get(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let url = eval(args[0].clone(), context)?;

    match url {
        AST::String(val) => {
            let resp = reqwest::blocking::get(&val);

            match resp {
                Ok(r) => {
                    let status = r.status();

                    let mut properties = HashMap::new();
	                properties = json::insert_functions(&mut properties);

                    properties.insert(
                        "status".to_string(),
                        AST::Number(status.as_u16() as i64)
                    );

                    properties.insert(
                        "status_text".to_string(),
                        AST::String(status.canonical_reason().unwrap_or("").to_string())
                    );

                    let headers = AST::Object {
                        properties: r.headers().iter().map(|(k, v)| {
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

                    let body = r.text().map_err(|e| e.to_string())?;

                    properties.insert(
                        "body".to_string(),
                        AST::String(body)
                    );

                    properties.insert(
                        "ok".to_string(),
                        AST::Boolean(status.is_success())
                    );

                    return Ok((AST::Object { properties, line: 0, }, AST::Null));
                },

                Err(e) => return Err(e.to_string()),
            };
        }

        _ => Err("http.get() expects a string".to_string())
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

    object
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_object_test() {
        let object = get_object();

        assert_eq!(object.len(), 1);
    }
}