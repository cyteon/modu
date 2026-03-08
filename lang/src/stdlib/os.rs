use std::collections::HashMap;
use crate::vm::value::{BuiltinFn, Value};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

pub fn object() -> Value {
    let mut methods = HashMap::new();

    methods.insert("exec".to_string(), Value::BuiltinFn(BuiltinFn::new("exec", exec)));
    methods.insert("pid".to_string(), Value::BuiltinFn(BuiltinFn::new("pid", pid)));
    methods.insert("uid".to_string(), Value::BuiltinFn(BuiltinFn::new("uid", uid)));
    methods.insert("gid".to_string(), Value::BuiltinFn(BuiltinFn::new("gid", gid)));
    methods.insert("getenv".to_string(), Value::BuiltinFn(BuiltinFn::new("getenv", getenv)));
    methods.insert("setenv".to_string(), Value::BuiltinFn(BuiltinFn::new("setenv", setenv)));
    methods.insert("unsetenv".to_string(), Value::BuiltinFn(BuiltinFn::new("unsetenv", unsetenv)));
    methods.insert("name".to_string(), Value::String({
        if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "linux") {
            "linux"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else {
            "unknown"
        }
    }.to_string()));

    Value::Object(methods)
}

fn exec(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        return Err("os.exec() takes at least one argument (0 given)".to_string());
    }

    let command = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("os.exec() first argument must be a string, got {}", args[0].type_name())),
    };

    let output = {
        #[cfg(windows)]
        {
            std::process::Command::new("cmd")
                .args(["/C", command])
                .creation_flags(0x08000000)
                .output()
                .map_err(|e| format!("os.exec() failed to execute command: {}", e))?
        }

        #[cfg(not(windows))]
        {
            std::process::Command::new("sh")
                .args(["-c", command])
                .output()
                .map_err(|e| format!("os.exec() failed to execute command: {}", e))?
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let status_code = output.status.code().unwrap_or(-1);

    let obj = Value::Object({
        let mut m = HashMap::new();
        m.insert("stdout".to_string(), Value::String(stdout));
        m.insert("stderr".to_string(), Value::String(stderr));
        m.insert("status_code".to_string(), Value::Int(status_code as i64));
        m.insert("success".to_string(), Value::Bool(output.status.success()));
        m
    });

    Ok(obj)
}

fn pid(_args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Int(std::process::id() as i64))
}

fn uid(_args: Vec<Value>) -> Result<Value, String> {
    #[cfg(unix)]
    {
        Ok(Value::Int(unsafe { libc::getuid() } as i64))
    }

    #[cfg(not(unix))]
    {
        Err("os.uid() is not supported on this platform".to_string())
    }
}

fn gid(_args: Vec<Value>) -> Result<Value, String> {
    #[cfg(unix)]
    {
        Ok(Value::Int(unsafe { libc::getgid() } as i64))
    }

    #[cfg(not(unix))]
    {
        Err("os.gid() is not supported on this platform".to_string())
    }
}

fn getenv(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("os.getenv() takes exactly one argument ({} given)", args.len()));
    }

    let key = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("os.getenv() argument must be a string, got {}", args[0].type_name())),
    };

    if let Some(value) = std::env::var_os(key) {
        Ok(Value::String(value.to_string_lossy().to_string()))
    } else {
        Ok(Value::Null)
    }
}

fn setenv(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("os.setenv() takes exactly two arguments ({} given)", args.len()));
    }

    let key = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("os.setenv() first argument must be a string, got {}", args[0].type_name())),
    };

    let value = match &args[1] {
        Value::String(s) => s,
        _ => return Err(format!("os.setenv() second argument must be a string, got {}", args[1].type_name())),
    };

    unsafe {
        std::env::set_var(key, value);
    }

    Ok(Value::Null)
}

fn unsetenv(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("os.unsetenv() takes exactly one argument ({} given)", args.len()));
    }

    let key = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("os.unsetenv() argument must be a string, got {}", args[0].type_name())),
    };

    unsafe {
        std::env::remove_var(key);
    }

    Ok(Value::Null)
}