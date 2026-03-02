use argon2::{PasswordHasher, PasswordVerifier};
use sha2::Digest;

use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

pub fn sha256(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let input = match &args[0].node {
        Expr::String(s) => s,
        _ => Err((
            "crypto.sha256 expects a string argument".to_string(),
            args[0].span.clone(),
        ))?,
    };

    let hashed = sha2::Sha256::digest(input.as_bytes());
    let hashed = format!("{:x}", hashed);

    Ok(InternalFunctionResponse {
        return_value: Expr::String(hashed),
        replace_self: None,
    })
}

pub fn sha512(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let input = match &args[0].node {
        Expr::String(s) => s,
        _ => Err((
            "crypto.sha512 expects a string argument".to_string(),
            args[0].span.clone(),
        ))?,
    };

    let hashed = sha2::Sha512::digest(input.as_bytes());
    let hashed = format!("{:x}", hashed);

    Ok(InternalFunctionResponse {
        return_value: Expr::String(hashed),
        replace_self: None,
    })
}

pub fn blake3(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let input = match &args[0].node {
        Expr::String(s) => s,
        _ => Err((
            "crypto.blake3 expects a string argument".to_string(),
            args[0].span.clone(),
        ))?,
    };

    let hashed = blake3::hash(input.as_bytes());

    Ok(InternalFunctionResponse {
        return_value: Expr::String(hashed.to_hex().to_string()),
        replace_self: None,
    })
}

fn bcrypt_hash(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let input = match &args[0].node {
        Expr::String(s) => s,
        _ => Err((
            "crypto.bcrypt_hash expects a string argument".to_string(),
            args[0].span.clone(),
        ))?,
    };

    let hashed = bcrypt::hash(input, 12)
        .map_err(|e| (
            format!("crypto.bcrypt_hash failed: {}", e),
            args[0].span.clone(),
        ))?;
    
    Ok(InternalFunctionResponse {
        return_value: Expr::String(hashed),
        replace_self: None,
    })
}

pub fn bcrypt_verify(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let password = match &args[0].node {
        Expr::String(s) => s,
        _ => Err((
            "crypto.bcrypt_verify expects the first argument to be a string".to_string(),
            args[0].span.clone(),
        ))?,
    };

    let hash = match &args[1].node {
        Expr::String(s) => s,
        _ => Err((
            "crypto.bcrypt_verify expects the second argument to be a string".to_string(),
            args[1].span.clone(),
        ))?,
    };

    let is_valid = bcrypt::verify(password, hash)
        .map_err(|e| (
            format!("crypto.bcrypt_verify failed: {}", e),
            args[0].span.clone(),
        ))?;

    Ok(InternalFunctionResponse {
        return_value: Expr::Bool(is_valid),
        replace_self: None,
    })
}

pub fn argon2_hash(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let input = match &args[0].node {
        Expr::String(s) => s,
        _ => Err((
            "crypto.argon2_hash expects a string argument".to_string(),
            args[0].span.clone(),
        ))?,
    };


    let hashed = argon2::Argon2::default()
        .hash_password(
            input.as_bytes(), 
            &argon2::password_hash::SaltString::generate(&mut rand::thread_rng())
        )
        .map_err(|e| (
            format!("crypto.argon2_hash failed: {}", e),
            args[0].span.clone(),
        ))?
        .to_string();

    Ok(InternalFunctionResponse {
        return_value: Expr::String(hashed),
        replace_self: None,
    })
}

pub fn argon2_verify(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let password = match &args[0].node {
        Expr::String(s) => s,
        _ => Err((
            "crypto.argon2_verify expects the first argument to be a string".to_string(),
            args[0].span.clone(),
        ))?,
    };

    let hash = match &args[1].node {
        Expr::String(s) => s,
        _ => Err((
            "crypto.argon2_verify expects the second argument to be a string".to_string(),
            args[1].span.clone(),
        ))?,
    };

    let parsed_hash = argon2::PasswordHash::new(hash)
        .map_err(|e| (
            format!("crypto.argon2_verify failed to parse hash: {}", e),
            args[1].span.clone(),
        ))?;

    let is_valid = argon2::Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(InternalFunctionResponse {
        return_value: Expr::Bool(is_valid),
        replace_self: None,
    })
}

pub fn scrypt_hash(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let input = match &args[0].node {
        Expr::String(s) => s,
        _ => Err((
            "crypto.scrypt_hash expects a string argument".to_string(),
            args[0].span.clone(),
        ))?,
    };

    let salt = scrypt::password_hash::SaltString::generate(&mut rand::thread_rng());

    let hashed = scrypt::Scrypt.hash_password(
        input.as_bytes(), 
        &salt
    ).map_err(|e| (
        format!("crypto.scrypt_hash failed: {}", e),
        args[0].span.clone(),
    ))?;

    Ok(InternalFunctionResponse {
        return_value: Expr::String(hashed.to_string()),
        replace_self: None,
    })
}

pub fn scrypt_verify(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let password = match &args[0].node {
        Expr::String(s) => s,
        _ => Err((
            "crypto.scrypt_verify expects the first argument to be a string".to_string(),
            args[0].span.clone(),
        ))?,
    };

    let hash = match &args[1].node {
        Expr::String(s) => s,
        _ => Err((
            "crypto.scrypt_verify expects the second argument to be a string".to_string(),
            args[1].span.clone(),
        ))?,
    };

    let parsed_hash = scrypt::password_hash::PasswordHash::new(hash)
        .map_err(|e| (
            format!("crypto.scrypt_verify failed to parse hash: {}", e),
            args[1].span.clone(),
        ))?;

    let is_valid = scrypt::Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    Ok(InternalFunctionResponse {
        return_value: Expr::Bool(is_valid),
        replace_self: None,
    })
}

// LEGACY
pub fn md5(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let input = match &args[0].node {
        Expr::String(s) => s,
        _ => Err((
            "crypto.legacy.md5 expects a string argument".to_string(),
            args[0].span.clone(),
        ))?,
    };

    let hashed = md5::compute(input.as_bytes());
    let hashed = format!("{:x}", hashed);

    Ok(InternalFunctionResponse {
        return_value: Expr::String(hashed),
        replace_self: None,
    })
}

pub fn get_object() -> Expr {
    let mut symbols = std::collections::HashMap::new();

    symbols.insert(
        "sha256".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "sha256".to_string(),
                args: vec!["input".to_string()],
                func: sha256,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "sha512".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "sha512".to_string(),
                args: vec!["input".to_string()],
                func: sha512,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "blake3".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "blake3".to_string(),
                args: vec!["input".to_string()],
                func: blake3,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "bcrypt_hash".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "bcrypt_hash".to_string(),
                args: vec!["input".to_string()],
                func: bcrypt_hash,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "bcrypt_verify".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "bcrypt_verify".to_string(),
                args: vec!["input".to_string(), "hash".to_string()],
                func: bcrypt_verify,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "argon2_hash".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "argon2_hash".to_string(),
                args: vec!["input".to_string()],
                func: argon2_hash,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "argon2_verify".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "argon2_verify".to_string(),
                args: vec!["input".to_string(), "hash".to_string()],
                func: argon2_verify,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "scrypt_hash".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "scrypt_hash".to_string(),
                args: vec!["input".to_string()],
                func: scrypt_hash,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "scrypt_verify".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "scrypt_verify".to_string(),
                args: vec!["input".to_string(), "hash".to_string()],
                func: scrypt_verify,
            },
            span: Span::default(),
        },
    );

    symbols.insert(
        "legacy".to_string(),
        SpannedExpr {
            node: Expr::Module {
                symbols: {
                    let mut legacy_symbols = std::collections::HashMap::new();

                    legacy_symbols.insert(
                        "md5".to_string(),
                        SpannedExpr {
                            node: Expr::InternalFunction {
                                name: "md5".to_string(),
                                args: vec!["input".to_string()],
                                func: md5,
                            },
                            span: Span::default(),
                        },
                    );

                    legacy_symbols
                },
            },
            span: Span::default(),
        },
    );

    Expr::Module { symbols }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha25() {
        let result = sha256(vec![ SpannedExpr { node: Expr::String("test".to_string()), span: Span::default() } ]);

        assert_eq!(
            result.unwrap().return_value, 
            Expr::String("9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08".to_string())
        )
    }

    #[test]
    fn test_sha512() {
        let result = sha512(vec![ SpannedExpr { node: Expr::String("test".to_string()), span: Span::default() } ]);

        assert_eq!(
            result.unwrap().return_value, 
            Expr::String("ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db27ac185f8a0e1d5f84f88bc887fd67b143732c304cc5fa9ad8e6f57f50028a8ff".to_string())
        )
    }

    #[test]
    fn test_blake3() {
        let result = blake3(vec![ SpannedExpr { node: Expr::String("test".to_string()), span: Span::default() } ]);

        assert_eq!(
            result.unwrap().return_value, 
            Expr::String("4878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".to_string())
        )
    }

    #[test]
    fn test_md5() {
        let result = md5(vec![ SpannedExpr { node: Expr::String("test".to_string()), span: Span::default() } ]);

        assert_eq!(
            result.unwrap().return_value, 
            Expr::String("098f6bcd4621d373cade4e832627b4f6".to_string())
        )
    }
}