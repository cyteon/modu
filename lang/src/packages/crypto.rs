use std::collections::HashMap;
use argon2::{PasswordHasher, PasswordVerifier};
use sha2::Digest;

use crate::ast::AST;
use crate::eval::eval;

fn sha256(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let input = match eval(args[0].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("crypto.sha256() expects a string argument".to_string()),
    };

    let hashed = sha2::Sha256::digest(input.as_bytes());
    let hashed = format!("{:x}", hashed);

    Ok((AST::String(hashed), AST::Null))
}

fn sha512(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let input = match eval(args[0].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("crypto.sha512() expects a string argument".to_string()),
    };

    let hashed = sha2::Sha512::digest(input.as_bytes());
    let hashed = format!("{:x}", hashed);

    Ok((AST::String(hashed), AST::Null))
}

fn blake3(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let input = match eval(args[0].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("crypto.blake3() expects a string argument".to_string()),
    };

    let hashed = blake3::hash(input.as_bytes());

    Ok((AST::String(hashed.to_string()), AST::Null))
}

fn bcrypt_hash(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let input = match eval(args[0].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("crypto.bcrypt_hash() expects a string argument".to_string()),
    };

    let hashed = bcrypt::hash(input, 12)
        .map_err(|e| format!("crypto.bcrypt_hash() failed: {}", e))?;

    Ok((AST::String(hashed.to_string()), AST::Null))
}

fn bcrypt_verify(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let password = match eval(args[0].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("crypto.bcrypt_verify() expects only string arguments".to_string()),
    };

    let hash = match eval(args[1].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("crypto.bcrypt_verify() expects only string arguments".to_string()),
    };

    let verified = bcrypt::verify(password, &hash)
        .map_err(|e| format!("crypto.bcrypt_verify() failed: {}", e))?;

    Ok((AST::Boolean(verified), AST::Null))
}

fn argon2_hash(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let input = match eval(args[0].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("crypto.argon2_hash() expects a string argument".to_string()),
    };

    let hashed = argon2::Argon2::default()
        .hash_password(
            input.as_bytes(), 
            &argon2::password_hash::SaltString::generate(&mut rand::thread_rng())
        )
        .map_err(|e| format!("crypto.argon2_hash() failed: {}", e))?
        .to_string();

    Ok((AST::String(hashed.to_string()), AST::Null))
}

fn argon2_verify(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let password = match eval(args[0].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("crypto.argon2_verify() expects only string arguments".to_string()),
    };

    let hash = match eval(args[1].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("crypto.argon2_verify() expects only string arguments".to_string()),
    };

    let verified = argon2::Argon2::default()
        .verify_password(
            password.as_bytes(),
            &argon2::password_hash::PasswordHash::new(&hash)
                .map_err(|e| format!("crypto.argon2_verify() failed: {}", e))?
        )
        .is_ok();

    Ok((AST::Boolean(verified), AST::Null))
}

fn scrypt_hash(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let input = match eval(args[0].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("crypto.scrypt_hash() expects a string argument".to_string()),
    };

    let salt = scrypt::password_hash::SaltString::generate(&mut rand::thread_rng());

    let hashed = scrypt::Scrypt.hash_password(
        input.as_bytes(), 
        &salt
    ).map_err(|e| format!("crypto.scrypt_hash() failed: {}", e))?;

    Ok((AST::String(hashed.to_string()), AST::Null))
}

fn scrypt_verify(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let password = match eval(args[0].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("crypto.scrypt_verify() expects only string arguments".to_string()),
    };

    let hash = match eval(args[1].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("crypto.scrypt_verify() expects only string arguments".to_string()),
    };

    let verified = scrypt::Scrypt
        .verify_password(
            password.as_bytes(),
            &scrypt::password_hash::PasswordHash::new(&hash)
                .map_err(|e| format!("crypto.scrypt_verify() failed: {}", e))?
        )
        .is_ok();

    Ok((AST::Boolean(verified), AST::Null))
}

// LEGACY

fn legacy_md5(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let input = match eval(args[0].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("crypto.md5() expects a string argument".to_string()),
    };

    let hashed = md5::compute(input.as_bytes());
    let hashed = format!("{:x}", hashed);

    Ok((AST::String(hashed), AST::Null))
}

pub fn get_object() -> HashMap<String, AST> {
    let mut object = HashMap::new();

    object.insert(
        "sha256".to_string(),
        AST::InternalFunction { 
            name: "sha256".to_string(), 
            args: vec!["str".to_string()], 
            call_fn: sha256 
        }
    );

    object.insert(
        "sha512".to_string(),
        AST::InternalFunction { 
            name: "sha512".to_string(), 
            args: vec!["str".to_string()], 
            call_fn: sha512 
        }
    );

    object.insert(
        "blake3".to_string(),
        AST::InternalFunction { 
            name: "blake3".to_string(), 
            args: vec!["str".to_string()], 
            call_fn: blake3 
        }
    );

    object.insert(
        "bcrypt_hash".to_string(),
        AST::InternalFunction { 
            name: "bcrypt_hash".to_string(), 
            args: vec!["str".to_string()],
            call_fn: bcrypt_hash
        } 
    );

    object.insert(
        "bcrypt_verify".to_string(),
        AST::InternalFunction { 
            name: "bcrypt_verify".to_string(), 
            args: vec!["password".to_string(), "hash".to_string()],
            call_fn: bcrypt_verify
        } 
    );

    object.insert(
        "argon2_hash".to_string(),
        AST::InternalFunction { 
            name: "argon2_hash".to_string(), 
            args: vec!["str".to_string()],
            call_fn: argon2_hash
        } 
    );

    object.insert(
        "argon2_verify".to_string(),
        AST::InternalFunction { 
            name: "argon2_verify".to_string(), 
            args: vec!["password".to_string(), "hash".to_string()],
            call_fn: argon2_verify
        } 
    );

    object.insert(
        "scrypt_hash".to_string(),
        AST::InternalFunction { 
            name: "scrypt_hash".to_string(), 
            args: vec!["str".to_string()],
            call_fn: scrypt_hash
        } 
    );

    object.insert(
        "scrypt_verify".to_string(),
        AST::InternalFunction { 
            name: "scrypt_verify".to_string(), 
            args: vec!["password".to_string(), "hash".to_string()],
            call_fn: scrypt_verify
        } 
    );

    // LEGACY
    object.insert(
        "legacy_md5".to_string(),
        AST::InternalFunction { 
            name: "legacy_md5".to_string(), 
            args: vec!["str".to_string()], 
            call_fn: legacy_md5 
        }
    );

    return object;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_time_package() {
        let object = get_object();

        assert_eq!(object.len(), 9);
    }
}