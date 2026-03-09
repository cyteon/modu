use std::collections::HashMap;
use crate::vm::value::{BuiltinFn, Value};

pub fn object() -> Value {
    let mut methods = HashMap::new();

    methods.insert("open".to_string(), Value::BuiltinFn(BuiltinFn::new("open", open)));
    methods.insert("read".to_string(), Value::BuiltinFn(BuiltinFn::new("read", read)));
    methods.insert("write".to_string(), Value::BuiltinFn(BuiltinFn::new("write", write)));
    methods.insert("stat".to_string(), Value::BuiltinFn(BuiltinFn::new("stat", stat)));
    methods.insert("close".to_string(), Value::BuiltinFn(BuiltinFn::new("close", close)));
    methods.insert("exists".to_string(), Value::BuiltinFn(BuiltinFn::new("exists", exists)));
    methods.insert("mkdir".to_string(), Value::BuiltinFn(BuiltinFn::new("mkdir", mkdir)));
    methods.insert("rmdir".to_string(), Value::BuiltinFn(BuiltinFn::new("rmdir", rmdir)));
    methods.insert("remove".to_string(), Value::BuiltinFn(BuiltinFn::new("remove", remove)));

    Value::Object(methods)
}

pub fn list_fns() -> Vec<String> {
    vec![
        "open".to_string(),
        "read".to_string(),
        "write".to_string(),
        "stat".to_string(),
        "close".to_string(),
        "exists".to_string(),
        "mkdir".to_string(),
        "rmdir".to_string(),
        "remove".to_string(),
    ]
}

thread_local! {
    static FILES: std::cell::RefCell<Vec<Option<(std::fs::File, FileMode)>>> = std::cell::RefCell::new(Vec::new());
}

enum FileMode {
    Read,
    Write,
    Append,
    ReadWrite
}

fn open(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("fs.open() takes exactly two arguments ({} given)", args.len()));
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("fs.open() argument must be a string, got {}", args[0].type_name())),
    };

    let mode = match &args[1] {
        Value::String(s) => s,
        _ => return Err(format!("fs.open() second argument must be a string, got {}", args[1].type_name())),
    };

    let (file, mode) = match mode.as_str() {
        "r" => (
            std::fs::File::open(path).map_err(|e| format!("failed to open file '{}': {}", path, e)),
            FileMode::Read
        ),

        "w" => (
            std::fs::File::create(path).map_err(|e| format!("failed to create file '{}': {}", path, e)),
            FileMode::Write
        ),

        "a" => (
            std::fs::OpenOptions::new().append(true).create(true).open(path).map_err(|e| format!("failed to open file '{}': {}", path, e)),
            FileMode::Append
        ),

        "rw" => (
            std::fs::OpenOptions::new().read(true).write(true).create(true).open(path).map_err(|e| format!("failed to open file '{}': {}", path, e)),
            FileMode::ReadWrite
        ),

        _ => return Err(format!("fs.open() mode must be 'r', 'w', 'a', or 'rw', got '{}'", mode)),
    };
    
   let idx = FILES.with(|files| {
        let mut files = files.borrow_mut();
        files.push(Some((file.unwrap(), mode)));
        files.len() - 1
    });

    Ok(Value::Int(idx as i64))
}

fn read(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("fs.read() takes exactly one argument ({} given)", args.len()));
    }

    let idx = match &args[0] {
        Value::Int(i) => *i as usize,
        _ => return Err(format!("fs.read() argument must be an integer file index, got {}", args[0].type_name())),
    };

    FILES.with(|files| {
        use std::io::Seek;

        let mut files = files.borrow_mut();
        let file_entry = files.get_mut(idx).ok_or_else(|| format!("fs.read() invalid file index: {}", idx))?;
        let (file, mode) = file_entry.as_mut().ok_or_else(|| format!("fs.read() file at index {} is closed", idx))?;

        if !matches!(mode, FileMode::Read | FileMode::ReadWrite) {
            return Err(format!("fs.read() file at index {} is not open for reading", idx));
        }

        file.seek(std::io::SeekFrom::Start(0)).map_err(|e| format!("failed to seek in file at index {}: {}", idx, e))?;

        let mut content = String::new();
        std::io::Read::read_to_string(file, &mut content).map_err(|e| format!("failed to read from file at index {}: {}", idx, e))?;
        
        Ok(Value::String(content))
    })
}

fn write(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!("fs.write() takes exactly two arguments ({} given)", args.len()));
    }

    let idx = match &args[0] {
        Value::Int(i) => *i as usize,
        _ => return Err(format!("fs.write() first argument must be an integer file index, got {}", args[0].type_name())),
    };

    let content = match &args[1] {
        Value::String(s) => s,
        _ => return Err(format!("fs.write() second argument must be a string, got {}", args[1].type_name())),
    };

    FILES.with(|files| {
        let mut files = files.borrow_mut();
        let file_entry = files.get_mut(idx).ok_or_else(|| format!("fs.write() invalid file index: {}", idx))?;
        let (file, mode) = file_entry.as_mut().ok_or_else(|| format!("fs.write() file at index {} is closed", idx))?;

        if !matches!(mode, FileMode::Write | FileMode::Append | FileMode::ReadWrite) {
            return Err(format!("fs.write() file at index {} is not open for writing", idx));
        }

        std::io::Write::write_all(file, content.as_bytes()).map_err(|e| format!("failed to write to file at index {}: {}", idx, e))?;
        Ok(Value::Null)
    })
}

fn stat(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("fs.stat() takes exactly one argument ({} given)", args.len()));
    }

    let idx = match &args[0] {
        Value::Int(i) => *i as usize,
        _ => return Err(format!("fs.stat() argument must be an integer file index, got {}", args[0].type_name())),
    };

    FILES.with(|files| {
        let mut files = files.borrow_mut();
        let file_entry = files.get_mut(idx).ok_or_else(|| format!("fs.stat() invalid file index: {}", idx))?;
        let (file, _) = file_entry.as_mut().ok_or_else(|| format!("fs.stat() file at index {} is closed", idx))?;

        let metadata = file.metadata().map_err(|e| format!("failed to get metadata for file at index {}: {}", idx, e))?;
        Ok(Value::Object(vec![
            ("size".to_string(), Value::Int(metadata.len() as i64)),
            ("is_file".to_string(), Value::Bool(metadata.is_file())),
            ("is_dir".to_string(), Value::Bool(metadata.is_dir())),
        ].into_iter().collect()))
    })
}

fn close(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("fs.close() takes exactly one argument ({} given)", args.len()));
    }

    let idx = match &args[0] {
        Value::Int(i) => *i as usize,
        _ => return Err(format!("fs.close() argument must be an integer file index, got {}", args[0].type_name())),
    };

    FILES.with(|files| {
        let mut files = files.borrow_mut();
        let file_entry = files.get_mut(idx).ok_or_else(|| format!("fs.close() invalid file index: {}", idx))?;
        *file_entry = None;
        Ok(Value::Null)
    })
}

fn exists(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("fs.exists() takes exactly one argument ({} given)", args.len()));
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("fs.exists() argument must be a string, got {}", args[0].type_name())),
    };

    Ok(Value::Bool(std::path::Path::new(path).exists()))
}

fn mkdir(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("fs.mkdir() takes exactly one argument ({} given)", args.len()));
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("fs.mkdir() argument must be a string, got {}", args[0].type_name())),
    };

    std::fs::create_dir(path).map_err(|e| format!("failed to create directory '{}': {}", path, e))?;
    Ok(Value::Null)
}

fn rmdir(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("fs.rmdir() takes exactly one argument ({} given)", args.len()));
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("fs.rmdir() argument must be a string, got {}", args[0].type_name())),
    };

    std::fs::remove_dir(path).map_err(|e| format!("failed to remove directory '{}': {}", path, e))?;
    Ok(Value::Null)
}

fn remove(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("fs.remove() takes exactly one argument ({} given)", args.len()));
    }

    let path = match &args[0] {
        Value::String(s) => s,
        _ => return Err(format!("fs.remove() argument must be a string, got {}", args[0].type_name())),
    };

    std::fs::remove_file(path).map_err(|e| format!("failed to remove file '{}': {}", path, e))?;
    Ok(Value::Null)
}