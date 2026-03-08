mod uuid;

pub fn get(name: &str) -> Option<crate::vm::value::Value> {
    match name {
        "uuid" => Some(uuid::object()),
        _ => None,
    }
}