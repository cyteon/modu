use std::ffi::c_char;

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
pub struct FFIValueUnion {
    pub string: *mut c_char,
    pub integer: i64,
    pub float: f64,
    pub boolean: bool,
}

#[repr(C)]
pub struct FFIValue {
    pub ty: FFIType,
    pub value: FFIValueUnion,
}