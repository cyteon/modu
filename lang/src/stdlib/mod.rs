mod crypto;
mod encoding;
mod json;
mod math;
mod time;
mod uuid;

#[cfg(not(target_arch = "wasm32"))]
mod os;

#[cfg(not(target_arch = "wasm32"))]
mod http;

#[cfg(not(target_arch = "wasm32"))]
mod fs;

pub fn get(name: &str) -> Option<crate::vm::value::Value> {
    match name {
        "crypto" => Some(crypto::object()),
        "encoding" => Some(encoding::object()),
        "json" => Some(json::object()),
        "math" => Some(math::object()),
        "time" => Some(time::object()),
        "uuid" => Some(uuid::object()),

        #[cfg(not(target_arch = "wasm32"))]
        "os" => Some(os::object()),

        #[cfg(not(target_arch = "wasm32"))]
        "http" => Some(http::object()),

        #[cfg(not(target_arch = "wasm32"))]
        "fs" => Some(fs::object()),

        _ => None,
    }
}