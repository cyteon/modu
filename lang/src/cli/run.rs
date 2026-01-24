use crate::utils;
use crate::parser::parse;
use bat::PrettyPrinter;
use bat::line_range::{LineRanges, LineRange};

pub fn run() {
    let args = std::env::args().collect::<Vec<String>>();
    
    let file: String;
    let file_path: String;
    let context = &mut utils::create_context();

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


    parse(&file, context).unwrap_or_else(|e| {
        println!("\n⚠️  {}", e.0);
        println!("Traceback (most recent call last):"); 
        println!("    File \"{}\", line {}", file_path, e.1);

        PrettyPrinter::new()
            .language("rust")
            .header(true)
            .line_numbers(true)
            .highlight(e.1)
            .grid(true)
            .input_file(std::path::Path::new(&file_path))
            .line_ranges(
                LineRanges::from(vec![LineRange::from(&format!("{}:{}", e.1 - 1, e.1 + 1)).unwrap()])
            )
            .print()
            .unwrap();

        println!("Believe this is a bug? Report it: https://github.com/cyteon/modu/issues/new");
            
        std::process::exit(1);
    });
}