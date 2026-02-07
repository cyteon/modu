#![feature(backtrace_frames)]
use std::panic::{catch_unwind, AssertUnwindSafe};

mod ast;
mod eval;
mod lexer;
mod parser;
mod utils;
mod libraries;
mod builtins;

fn main() {
    std::panic::set_hook(Box::new(|_| {}));

    if let Ok(data) = std::fs::read(std::env::current_exe().unwrap()) {
        let result = catch_unwind(AssertUnwindSafe(|| {
            let code = extract_code(&data);
            let mut context = utils::create_context();

            parser::parse(&code, "<embedded>", &mut context);
        }));

        if let Err(err) = result {
            let msg = err
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| err.downcast_ref::<String>().map(String::as_str))
                .unwrap_or("Unknown error");
            
            println!("Internal error: {}", msg);
        }

        return;
    } else {
        println!("Failed to read embedded code");
    }
}

fn extract_code(data: &[u8]) -> String {
    let start = b"__BEGIN_MODU_EMBEDDED_CODE__";
    let end = b"__END_MODU_EMBEDDED_CODE__";

    let start = data.windows(start.len()).position(|window| window == start).expect("Start marker not found") + start.len();
    let end = data.windows(end.len()).position(|window| window == end).expect("End marker not found");

    String::from_utf8(data[start..end].to_vec()).expect("Invalid UTF-8 in embedded code")
}