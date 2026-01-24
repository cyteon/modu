
use std::collections::HashMap;
use crate::packages::array;

#[derive(Debug, PartialEq, Clone)]
pub enum AST {
    LetDeclaration {
        name: Option<String>,
        value: Box<AST>,
        line: usize, // for error msgs
    },

    IfStatement {
        condition: Box<AST>,
        body: Vec<AST>,
        line: usize,
    },

    Import {
        file: Option<String>,
        as_: Option<String>,
        line: usize,
    },

    Object {
        properties: HashMap<String, AST>,
        line: usize,
    },

    PropertyAccess {
        object: Option<String>,
        property: Option<String>,
        line: usize,
    },

    PropertyCall {
        object: Option<String>,
        property: Option<String>,
        args: Vec<AST>,
        line: usize,
    },

    Call {
        name: String,
        args: Vec<AST>,
        line: usize,
    },

    Function {
        name: String,
        args: Vec<String>,
        body: Vec<AST>,
        line: usize,
    },

    Return {
        value: Box<AST>,
        line: usize,
    },

    InternalFunction {
        name: String,
        args: Vec<String>,
        call_fn: fn(Vec<AST>, &mut HashMap<String, AST>) -> Result<(AST, AST), String>,
    },

    Exists {
        value: Box<AST>,
        line: usize,
    },

    IsEqual {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    LessThan {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    GreaterThan {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    LessThanOrEqual {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    GreaterThanOrEqual {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    IsUnequal {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    Addition {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    Subtraction {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
    },

    Identifer(String),
    Number(i64),
    String(String),
    Boolean(bool),
    Float(f64),
    Null,
    Semicolon,
    Lparen,
    Rparen,
    RBracket,
    Comma,
    Dot,
    Minus,
    Plus,
}


impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // TODO: Implement more
            AST::String(s) => { 
                let mut s = s.replace("\\t", "\t")
                    .replace("\\n", "\n")
                    .replace("\\r", "\r")
                    .replace("\\\"", "\"")
                    .replace("\\\\", "\\");
            
                if s.starts_with("\"") && s.ends_with("\"") {
                    s = s[1..s.len() - 1].to_string();
                } else if s.starts_with("'") && s.ends_with("'") {
                    s = s[1..s.len() - 1].to_string();
                }

                write!(f, "{}", s)
            },

            AST::Number(n) => write!(f, "{}", n),
            AST::Float(n) => write!(f, "{}", n),
            AST::Boolean(b) => write!(f, "{}", b),
            AST::Null => write!(f, "null"),

            AST::Object { properties, line: _ } => {
                if properties.contains_key(array::IDENTITY) && properties[array::IDENTITY].clone() == AST::String("array".to_string()) {
                    write!(f, "[")?;

                    let mut str = String::new();
                    

                    for i in 0..properties.len() {
                        if properties.contains_key(&format!("{}", i)) {
                            str.push_str(&format!("{}, ", properties[&format!("{}", i)]));
                        }
                    }

                    if str.len() > 0 {
                        write!(f, "{}", &str[..str.len() - 2])?;
                    }

                    write!(f, "]")?;

                    return Ok(());
                }

                write!(f, "{{ ")?;

                if properties.len() as i32 - crate::packages::json::BUILTINS.len() as i32 == 0 {
                    write!(f, "}}")?;
                } else {
                    let mut str = String::new();

                    for (key, value) in properties {
                        if crate::packages::json::BUILTINS.contains(&key.as_str()) {
                            continue;
                        }

                        match value {
                            AST::String(s) => {
                                str.push_str(&format!("\"{}\": \"{}\", ", key, s.replace("\"", "\\\"").replace("\n", "\\n").replace("\t", "\\t")));
                            }

                            _ => {
                                str.push_str(&format!("\"{}\": {}, ", key, value));
                            }
                        }
                    }

                    if str.len() > 0 {
                        write!(f, "{}", &str[..str.len() - 2])?;
                    }


                    write!(f, " }}")?;
                }
                
                Ok(())
            }

            _ => write!(f, "{:?}", self),
        }
    }
}