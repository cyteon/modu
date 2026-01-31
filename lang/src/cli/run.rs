use crate::parser::parse;

pub fn run() {
    let args = std::env::args().collect::<Vec<String>>();
    
    let file: String;
    let file_path: String;
    
    if args.len() < 3 || (args[2].as_str().contains("--") && args.len() == 3) {
        let main_path = std::path::Path::new("main.modu");
        if main_path.exists() {
            file = std::fs::read_to_string(&main_path).unwrap();
            file_path = main_path.to_str().unwrap().to_string();
        } else {
            println!("Usage: modu run [file]");
            return;
        }
    } else {
        file = std::fs::read_to_string(&args[2]).unwrap();
        file_path = args[2].clone();
    }

    let context: &mut std::collections::HashMap<String, crate::ast::Expr> = &mut crate::utils::create_context();

    parse(&file, &file_path, context);
}