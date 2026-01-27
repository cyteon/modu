use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::ast::AST;
use crate::eval::eval;

pub fn read(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let path = eval(args[0].clone(), context)?;

    match path {
        AST::String(val) => {
            let contents = std::fs::read_to_string(val).map_err(|e| e.to_string())?;
            Ok((AST::String(contents), AST::Null))
        }

        _ => Err("read() expects a string".to_string())
    }
}

pub fn write(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let path = eval(args[0].clone(), context)?;
    let contents = eval(args[1].clone(), context)?;

    match (path, contents) {
        (AST::String(path), AST::String(contents)) => {
            let contents = contents
                .replace("\\n", "\n")
                .replace("\\t", "\t");

            std::fs::write(path, contents).map_err(|e| e.to_string())?;
            Ok((AST::Null, AST::Null))
        }

        _ => Err("write() expects two strings".to_string())
    }
}

pub fn write_append(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let path = eval(args[0].clone(), context)?;
    let contents = eval(args[1].clone(), context)?;

    match (path, contents) {
        (AST::String(path), AST::String(contents)) => {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(path)
                .map_err(|e| e.to_string())?;

            let contents = contents
                .replace("\\n", "\n")
                .replace("\\t", "\t");

            if let Err(e) = writeln!(file, "{}", contents) {
                return Err(e.to_string());
            }

            Ok((AST::Null, AST::Null))
        }

        _ => Err("write_append() expects two strings".to_string())
    }
}

pub fn get_object() -> HashMap<String, AST> {
    let mut object = HashMap::new();

    object.insert(
        "read".to_string(),
        AST::InternalFunction { 
            name:"read".to_string(), args: vec!["path".to_string()], call_fn: read }
    );

    object.insert(
        "write".to_string(),
        AST::InternalFunction { 
            name:"write".to_string(), args: vec!["path".to_string(), "content".to_string()], call_fn: write }
    );

    object.insert(
        "write_append".to_string(),
        AST::InternalFunction { 
            name:"write_append".to_string(), args: vec!["path".to_string(), "content".to_string()], call_fn: write_append }
    );

    return object;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_object_test() {
        let object = get_object();

        assert_eq!(object.len(), 3);
    }
}
