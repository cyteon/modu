use std::collections::HashMap;
use libloading::Library;
use std::sync::Arc;

use crate::packages::array;

#[derive(Debug, Clone)]
pub enum AST {
    LetDeclaration {
        name: Option<String>,
        value: Box<AST>,
        line: usize,
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

    FFILibrary {
        path: String,
        lib: Arc<Library>,
    },

    Loop {
        body: Vec<AST>,
        line: usize,
    },

    ForLoop {
        start: Box<AST>,
        end: Box<AST>,
        index_name: String,
        body: Vec<AST>,
        line: usize,
    },

    Range {
        left: Box<AST>,
        right: Box<AST>,
        line: usize,
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
    Integer(i64),
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
    Break,
}

impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AST::String(s) => { 
                let mut s = s.replace("\\t", "\t")
                    .replace("\\n", "\n")
                    .replace("\\r", "\r")
                    .replace("\\\"", "\"")
                    .replace("\\\'", "\'")
                    .replace("\\\\", "\\");
                if s.starts_with("\"") && s.ends_with("\"") {
                    s = s[1..s.len() - 1].to_string();
                } else if s.starts_with("'") && s.ends_with("'") {
                    s = s[1..s.len() - 1].to_string();
                }
                write!(f, "{}", s)
            },
            AST::Integer(n) => write!(f, "{}", n),
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

impl PartialEq for AST {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AST::FFILibrary { path: p1, .. }, AST::FFILibrary { path: p2, .. }) => p1 == p2,
            (AST::Integer(i1), AST::Integer(i2)) => i1 == i2,
            (AST::String(s1), AST::String(s2)) => s1 == s2,
            (AST::Boolean(b1), AST::Boolean(b2)) => b1 == b2,
            (AST::Float(f1), AST::Float(f2)) => f1 == f2,
            (AST::Integer(i1), AST::Float(f2)) => (*i1 as f64) == *f2,
            (AST::Float(f1), AST::Integer(i2)) => *f1 == (*i2 as f64),
            _ => std::mem::discriminant(self) == std::mem::discriminant(other),
        }
    }
}
