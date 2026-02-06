use colored::Colorize;
use crate::parser::parse;

pub fn run() {
    let args = std::env::args().collect::<Vec<String>>();
    
    let file: String;
    let file_path: String;
    
    if args.len() < 3 || (args[2].as_str().contains("--") && args.len() == 3) {
        let main_path = std::path::Path::new("main.modu");
        if main_path.exists() {
            file = std::fs::read_to_string(&main_path).unwrap_or_else(|e| {
                println!("Failed to read main.modu: {}", e);
                std::process::exit(1);
            });
            file_path = main_path.to_str().unwrap().to_string();
        } else {
            println!("Usage: modu run [file]");
            return;
        }
    } else {
        let path = std::path::Path::new(&args[2]);
        if !path.exists() {
            println!("{}", format!("File not found: {}", args[2]).red());
            return;
        }

        file = std::fs::read_to_string(&path).unwrap_or_else(|e| {
            println!("{}", format!("Failed to read file: {}", e).red());
            std::process::exit(1);
        });

        file_path = path.to_str().unwrap().to_string();
    }

    let context: &mut std::collections::HashMap<String, crate::ast::Expr> = &mut crate::utils::create_context();

    parse(&file, &file_path, context);
}