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

    let ast = parse(&file, &file_path);

    if ast.is_err() {
        println!("{}: failed to parse file", "error".red());
        return;
    }

    let mut compiler = crate::compiler::compiler::Compiler::new();

    if let Err(e) = compiler.compile_program(ast.clone().unwrap()) {
        println!("{}: {}", "Compilation error".red(), e);
        return;
    }

    if args.contains(&"--dump".to_string()) {
        use std::io::Write;

        let mut ast_file = std::fs::File::create("dump.ast").unwrap();
        ast_file
            .write_all(format!("{:#?}", ast).as_bytes())
            .unwrap();

        let mut bytecode_file = std::fs::File::create("dump.bytecode").unwrap();
        let mut string = String::new();

        for (i, chunk) in compiler.chunks.iter().enumerate() {
            string.push_str(&format!("=== chunk[{}] \"{}\" ({} locals) ===\n", i, chunk.name, chunk.locals_count));

            for (j, instruction) in chunk.instructions.iter().enumerate() {
                string.push_str(&format!("\t{:04}: {:?}\n", j, instruction));
            }

            if !chunk.constants.is_empty() {
                string.push_str("\n\tconstants:\n");

                for (j, constant) in chunk.constants.iter().enumerate() {
                    string.push_str(&format!("\t\t{:04}: {:?}\n", j, constant));
                }
            }

            if i != compiler.chunks.len() - 1 {
                string.push_str("\n");
            }
        }

        bytecode_file
            .write_all(string.as_bytes())
            .unwrap();
    }

    let source_path = std::path::PathBuf::from(&args[2])
        .canonicalize()
        .map_err(|_| format!("cannot find file '{}'", args[2]))
        .unwrap();

    let mut vm = crate::vm::vm::VM::new_with_source(compiler.chunks, source_path);

    if let Err(e) = vm.run() {
        println!("{}: {}", "Runtime error".red(), e);
        return;
    }
}