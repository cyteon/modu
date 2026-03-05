use colored::Colorize;

mod time;
mod encoding;
mod uuid;
mod math;
mod json;
mod crypto;

#[cfg(not(target_arch = "wasm32"))]
pub mod fs;
#[cfg(not(target_arch = "wasm32"))]
mod http;
#[cfg(not(target_arch = "wasm32"))]
pub mod ffi;
#[cfg(not(target_arch = "wasm32"))]
mod os;

pub fn get_package(name: &str) -> Option<crate::ast::Expr> {
    match name {
        "time" => Some(time::get_object()),
        "encoding" => Some(encoding::get_object()),
        "uuid" => Some(uuid::get_object()),
        "math" => Some(math::get_object()),
        "json" => Some(json::get_object()),
        "crypto" => Some(crypto::get_object()),

        #[cfg(not(target_arch = "wasm32"))]
        "os" => Some(os::get_object())

        #[cfg(not(target_arch = "wasm32"))]
        "http" => Some(http::get_object())

        #[cfg(not(target_arch = "wasm32"))]
        "ffi" => Some(ffi::get_object())

        #[cfg(not(target_arch = "wasm32"))]
        "file" => {
            println!("{}", "warning: the 'file' package has been renamed to 'fs' to better represent it, the 'file' import will be removed in a future version".dimmed());  
            Some(fs::get_object())
        }

        #[cfg(not(target_arch = "wasm32"))]
        "fs" => Some(fs::get_object())

        _ => None,
    }
}