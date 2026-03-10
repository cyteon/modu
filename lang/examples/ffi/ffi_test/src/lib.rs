#[unsafe(no_mangle)]
pub extern "C" fn add(
    a: i64,
    b: i64
) -> i64 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn one() -> i64 {
    1
}

#[unsafe(no_mangle)]
pub extern "C" fn string() -> *const std::os::raw::c_char {
    std::ffi::CString::new("Hello from Rust!").unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn print(
    ptr: *const std::os::raw::c_char
) {
    let c_str = unsafe { std::ffi::CStr::from_ptr(ptr) };
    if let Ok(s) = c_str.to_str() {
        println!("{}", s);
    } else {
        eprintln!("Failed to convert C string to Rust string");
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn hello_world() {
    println!("Hello, world!");
}