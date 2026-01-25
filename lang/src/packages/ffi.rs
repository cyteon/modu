use std::{collections::HashMap, sync::Arc};
use std::path::PathBuf;
use crate::ast::AST;
use crate::eval::eval;

use modu_ffi::{FFIValue, FFIType};
type FFIFunction = unsafe extern "C" fn(i32, *const FFIValue) -> FFIValue;

pub fn execute_ffi_call(
    lib: Arc<libloading::Library>,
    name: &str,
    args: Vec<AST>,
    context: &mut HashMap<String, AST>,
) -> Result<AST, String> {
    let mut ffi_args = Vec::<FFIValue>::new();
    let mut owned_strings = Vec::<*mut std::ffi::c_char>::new();

    for arg in args {
        match eval(arg, context)? {
            AST::Null => ffi_args.push(FFIValue::null()),
            AST::String(v) => {
                let c = std::ffi::CString::new(v).unwrap();
                let ptr = c.into_raw();

                owned_strings.push(ptr);
                ffi_args.push(FFIValue::string(ptr));
            }
            AST::Integer(v) => ffi_args.push(FFIValue::integer(v)),
            AST::Float(v) => ffi_args.push(FFIValue::float(v)),
            AST::Boolean(v) => ffi_args.push(FFIValue::boolean(v)),

            _ => return Err("unsupported FFI argument".into()),
        }
    }

    unsafe {
        let func = lib.get::<FFIFunction>(name.as_bytes())
            .map_err(|e| format!("failed to load FFI function: {}", e))?;

        let result = func(ffi_args.len() as i32, ffi_args.as_ptr());

        for ptr in owned_strings {
            modu_ffi::ffi_free_string(ptr);
        }

        match result.ty {
            FFIType::Null => Ok(AST::Null),
            FFIType::String => {
                let c_str = std::ffi::CStr::from_ptr(result.value.string);
                let str_slice = match c_str.to_str() {
                    Ok(s) => s,
                    Err(_) => {
                        modu_ffi::ffi_free_string(result.value.string);
                        return Err("failed to convert FFI string to Rust string".to_string());
                    }
                };
                
                let string = str_slice.to_string();

                modu_ffi::ffi_free_string(result.value.string);

                Ok(AST::String(string))
            }
            FFIType::Integer => Ok(AST::Integer(result.value.integer)),
            FFIType::Float => Ok(AST::Float(result.value.float)),
            FFIType::Boolean => Ok(AST::Boolean(result.value.boolean)),
        }
    }
}

pub fn load(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    if args.is_empty() {
        return Err("ffi.load requires at least 1 argument".to_string());
    }
    
    let path = match eval(args[0].clone(), context) {
        Ok(AST::String(v)) => v,
        _ => return Err("ffi.load first argument must be a string".to_string()),
    };

    let mut full_path: String = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string() + "/";

    let sys_args = std::env::args().collect::<Vec<String>>();
    
    if sys_args.len() > 2 && sys_args[1] == "run" {
        let file_path = PathBuf::from(&sys_args[2]);
        let parent = file_path.parent().unwrap();
        let parent_str = parent.to_str().unwrap();
        full_path = parent_str.to_string() + "/";
    }

    full_path += path.as_str();

    unsafe {
        let lib = match libloading::Library::new(full_path.clone()) {
            Ok(lib) => lib,
            Err(e) => return Err(format!("failed to load library: {}", e)),
        };

        Ok((AST::FFILibrary { path: full_path, lib: std::sync::Arc::new(lib) }, AST::Null))
    }
}

pub fn get_object() -> HashMap<String, AST> {
	let mut object = HashMap::new();

    object.insert(
        "load".to_string(),
        AST::InternalFunction {
            name: "load".to_string(),
            args: vec!["__args__".to_string()],
            call_fn: load,
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