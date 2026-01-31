
use std::io::Write;
use crate::parser::parse;

pub fn repl() {
    println!("Modu REPL");

    let context = &mut crate::utils::create_context();
    
    let mut history: Vec<String> = Vec::new();
    let mut open_functions = 0;
    let mut input = String::new();

    loop {
        if open_functions > 0 {
            print!("|{}", " ".repeat(open_functions * 4));
        } else {
            input.clear();
            print!("> ");
        }

        std::io::stdout().flush().unwrap();

        let mut this_input = String::new();
        std::io::stdin().read_line(&mut this_input).unwrap();
        history.push(input.clone());

        if this_input.contains("{") {
            open_functions += 1;
        }

        if this_input.contains("}") {
            let _ = open_functions.saturating_sub(1);
        }

        input.push_str(&this_input);

        if open_functions == 0 {
            parse(&input, "<repl>", context);
        }
    }
}