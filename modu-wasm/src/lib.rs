use wasm_bindgen::prelude::*;
use std::sync::Mutex;
use std::panic::{catch_unwind, AssertUnwindSafe};

static OUTPUT: Mutex<String> = Mutex::new(String::new());

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

// for smaller binary size
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn eval_modu(code: &str) -> String {
    let mut context = modu::utils::create_context();
    
    let result = catch_unwind(AssertUnwindSafe(|| {
        modu::parser::parse(code, "<browser>", &mut context);
    }));

    if let Err(panic) = result {
        let msg = panic
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| panic.downcast_ref::<String>().map(String::as_str))
            .unwrap_or("Unknown internal error");

        let mut output = OUTPUT.lock().unwrap();
        output.push_str(&format!("Internal error: {}\n", msg));
    }

    let string = format!("{}", OUTPUT.lock().unwrap().as_str());
    OUTPUT.lock().unwrap().clear();

    string
}

#[wasm_bindgen]
pub fn modu_version() -> String {
    modu::VERSION.to_string()
}

#[unsafe(no_mangle)]
pub extern "C" fn _modu_print(ptr: *const u8, len: usize) {
    let text = unsafe {
        std::str::from_utf8(std::slice::from_raw_parts(ptr, len)).unwrap()
    };

    let mut output = OUTPUT.lock().unwrap();
    output.push_str(text);
}

#[unsafe(no_mangle)]
pub extern "C" fn _modu_input(ptr: *const u8, len: usize, out_len: *mut usize) -> *mut u8 {
    let prompt = unsafe {
        std::str::from_utf8(std::slice::from_raw_parts(ptr, len)).unwrap()
    };

    let input = web_sys::window().unwrap().prompt_with_message(prompt).unwrap_or_default().unwrap_or_default();

    let mut output = OUTPUT.lock().unwrap();
    output.push_str(&format!("> {}\n", input));

    let mut bytes = input.into_bytes();
    bytes.shrink_to_fit();
    let len = bytes.len();
    let ptr = bytes.as_mut_ptr();
    std::mem::forget(bytes);

    unsafe { *out_len = len; }
    ptr
}