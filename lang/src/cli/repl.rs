
use rustyline::highlight::{Highlighter, CmdKind};
use rustyline::error::ReadlineError;
use rustyline::{Editor, history::DefaultHistory, Helper, Completer, Hinter, Validator};
use regex::Regex;
use colored::Colorize;
use crate::parser::parse;

#[derive(Completer, Helper, Hinter, Validator)]
pub struct Syntax {
    keyword_re: Regex,
    string_re: Regex,
    comment_re: Regex,
    number_re: Regex,
    boolean_re: Regex,
    function_re: Regex,
}

impl Syntax {
    pub fn new() -> Self {
        Self {
            keyword_re: Regex::new(r"\b(if|else|fn|let|import|as|return|loop|break|continue|for|in|not in)\b").unwrap(),
            string_re: Regex::new(r#""([^"\\]|\\.)*"|'([^'\\]|\\.)*'"#).unwrap(),
            comment_re: Regex::new(r"//.*$|/\*.*?\*/").unwrap(),
            number_re: Regex::new(r"\b\d(?:_?\d)*\b").unwrap(),
            boolean_re: Regex::new(r"\b(true|false|null)\b").unwrap(),
            function_re: Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)(\()").unwrap(),
        }
    }
}

impl Highlighter for Syntax {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        let mut result = line.to_string();

        result = self.keyword_re.replace_all(&result, |caps: &regex::Captures| {
            format!("\x1b[1m{}\x1b[0m", caps[0].magenta()) // bug that with .blue().magenta() it didnt work, but this works
        }).to_string();

        result = self.number_re.replace_all(&result, |caps: &regex::Captures| {
            caps[0].yellow().to_string()
        }).to_string();

        result = self.boolean_re.replace_all(&result, |caps: &regex::Captures| {
            caps[0].red().to_string()
        }).to_string();

        result = self.function_re.replace_all(&result, |caps: &regex::Captures| {
            format!("{}{}", caps[1].blue().to_string(), &caps[2])
        }).to_string();

        result = self.string_re.replace_all(&result, |caps: &regex::Captures| {
            caps[0].green().to_string()
        }).to_string();

        result = self.comment_re.replace_all(&result, |caps: &regex::Captures| {
            caps[0].dimmed().to_string()
        }).to_string();

        std::borrow::Cow::Owned(result)
    }

    fn highlight_prompt<'a, 'b: 'a, 'p: 'a>(
        &self,
        prompt: &'p str,
        _default: bool,
    ) -> std::borrow::Cow<'a, str> {
        std::borrow::Cow::Borrowed(prompt)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _cmd: CmdKind) -> bool {
        true
    }
}

pub fn repl() {
    println!("Modu REPL");

    let context = &mut crate::utils::create_context();
    let mut rl: Editor<Syntax, DefaultHistory> = Editor::new().unwrap();
    rl.set_helper(Some(Syntax::new()));

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