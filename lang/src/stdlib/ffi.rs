use std::collections::HashMap;
use crate::vm::value::{BuiltinFn, Value};

pub fn object() -> Value {
    let mut methods = HashMap::new();

    methods.insert("load".to_string(), Value::BuiltinFn(BuiltinFn::new("load", load)));
    methods.insert("define".to_string(), Value::BuiltinFn(BuiltinFn::new("define", define)));
    methods.insert("unload".to_string(), Value::BuiltinFn(BuiltinFn::new("unload", unload)));

    Value::Object(methods)
}

pub struct FFILib {
    pub lib: libloading::Library,
    pub funcs: HashMap<String, FFISig>,
}

pub struct FFISig {
    pub arg_types: Vec<String>,
    pub ret_type: String,
}

thread_local! {
    pub static LIBS: std::cell::RefCell<Vec<Option<FFILib>>> = std::cell::RefCell::new(Vec::new());
}


fn load(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("ffi.load() takes exactly one argument ({} given)", args.len()));
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("ffi.load() argument must be a string, got {}", args[0].type_name())),
    };

    let lib = unsafe {
        libloading::Library::new(path)
            .map_err(|e| format!("ffi.load() failed to load library: {}", e))?
    };

    let idx = LIBS.with(|libs| {
        let mut libs = libs.borrow_mut();
        libs.push(Some(FFILib { lib, funcs: HashMap::new() }));
        libs.len() - 1
    });

    Ok(Value::FFILib(idx))
}

fn define(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 4 {
        return Err(format!("ffi.define() takes exactly four arguments ({} given)", args.len()));
    }

    let idx = match &args[0] {
        Value::FFILib(i) => *i,
        _ => return Err(format!("ffi.define() first argument must be a library handle, got {}", args[0].type_name())),
    };
    
    let name = match &args[1] {
        Value::String(s) => s,
        _ => return Err(format!("ffi.define() second argument must be a string, got {}", args[1].type_name())),
    };

    let arg_types = match &args[2] {
        Value::Array(arr) => arr.iter().map(|v| {
            match v {
                Value::String(s) => {
                    validate_type(s)?;
                    Ok(s.clone())
                },

                Value::Null => Ok("void".to_string()),

                _ => Err(format!("ffi.define() third argument must be an array of strings, got {}", v.type_name())),
            }
        }).collect::<Result<Vec<String>, String>>()?,

        _ => return Err(format!("ffi.define() third argument must be an array of strings, got {}", args[2].type_name())),
    };

    let ret_type = match &args[3] {
        Value::String(s) => s.clone(),
        Value::Null => "void".to_string(),
        _ => return Err(format!("ffi.define() fourth argument must be a string or null, got {}", args[3].type_name())),
    };

    LIBS.with(|libs| {
        let mut libs = libs.borrow_mut();
        let lib = libs[idx].as_mut()
            .ok_or_else(|| format!("invalid library handle: {}", idx))?;
        
        unsafe {
            lib.lib.get::<unsafe extern "C" fn()>(name.as_bytes())
                .map_err(|e| format!("ffi.define() failed to find symbol '{}': {}", name, e))?;
        }

        lib.funcs.insert(name.clone(), FFISig { arg_types, ret_type });
        Ok(Value::Null)
    })
}

fn unload(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("ffi.unload() takes exactly one argument ({} given)", args.len()));
    }

    let lib_index = match &args[0] {
        Value::FFILib(i) => *i,
        _ => return Err(format!("ffi.unload() argument must be a library handle, got {}", args[0].type_name())),
    };

    LIBS.with(|libs| {
        let mut libs = libs.borrow_mut();
        if lib_index >= libs.len() {
            return Err(format!("invalid library handle: {}", lib_index));
        }

        libs[lib_index] = None;
        Ok(Value::Null)
    })
}

fn validate_type(t: &str) -> Result<(), String> {
    match t {
        "i64" | "f64" | "string" | "bool" | "void" => Ok(()),
        _ => Err(format!("unsupported ffi type '{}', supported types: i64, f64, string, bool, void", t)),
    }
}

pub fn call_ffi(idx: usize, name: &str, args: Vec<Value>) -> Result<Value, String> {
    LIBS.with(|libs| {
        let libs = libs.borrow();
        let lib = libs.get(idx)
            .ok_or_else(|| format!("invalid library handle: {}", idx))?.as_ref()
            .ok_or_else(|| format!("library at index {} has been unloaded", idx))?;
        
        let sig = lib.funcs.get(name)
            .ok_or_else(|| format!("function '{}' is not defined in library {}", name, idx))?;
        
        if args.len() != sig.arg_types.len() {
            return Err(format!("function '{}' expects {} arguments, got {}", name, sig.arg_types.len(), args.len()));
        }

        let ffi_args: Vec<FFIArg> = args.iter()
            .zip(&sig.arg_types)
            .map(|(v, t)| to_ffi_arg(v, t))
            .collect::<Result<_, _>>()?;
        
        let ffi_arg_types: Vec<libffi::middle::Type> = sig.arg_types.iter()
            .map(|t| modu_to_ffi_type(t))
            .collect::<Result<_, _>>()?;
        
        let ffi_ret_type = modu_to_ffi_type(&sig.ret_type)?;
        let cif = libffi::middle::Cif::new(ffi_arg_types, ffi_ret_type);

        let func_ptr = unsafe {
            lib.lib.get::<unsafe extern "C" fn()>(name.as_bytes())
                .map_err(|e| format!("failed to get symbol '{}': {}", name, e))?
        };
        
        let code_ptr = libffi::middle::CodePtr::from_fun(*func_ptr);

        let c_args: Vec<libffi::middle::Arg> = ffi_args.iter().map(|arg| arg.as_arg()).collect();

        unsafe {
            match sig.ret_type.as_str() {
                "i64" => Ok(Value::Int(cif.call::<i64>(code_ptr, &c_args))),
                "f64" => Ok(Value::Float(cif.call::<f64>(code_ptr, &c_args))),
                "bool" => Ok(Value::Bool(cif.call::<i32>(code_ptr, &c_args) != 0)),

                "string" => {
                    let ptr = cif.call::<*const std::os::raw::c_char>(code_ptr, &c_args);
                    if ptr.is_null() {
                        Ok(Value::Null)
                    } else {
                        let c_str = std::ffi::CStr::from_ptr(ptr);
                        c_str.to_str()
                            .map(|s| Value::String(s.to_string()))
                            .map_err(|e| format!("failed to convert C string to Rust string: {}", e))
                    }
                },

                "void" => {
                    cif.call::<()>(code_ptr, &c_args);
                    Ok(Value::Null)
                }

                _ => Err(format!("unsupported return type '{}'", sig.ret_type)),
            }
        }
    })
}

fn modu_to_ffi_type(t: &str) -> Result<libffi::middle::Type, String> {
    match t {
        "i64" => Ok(libffi::middle::Type::i64()),
        "f64" => Ok(libffi::middle::Type::f64()),
        "bool" => Ok(libffi::middle::Type::i32()),
        "string" => Ok(libffi::middle::Type::pointer()),
        "void" => Ok(libffi::middle::Type::void()),
        _ => Err(format!("unsupported ffi type '{}'", t)),
    }
}

enum FFIArg {
    I64(i64),
    F64(f64),
    I32(i32),
    CString { ptr: *const std::os::raw::c_char, _owner: std::ffi::CString },
}

impl FFIArg {
    fn as_arg(&self) -> libffi::middle::Arg<'_> {
        match self {
            FFIArg::I64(i) => libffi::middle::Arg::new(i),
            FFIArg::F64(f) => libffi::middle::Arg::new(f),
            FFIArg::I32(i) => libffi::middle::Arg::new(i),
            FFIArg::CString { ptr, .. } => libffi::middle::Arg::new(ptr),
        }
    }
}

fn to_ffi_arg(value: &Value, t: &str) -> Result<FFIArg, String> {
    match (value, t) {
        (Value::Int(i), "i64") => Ok(FFIArg::I64(*i)),
        (Value::Float(f), "f64") => Ok(FFIArg::F64(*f)),
        (Value::Bool(b), "bool") => Ok(FFIArg::I32(*b as i32)),

        (Value::String(s), "string") => {
            let c_string = std::ffi::CString::new(s.as_str())
                .map_err(|e| format!("failed to convert string to C string: {}", e))?;
            let ptr = c_string.as_ptr();
            Ok(FFIArg::CString { ptr, _owner: c_string })
        },
        
        _ => Err(format!("cannot convert value of type {} to ffi type {}", value.type_name(), t)),
    }
}