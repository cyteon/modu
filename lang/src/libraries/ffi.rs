use std::path::PathBuf;
use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

type FFIFunction = unsafe extern "C" fn(i32, *const modu_ffi::FFIValue) -> modu_ffi::FFIValue;

pub fn execute_ffi_call(
    lib: &std::sync::Arc<libloading::Library>,
    func_name: &str,
    args: Vec<Spanned<Expr>>,
) -> Result<Expr, String> {
    let mut ffi_args = Vec::new();
    let mut owned_strings = Vec::<*mut std::ffi::c_char>::new();

    for arg in args {
        match arg.node {
            Expr::Int(i) => {
                ffi_args.push(modu_ffi::FFIValue::integer(i));
            }

            Expr::Float(f) => {
                ffi_args.push(modu_ffi::FFIValue::float(f));
            }

            Expr::String(s) => {
                let c_string = std::ffi::CString::new(s.clone())
                    .map_err(|e| format!("failed to convert string to C string: {}", e))?;
                let ptr = c_string.into_raw();
                owned_strings.push(ptr);

                ffi_args.push(modu_ffi::FFIValue::string(ptr));
            }

            Expr::Bool(b) => {
                ffi_args.push(modu_ffi::FFIValue::boolean(b));
            }

            Expr::Null => {
                ffi_args.push(modu_ffi::FFIValue::null());
            }

            _ => {
                return Err("unsupported argument type for FFI call".to_string());
            }
        }
    }

    unsafe {
        let func = lib.get::<FFIFunction>(func_name.as_bytes())
            .map_err(|e| format!("failed to load FFI function '{}': {}", func_name, e))?;
        
        let result = func(ffi_args.len() as i32, ffi_args.as_ptr());

        for ptr in owned_strings {
            modu_ffi::ffi_free_string(ptr);
        }

        match result.ty {
            modu_ffi::FFIType::Integer => Ok(Expr::Int(result.value.integer as i64)),
            modu_ffi::FFIType::Float => Ok(Expr::Float(result.value.float as f64)),
            modu_ffi::FFIType::String => {
                let c_str = std::ffi::CStr::from_ptr(result.value.string);
                let str_slice = c_str.to_str()
                    .map_err(|e| format!("failed to convert C string to Rust string: {}", e))?;
                
                let string = str_slice.to_string();
                modu_ffi::ffi_free_string(result.value.string);
                
                Ok(Expr::String(string))
            }
            modu_ffi::FFIType::Boolean => Ok(Expr::Bool(result.value.boolean)),
            modu_ffi::FFIType::Null => Ok(Expr::Null),
        }
    }
}

pub fn load(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    let path = match &args[0].node {
        Expr::String(s) => s,
        _ => return Err((
            "ffi.load expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let mut full_path = std::env::current_dir()
        .map_err(|e| (format!("failed to get current directory: {}", e), args[0].span.clone()))?
        .to_str()
        .ok_or_else(|| (format!("current directory path is not valid UTF-8"), args[0].span.clone()))?
        .to_string() + "/";

    let sys_args = std::env::args().collect::<Vec<String>>();
    if sys_args.len() > 2 && sys_args[1] == "run" {
        let file_path = PathBuf::from(&sys_args[2]);
        let parent = file_path.parent().ok_or_else(|| (
            "failed to get parent directory of current file".to_string(),
            args[0].span.clone(),
        ))?;

        full_path = parent.to_str().ok_or_else(|| (
            "parent directory path is not valid UTF-8".to_string(),
            args[0].span.clone(),
        ))?.to_string() + "/";
    }

    full_path.push_str(path);

    unsafe {
        let library = libloading::Library::new(&full_path)
            .map_err(|e| (format!("failed to load FFI library: {}", e), args[0].span.clone()))?;

        Ok(InternalFunctionResponse {
            return_value: Expr::FFILibrary(std::sync::Arc::new(library)),
            replace_self: None,
        })
    }
}

pub fn get_object() -> Expr {
    let mut symbols = std::collections::HashMap::new();

    symbols.insert(
        "load".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "load".to_string(),
                args: vec!["path".to_string()],
                func: load,
            },
            span: Span::default(),
        },
    );

    Expr::Module { symbols }
}