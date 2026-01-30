mod time;
mod encoding;
mod uuid;
mod os;
mod math;
mod http;
mod json;

pub fn get_package(name: &str) -> Option<crate::ast::Expr> {
    match name {
        "time" => Some(time::get_object()),
        "encoding" => Some(encoding::get_object()),
        "uuid" => Some(uuid::get_object()),
        "math" => Some(math::get_object()),
        "json" => Some(json::get_object()),

        "os" => {
            let sys_args = std::env::args().collect::<Vec<String>>();
            if sys_args.len() > 1 && sys_args[1] == "server" {
                return None;
            }

            Some(os::get_object())
        }
        "http" => {
            let sys_args = std::env::args().collect::<Vec<String>>();
            if sys_args.len() > 1 && sys_args[1] == "server" {
                return None;
            }

            Some(http::get_object())
        }
        _ => None,
    }
}