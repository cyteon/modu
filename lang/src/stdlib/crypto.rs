use argon2::{PasswordHasher, PasswordVerifier};
use sha2::Digest;
use std::collections::HashMap;

use crate::vm::value::{BuiltinFn, Value};

pub fn object() -> Value {
    let mut methods = HashMap::new();

    methods.insert("sha256".to_string(), Value::BuiltinFn(BuiltinFn::new("sha256", sha256)));
    methods.insert("sha512".to_string(), Value::BuiltinFn(BuiltinFn::new("sha512", sha512)));
    methods.insert("blake3".to_string(), Value::BuiltinFn(BuiltinFn::new("blake3", blake3)));
    methods.insert("bcrypt_hash".to_string(), Value::BuiltinFn(BuiltinFn::new("bcrypt_hash", bcrypt_hash)));
    methods.insert("bcrypt_verify".to_string(), Value::BuiltinFn(BuiltinFn::new("bcrypt_verify", bcrypt_verify)));
    methods.insert("argon2_hash".to_string(), Value::BuiltinFn(BuiltinFn::new("argon2_hash", argon2_hash)));
    methods.insert("argon2_verify".to_string(), Value::BuiltinFn(BuiltinFn::new("argon2_verify", argon2_verify)));

    Value::Object(methods)
}

pub fn list_fns() -> Vec<String> {
    vec![
        "sha256".to_string(),
        "sha512".to_string(),
        "blake3".to_string(),
        "bcrypt_hash".to_string(),
        "bcrypt_verify".to_string(),
        "argon2_hash".to_string(),
        "argon2_verify".to_string(),
    ]
}

fn sha256(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("crypto.sha256() takes exactly one argument ({} given)", args.len()));
    }

    let input = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("crypto.sha256() argument must be a string, got {}", args[0].type_name())),
    };

    let mut hasher = sha2::Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    Ok(Value::String(format!("{:x}", result)))
}

fn sha512(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("crypto.sha512() takes exactly one argument ({} given)", args.len()));
    }

    let input = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("crypto.sha512() argument must be a string, got {}", args[0].type_name())),
    };

    let mut hasher = sha2::Sha512::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();

    Ok(Value::String(format!("{:x}", result)))
}

fn blake3(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("crypto.blake3() takes exactly one argument ({} given)", args.len()));
    }

    let input = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("crypto.blake3() argument must be a string, got {}", args[0].type_name())),
    };

    let result = blake3::hash(input.as_bytes());

    Ok(Value::String(result.to_hex().to_string()))
}

fn bcrypt_hash(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("crypto.bcrypt_hash() takes exactly one argument ({} given)", args.len()));
    }

    let input = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("crypto.bcrypt_hash() argument must be a string, got {}", args[0].type_name())),
    };

    match bcrypt::hash(input, bcrypt::DEFAULT_COST) {
        Ok(hash) => Ok(Value::String(hash)),
        Err(e) => Err(format!("crypto.bcrypt_hash() error: {}", e)),
    }
}

fn bcrypt_verify(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("crypto.bcrypt_verify() takes exactly two arguments ({} given)", args.len()));
    }

    let input = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("crypto.bcrypt_verify() first argument must be a string, got {}", args[0].type_name())),
    };

    let hash = match &args[1] {
        Value::String(s) => s,
        _ => return Err(format!("crypto.bcrypt_verify() second argument must be a string, got {}", args[1].type_name())),
    };

    match bcrypt::verify(input, hash) {
        Ok(valid) => Ok(Value::Bool(valid)),
        Err(e) => Err(format!("crypto.bcrypt_verify() error: {}", e)),
    }
}

fn argon2_hash(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("crypto.argon2_hash() takes exactly one argument ({} given)", args.len()));
    }

    let input = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("crypto.argon2_hash() argument must be a string, got {}", args[0].type_name())),
    };

    let salt = argon2::password_hash::SaltString::generate(&mut rand::thread_rng());
    let argon2 = argon2::Argon2::default();

    match argon2.hash_password(input.as_bytes(), &salt) {
        Ok(hash) => Ok(Value::String(hash.to_string())),
        Err(e) => Err(format!("crypto.argon2_hash() error: {}", e)),
    }
}

fn argon2_verify(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("crypto.argon2_verify() takes exactly two arguments ({} given)", args.len()));
    }

    let input = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("crypto.argon2_verify() first argument must be a string, got {}", args[0].type_name())),
    };

    let hash = match &args[1] {
        Value::String(s) => s,
        _ => return Err(format!("crypto.argon2_verify() second argument must be a string, got {}", args[1].type_name())),
    };

    let parsed_hash = match argon2::PasswordHash::new(hash) {
        Ok(h) => h,
        Err(e) => return Err(format!("crypto.argon2_verify() error parsing hash: {}", e)),
    };

    let argon2 = argon2::Argon2::default();

    match argon2.verify_password(input.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(Value::Bool(true)),
        Err(argon2::password_hash::Error::Password) => Ok(Value::Bool(false)),
        Err(e) => Err(format!("crypto.argon2_verify() error: {}", e)),
    }
}