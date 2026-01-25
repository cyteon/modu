use std::collections::HashMap;

use crate::ast::AST;
use crate::eval::eval;

pub fn insert_functions(properties: &mut HashMap<String, AST>) -> HashMap<String, AST> {
	properties.insert(
		"set".to_string(),
		AST::InternalFunction {
			name: "set".to_string(),
			args: vec!["self".to_string(), "key".to_string(), "value".to_string()],
			call_fn: set,
		}
	);

	properties.insert(
		"get".to_string(),
		AST::InternalFunction {
			name: "get".to_string(),
			args: vec!["self".to_string(), "key".to_string()],
			call_fn: get,
		}
	);

	properties.insert(
		"has".to_string(),
		AST::InternalFunction {
			name: "has".to_string(),
			args: vec!["self".to_string(), "key".to_string()],
			call_fn: has,
		}
	);

	properties.insert(
		"delete".to_string(),
		AST::InternalFunction {
			name: "delete".to_string(),
			args: vec!["self".to_string(), "key".to_string()],
			call_fn: delete,
		}
	);

	properties.clone()
}

pub fn new(_: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
	let mut properties = HashMap::new();
	properties = insert_functions(&mut properties);

    Ok((AST::Object {
		properties,
		line: 0,
	}, AST::Null))
}

fn make_stuff_string(value: AST) -> String {
	match value {
		AST::String(string) => format!("\"{}\"", string),

		AST::Object { properties, .. } => {
			let mut string = "{".to_string();

			for (key, value) in properties {
				if BUILTINS.contains(&&key[..]) {
					continue;
				}

				string.push_str(&format!("\"{}\":{},", key, make_stuff_string(value)));
			}

			if string.len() > 1 {
				string.pop();
			}

			string.push_str("}");

			string
		}

		_ => value.to_string(),
	}
}

pub fn stringify(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
	let value = eval(args[0].clone(), context)?;

	match value {
		AST::Object { properties, .. } => {
			let mut string = "{".to_string();

			for (key, value) in properties {
				if BUILTINS.contains(&&key[..]) {
					continue;
				}

				string.push_str(&format!("\"{}\":{},", key, make_stuff_string(value)));
			}

			if string.len() > 1 {
				string.pop();
			}

			string.push_str("}");

			Ok((AST::String(string), AST::Null))
		},

		_ => Err("json.stringify argument must be an object".to_string()),
	}
}

fn parse_obj(obj: &mut HashMap<String, serde_json::Value>) -> HashMap<String, AST> {
	let mut map = HashMap::new();

	for (key, value) in obj.iter_mut() {
		match value {
			serde_json::Value::String(string) => {
				map.insert(key.clone(), AST::String(string.clone()));
			}

			serde_json::Value::Number(number) => {
				if number.as_i64().is_none() {
					map.insert(key.clone(), AST::Float(number.as_f64().unwrap()));
				} else {
					map.insert(key.clone(), AST::Integer(number.as_i64().unwrap()));
				}
			}

			serde_json::Value::Bool(boolean) => {
				map.insert(key.clone(), AST::Boolean(*boolean));
			}

			serde_json::Value::Null => {
				map.insert(key.clone(), AST::Null);
			}

			serde_json::Value::Object(obj) => {
				let mut hashmap: HashMap<String, serde_json::Value> = obj
					.into_iter()
					.map(|(k, v)| (k.clone(), v.clone()))
					.collect();
				
				let parsed = parse_obj(&mut hashmap);

				map.insert(key.clone(), AST::Object {
					properties: parsed,
					line: 0,
				});
			}

			_ => {}
		}
	}

	map
}

pub fn parse(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
	if args.len() != 1 {
		return Err("json.parse requires exactly one argument".to_string());
	}

	let value = match &args[0] {
		AST::String(string) => AST::String(string.clone()),

		_ => match eval(args[0].clone(), context) {
			Ok(AST::String(string)) => AST::String(string),
			Ok(_) => return Err("json.parse argument must be a string".to_string()),
			Err(e) => return Err(e),
		}
	};

	match value {
		AST::String(string) => {			
			let mut json: HashMap<String, serde_json::Value> = serde_json::from_str(&string).unwrap();

			let mut properties = parse_obj(&mut json);
			properties = insert_functions(&mut properties);

			Ok((AST::Object {
				properties,
				line: 0,
			}, AST::Null))
		}

		_ => Err("json.parse argument must be a string".to_string()),
	}
}

// Self-functions

pub static BUILTINS: [&str; 4] = ["set", "get", "has", "delete"];

pub fn set(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
	if args.len() != 3 {
		return Err("json.set requires exactly two arguments".to_string());
	}

	let key = match eval(args[1].clone(), context) {
		Ok(AST::String(value)) => value,
		Ok(_) => return Err("json.set second argument must be a string".to_string()),
		Err(e) => return Err(e),
	};

	let value = eval(args[2].clone(), context)?;

	let mut properties = match &args[0] {
		AST::Object { properties, .. } => properties.clone(),
		_ => return Err("uh oh, why is self not an object? this is a bug, please report it".to_string()),
	};

	properties.insert(key, value);

	Ok((AST::Null, AST::Object {
		properties,
		line: 0,
	}))
}

pub fn get(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
	if args.len() != 2 {
		return Err("json.get requires exactly two arguments".to_string());
	}

	let key = match eval(args[1].clone(), context) {
		Ok(AST::String(value)) => value,
		Ok(_) => return Err("json.get second argument must be a string".to_string()),
		Err(e) => return Err(e),
	};

	let properties = match &args[0] {
		AST::Object { properties, .. } => properties.clone(),
		_ => return Err("uh oh, why is self not an object? this is a bug, please report it".to_string()),
	};

	let value = match properties.get(&key) {
		Some(value) => value.clone(),
		None => AST::Null,
	};

	Ok((value, AST::Object {
		properties,
		line: 0,
	}))
}

pub fn has(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
	if args.len() != 2 {
		return Err("json.get requires exactly two arguments".to_string());
	}

	let key = match eval(args[1].clone(), context) {
		Ok(AST::String(value)) => value,
		Ok(_) => return Err("json.get second argument must be a string".to_string()),
		Err(e) => return Err(e),
	};

	let properties = match &args[0] {
		AST::Object { properties, .. } => properties.clone(),
		_ => return Err("uh oh, why is self not an object? this is a bug, please report it".to_string()),
	};

	let value = match properties.get(&key) {
		Some(_) => AST::Boolean(true),
		None => AST::Boolean(false),
	};

	Ok((value, AST::Object {
		properties,
		line: 0,
	}))
}

pub fn delete(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
	if args.len() != 2 {
		return Err("json.delete requires exactly two arguments".to_string());
	}

	let key = match eval(args[1].clone(), context) {
		Ok(AST::String(value)) => value,
		Ok(_) => return Err("json.delete second argument must be a string".to_string()),
		Err(e) => return Err(e),
	};

	let mut properties = match &args[0] {
		AST::Object { properties, .. } => properties.clone(),
		_ => return Err("uh oh, why is self not an object? this is a bug, please report it".to_string()),
	};

	properties.remove(&key);

	Ok((AST::Null, AST::Object {
		properties,
		line: 0,
	}))
}

pub fn get_object() -> HashMap<String, AST> {
	let mut object = HashMap::new();

	object.insert(
		"new".to_string(), 
		AST::InternalFunction {
			name: "new".to_string(),
			args: vec![],
			call_fn: new,
		}
	);

	object.insert(
		"stringify".to_string(),
		AST::InternalFunction {
			name: "stringify".to_string(),
			args: vec!["object".to_string()],
			call_fn: stringify,
		}
	);

	object.insert(
		"parse".to_string(),
		AST::InternalFunction {
			name: "parse".to_string(),
			args: vec!["string".to_string()],
			call_fn: parse,
		}
	);


	object
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_stringify() {
		let mut context = HashMap::new();

		let object = AST::Object {
			properties: vec![
				("key".to_string(), AST::String("value".to_string())),
				("key2".to_string(), AST::Integer(1)),
			].iter().cloned().collect(),
			line: 0,
		};

		let result = stringify(vec![object], &mut context).unwrap();

		match result.0 {
			AST::String(string) => {
				let mut equals = string == "{\"key\":\"value\",\"key2\":1}";

				if !equals {
					equals = string == "{\"key2\":1,\"key\":\"value\"}";			
				}

				assert_eq!(equals, true);
			}

			_ => panic!("json.stringify did not return a string"),
		}
	}

	#[test]
	fn test_parse() {
		let mut context = HashMap::new();

		let string = AST::String("{\"key\":\"value\",\"key2\":1}".to_string());

		let result = parse(vec![string], &mut context).unwrap();

		match result.0 {
			AST::Object { properties, .. } => {
				assert_eq!(properties.len(), 2 + BUILTINS.len());
				assert_eq!(properties.contains_key("key"), true);
				assert_eq!(properties.contains_key("key2"), true);
			}

			_ => panic!("json.parse did not return an object"),
		}
	}
}