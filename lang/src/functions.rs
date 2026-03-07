use crate::vm::value::{InternalFn, Value};

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn _modu_print(ptr: *const u8, len: usize);
}

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn _modu_input(ptr: *const u8, len: usize, out_len: *mut usize) -> *mut u8;
}

fn native(name: &str, func: fn(Vec<Value>) -> Result<Value, String>) -> InternalFn {
    InternalFn { name: name.to_string(), func }
}

pub fn get_functions() -> Vec<InternalFn> {
    vec![
        native("print", print), 
    ]
}

fn print(args: Vec<Value>) -> Result<Value, String> {
    let output = args.iter().map(|v| format!("{}", v)).collect::<String>();

    #[cfg(target_arch = "wasm32")]
    {
        let text = format!("{}\n", output);
        unsafe { _modu_print(text.as_ptr(), text.len()) };
    }

    #[cfg(not(target_arch = "wasm32"))]
    println!("{}", output);

    Ok(Value::Null)
}