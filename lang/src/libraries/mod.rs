mod time;
mod encoding;
mod uuid;
mod math;
mod json;
mod crypto;

#[cfg(not(target_arch = "wasm32"))]
pub mod file;
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
        "os" => {
            let sys_args = std::env::args().collect::<Vec<String>>();
            if sys_args.len() > 1 && sys_args[1] == "server" {
                return None;
            }

            Some(os::get_object())
        }

        #[cfg(not(target_arch = "wasm32"))]
        "http" => {
            let sys_args = std::env::args().collect::<Vec<String>>();
            if sys_args.len() > 1 && sys_args[1] == "server" {
                return None;
            }

            Some(http::get_object())
        },

        #[cfg(not(target_arch = "wasm32"))]
        "ffi" => {
            let sys_args = std::env::args().collect::<Vec<String>>();
            if sys_args.len() > 1 && sys_args[1] == "server" {
                return None;
            }

            Some(ffi::get_object())
        }

        #[cfg(not(target_arch = "wasm32"))]
        "file" => {
            let sys_args = std::env::args().collect::<Vec<String>>();
            if sys_args.len() > 1 && sys_args[1] == "server" {
                return None;
            }

            Some(file::get_object())
        }

        _ => None,
    }
}