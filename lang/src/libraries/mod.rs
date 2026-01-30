mod time;
mod encoding;
mod uuid;

pub fn get_package(name: &str) -> Option<crate::ast::Expr> {
    match name {
        "time" => Some(time::get_object()),
        "encoding" => Some(encoding::get_object()),
        "uuid" => Some(uuid::get_object()),
        _ => None,
    }
}