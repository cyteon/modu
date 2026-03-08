mod uuid;
mod time;

pub fn get(name: &str) -> Option<crate::vm::value::Value> {
    match name {
        "uuid" => Some(uuid::object()),
        "time" => Some(time::object()),
        _ => None,
    }
}