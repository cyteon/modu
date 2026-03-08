mod math;
mod time;
mod uuid;

pub fn get(name: &str) -> Option<crate::vm::value::Value> {
    match name {
        "math" => Some(math::object()),
        "time" => Some(time::object()),
        "uuid" => Some(uuid::object()),
        _ => None,
    }
}