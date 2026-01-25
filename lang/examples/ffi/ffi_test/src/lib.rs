use modu_ffi::*;

#[unsafe(no_mangle)]
pub extern "C" fn add(
    argc: std::ffi::c_int,
    argv: *const FFIValue
) -> FFIValue {
    if argc != 2 {
        panic!("add requires 2 arguments");
    }

    unsafe {
        let a = (*argv.offset(0 as isize)).value.integer;
        let b = (*argv.offset(1 as isize)).value.integer;

        FFIValue::integer(a + b)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn one() -> FFIValue {
    FFIValue::integer(1)
}

#[unsafe(no_mangle)]
pub extern "C" fn string() -> FFIValue {
    FFIValue::string(std::ffi::CString::new("Hello from Rust!").unwrap().into_raw())
}

#[unsafe(no_mangle)]
pub extern "C" fn print(
    argc: std::ffi::c_int,
    argv: *const FFIValue
) {
    if argc != 1 {
        panic!("print requires 1 argument");
    }

    let str = unsafe {
        std::ffi::CStr::from_ptr((*argv.offset(0 as isize)).value.string)
    };

    println!("{}", str.to_str().unwrap());
}


#[unsafe(no_mangle)]
pub extern "C" fn hello_world() -> FFIValue {
    println!("Hello, World!");
    FFIValue::null()
}