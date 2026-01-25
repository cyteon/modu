use std::ffi::{c_char, c_double};

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FFIType {
    Null,
    String,
    Integer,
    Float,
    Boolean,
}

#[repr(C)]
pub union FFIValueUnion {
    pub string: *mut c_char,
    pub integer: i64,
    pub float: c_double,
    pub boolean: bool,
}

#[repr(C)]
pub struct FFIValue {
    pub ty: FFIType,
    pub value: FFIValueUnion,
}

impl FFIValue {
    pub fn null() -> Self {
        FFIValue {
            ty: FFIType::Null,
            value: unsafe { std::mem::zeroed() },
        }
    }

    pub fn string(ptr: *mut c_char) -> Self {
        FFIValue {
            ty: FFIType::String,
            value: FFIValueUnion { string: ptr },
        }
    }

    pub fn integer(val: i64) -> Self {
        FFIValue {
            ty: FFIType::Integer,
            value: FFIValueUnion { integer: val },
        }
    }

    pub fn float(val: f64) -> Self {
        FFIValue {
            ty: FFIType::Float,
            value: FFIValueUnion { float: val },
        }
    }

    pub fn boolean(val: bool) -> Self {
        FFIValue {
            ty: FFIType::Boolean,
            value: FFIValueUnion { boolean: val },
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ffi_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(std::ffi::CString::from_raw(ptr));
        }
    }
}