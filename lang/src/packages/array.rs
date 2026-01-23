use std::collections::HashMap;
use crate::ast::AST;
use crate::eval::eval;

pub static IDENTITY: &str = "\x1b \x1b"; // name of the property used to indentify arrays

pub fn new(_: Vec<AST>, _: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let mut obj: HashMap<String, AST> = HashMap::new();

    obj.insert(
        IDENTITY.to_string(),
        AST::String("array".to_string())
    ); // this is how you should identify arrays

    obj.insert(
        "length".to_string(),
        AST::Number(0)
    );

    obj.insert(
        "at".to_string(),
        AST::InternalFunction {
            name: "at".to_string(),
            args: vec!["self".to_string(), "index".to_string()],
            call_fn: at
        }
    );
    obj.insert(
        "push".to_string(),
        AST::InternalFunction {
            name: "push".to_string(),
            args: vec!["self".to_string(), "item".to_string()],
            call_fn: push
        }
    );
    obj.insert(
        "pop".to_string(),
        AST::InternalFunction {
            name: "pop".to_string(),
            args: vec!["self".to_string()],
            call_fn: pop
        }
    );
    obj.insert(
        "shift".to_string(),
        AST::InternalFunction {
            name: "shift".to_string(),
            args: vec!["self".to_string()],
            call_fn: shift
        }
    );
    obj.insert(
        "unshift".to_string(),
        AST::InternalFunction {
            name: "unshift".to_string(),
            args: vec!["self".to_string(), "item".to_string()],
            call_fn: unshift
        }
    );

    Ok((AST::Object { properties: obj, line: 0 }, AST::Null))
}

// tests if an object is an array
fn check_array(obj: HashMap<String, AST>) -> bool {
    let prop = obj.get(&IDENTITY.to_string());
    if let Some(val) = prop {
        if let AST::String(valstr) = val {
            return *valstr == "array".to_string();
        }
        return false;
    }
    false
}

pub fn isarray(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let arr = eval(args[0].clone(), context)?;

    if let AST::Object { properties: obj, line: _ } = arr {
        return Ok((AST::Boolean(check_array(obj.clone())), AST::Null));
    }
    Ok((AST::Boolean(false), AST::Null))
}

// Self-functions

pub fn at(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let arr = eval(args[0].clone(), context)?;
    let index = eval(args[1].clone(), context)?;
    
    match (arr, index) {
        (AST::Object { properties: obj, line: _ }, AST::Number(i)) => {
            if !check_array(obj.clone()) {
                return Err("first argument is not an array".to_string())
            }
            let itemornone = obj.get(&i.to_string());
            if let None = itemornone {
                return Err("no such element at that index".to_string());
            }
            let item = itemornone.unwrap();
            Ok((item.clone(), AST::Null))
        },

        _ => Err("at() expects an array and a number".to_string())
    }
}

pub fn push(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let arr = eval(args[0].clone(), context)?;
    let item = eval(args[1].clone(), context)?;

    match arr {
        AST::Object { properties: mut obj, line: _ } => {
            let objclone = obj.clone();
            let lenornone = objclone.get(&"length".to_string());

            if let None = lenornone {
                return Err("corrupted array".to_string());
            }

            let len = lenornone.unwrap();

            match len {
                AST::Number(length) => {
                    obj.insert(
                        length.to_string(),
                        item
                    );

                    obj.insert("length".to_string(), AST::Number(length+1));

                    Ok((AST::Null, AST::Object { properties: obj, line: 0 }))
                }

                _ => Err("corrupted array".to_string())
            }
        }

        _ => Err("push() expects an array and a value".to_string())
    }
}

pub fn pop(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let arr = eval(args[0].clone(), context)?;

    match arr {
        AST::Object { properties: mut obj, line: _ } => {
            let objclone = obj.clone();
            if !check_array(obj.clone()) {
                return Err("first argument is not an array".to_string())
            }
            let lenornone = objclone.get(&"length".to_string());

            if let None = lenornone {
                return Err("corrupted array".to_string());
            }

            let len = lenornone.unwrap();

            match len {
                AST::Number(length) => {
                    if *length < 1 {
                        return Err("empty array".to_string());
                    }

                    let lastornone = objclone.get(&(length-1).to_string());

                    if let None = lastornone {
                        return Err("corrupted array".to_string());
                    }

                    let last = lastornone.unwrap();

                    obj.remove(&(length-1).to_string());
                    obj.insert("length".to_string(), AST::Number(length-1));

                    Ok((last.clone(), AST::Object { properties: obj, line: 0 }))
                }
                
                _ => Err("corrupted array".to_string())
            }
        }

        _ => Err("pop() expects an array".to_string())
    }
}

pub fn shift(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let arr = eval(args[0].clone(), context)?;

    match arr {
        AST::Object { properties: mut obj, line: _ } => {
            let objclone = obj.clone();
            if !check_array(obj.clone()) {
                return Err("first argument is not an array".to_string())
            }
            let lenornone = objclone.get(&"length".to_string());

            if let None = lenornone {
                return Err("corrupted array".to_string());
            }

            let len = lenornone.unwrap();

            match len {
                AST::Number(length) => {
                    if *length < 1 {
                        return Err("empty array".to_string());
                    }

                    let firstornone = objclone.get(&0.to_string());

                    if let None = firstornone {
                        return Err("corrupted array".to_string());
                    }

                    let first = firstornone.unwrap();

                    let mut elems: Vec<AST> = vec![];

                    if *length > 1 {
                        for i in 1..*length {
                            let elemornone = objclone.get(&i.to_string());

                            if let None = elemornone {
                                return Err("corrupted array".to_string());
                            }

                            let elem = elemornone.unwrap();

                            elems.push(elem.clone());
                        }
                    }

                    obj.clear();

                    if *length > 1 {
                        let mut i: i64 = 0;
                        for elem in elems {
                            obj.insert(i.to_string(), elem);
                            i += 1;
                        }
                    }

                    let default_obj = new(vec![], context).unwrap().0;

                    match default_obj {
                        AST::Object { properties: default_obj, line: _ } => {
                            for (key, value) in default_obj {
                                obj.insert(key, value);
                            }
                        }
                        _ => {}
                    }

                    obj.insert("length".to_string(), AST::Number(length-1));

                    Ok((first.clone(), AST::Object { properties: obj, line: 0 }))
                }

                _ => Err("corrupted array".to_string())
            }
        }

        _ => Err("shift() expects an array".to_string())
    }
}

pub fn unshift(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let arr = eval(args[0].clone(), context)?;
    let item = eval(args[1].clone(), context)?;

    match arr {
        AST::Object { properties: mut obj, line: _ } => {
            let objclone = obj.clone();
            if !check_array(obj.clone()) {
                return Err("first argument is not an array".to_string())
            }

            let lenornone = objclone.get(&"length".to_string());

            if let None = lenornone {
                return Err("corrupted array".to_string());
            }

            let len = lenornone.unwrap();

            match len {
                AST::Number(length) => {
                    let mut elems: Vec<AST> = vec![];

                    if *length > 0 {
                        for i in 0..*length {
                            let elemornone = objclone.get(&i.to_string());

                            if let None = elemornone {
                                return Err("corrupted array".to_string());
                            }

                            let elem = elemornone.unwrap();

                            elems.push(elem.clone());
                        }
                    }

                    obj.clear();

                    obj.insert(0.to_string(), item);

                    if *length > 0 {
                        let mut i: i64 = 1;
                        for elem in elems {
                            obj.insert(i.to_string(), elem);
                            i += 1;
                        }
                    }

                    let default_obj = new(vec![], context).unwrap().0;

                    match default_obj {
                        AST::Object { properties: default_obj, line: _ } => {
                            for (key, value) in default_obj {
                                obj.insert(key, value);
                            }
                        }
                        _ => {}
                    }

                    obj.insert("length".to_string(), AST::Number(length+1));

                    Ok((AST::Null, AST::Object { properties: obj, line: 0 }))
                }

                _ => Err("corrupted array".to_string())
            }
        }

        _ => Err("unshift() expects an array".to_string())
    }
}

pub fn get_object() -> HashMap<String, AST> {
    let mut object = HashMap::new();

    object.insert(
        "new".to_string(),
        AST::InternalFunction {
            name: "new".to_string(),
            args: vec![],
            call_fn: new
        }
    );

    object.insert(
        "isarray".to_string(),
        AST::InternalFunction {
            name: "isarray".to_string(),
            args: vec!["arr".to_string()],
            call_fn: isarray
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