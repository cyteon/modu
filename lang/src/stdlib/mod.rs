mod crypto;
mod encoding;
mod math;
mod time;
mod uuid;

pub fn get(name: &str) -> Option<crate::vm::value::Value> {
    match name {
        "crypto" => Some(crypto::object()),
        "encoding" => Some(encoding::object()),
        "math" => Some(math::object()),
        "time" => Some(time::object()),
        "uuid" => Some(uuid::object()),
        _ => None,
    }
}