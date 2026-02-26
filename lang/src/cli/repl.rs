
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use crate::parser::parse;

pub fn repl() {
    println!("Modu REPL");

    let context = &mut crate::utils::create_context();
    let mut rl = DefaultEditor::new().unwrap();

    let mut open_functions = 0;
    let mut buffer = String::new();

    loop {
        let prompt = if open_functions > 0 {
            format!("|{}", " ".repeat(open_functions * 4))
        } else {
            "> ".to_string()
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                if line.trim() == "exit" {
                    break;
                }

                rl.add_history_entry(line.as_str()).unwrap();
                
                open_functions += line.chars().filter(|&c| c == '{').count();
                open_functions = open_functions.saturating_sub(line.chars().filter(|&c| c == '}').count());

                buffer.push_str(&line);
                buffer.push('\n');

                if open_functions == 0 {
                    parse(&buffer, "<repl>", context);
                    buffer.clear();
                }
            }

            Err(ReadlineError::Interrupted) => {
                println!("^C");
                buffer.clear();
                open_functions = 0;
            }

            Err(ReadlineError::Eof) => {
                break;
            }

            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}