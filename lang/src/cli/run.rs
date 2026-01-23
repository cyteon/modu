use crate::utils;
use crate::parser::parse;
use bat::PrettyPrinter;
use bat::line_range::{LineRanges, LineRange};

pub fn run() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("Usage: modu run [file]");
        return;
    }

    let file: String = std::fs::read_to_string(&args[2]).unwrap();

    let context = &mut utils::create_context();

    parse(&file, context).unwrap_or_else(|e| {
        println!("\n⚠️  {}", e.0);
        println!("Traceback (most recent call last):"); 
        println!("    File \"{}\", line {}", &args[2], e.1);

        PrettyPrinter::new()
            .language("rust")
            .header(true)
            .line_numbers(true)
            .highlight(e.1)
            .grid(true)
            .input_file(std::path::Path::new(&args[2]))
            .line_ranges(
                LineRanges::from(vec![LineRange::from(&format!("{}:{}", e.1 - 1, e.1 + 1)).unwrap()])
            )
            .print()
            .unwrap();

        println!("Believe this is a bug? Report it: https://github.com/cyteon/modu/issues/new");
            
        std::process::exit(1);
    });
}