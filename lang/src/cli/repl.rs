
use rustyline::highlight::{Highlighter, CmdKind};
use rustyline::error::ReadlineError;
use rustyline::{Editor, history::DefaultHistory, Helper, Completer, Hinter, Validator};
use regex::Regex;
use colored::Colorize;
use std::collections::HashMap;
use crate::parser::parse;

#[derive(Completer, Helper, Hinter, Validator)]
pub struct Syntax {
    keyword_re: Regex,
    string_re: Regex,
    comment_re: Regex,
    number_re: Regex,
    boolean_re: Regex,
    function_re: Regex,
    compare_re: Regex,
    math_re: Regex,
}

impl Syntax {
    pub fn new() -> Self {
        Self {
            keyword_re: Regex::new(r"\b(if|else|fn|let|import|as|return|loop|break|continue|for|while|and|or|in|not in)\b").unwrap(),
            string_re: Regex::new(r#""([^"\\]|\\.)*"|'([^'\\]|\\.)*'"#).unwrap(),
            comment_re: Regex::new(r"//.*$|/\*.*?\*/").unwrap(),
            number_re: Regex::new(r"\b\d(?:_?\d)*\b").unwrap(),
            boolean_re: Regex::new(r"\b(true|false|null)\b").unwrap(),
            function_re: Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)(\()").unwrap(),
            compare_re: Regex::new(r"(==|!=|<=|>=|<|>|!)").unwrap(),
            math_re: Regex::new(r"(\+|-|\*|/|%)").unwrap(),
        }
    }
}

impl Highlighter for Syntax {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        let mut result = line.to_string();

        result = self.keyword_re.replace_all(&result, |caps: &regex::Captures| {
            format!("\x1b[1m{}\x1b[0m", caps[0].magenta()) // bug that with .bold().magenta() it didnt work, but this works
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

        result = self.comment_re.replace_all(&result, |caps: &regex::Captures| {
            caps[0].dimmed().to_string()
        }).to_string();

        result = self.compare_re.replace_all(&result, |caps: &regex::Captures| {
            caps[0].cyan().to_string()
        }).to_string();

        result = self.math_re.replace_all(&result, |caps: &regex::Captures| {
            caps[0].cyan().to_string()
        }).to_string();

        result = self.string_re.replace_all(&result, |caps: &regex::Captures| {
            caps[0].green().to_string()
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

    let mut rl: Editor<Syntax, DefaultHistory> = Editor::new().unwrap();
    rl.set_helper(Some(Syntax::new()));

    let mut open_functions = 0;
    let mut buffer = String::new();

    let mut globals: HashMap<String, crate::vm::value::Value> = HashMap::new();
    for func in crate::functions::get_functions() {
        globals.insert(func.name.clone(), crate::vm::value::Value::BuiltinFn(func));
    }

    let mut persistent_chunks: Vec<crate::vm::chunk::Chunk> = Vec::new();

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
                    let ast = parse(&buffer, "<repl>");
        
                    if let Err(_) = ast {
                        continue;
                    }

                    let mut compiler = crate::compiler::compiler::Compiler::new();
                    compiler.offset = persistent_chunks.len();
                    
                    if let Err(e) = compiler.compile_program(ast.clone().unwrap()) {
                        println!("{}: {}", "Compilation error".red(), e);
                        continue;
                    }

                    let mut all_chunks = persistent_chunks.clone();
                    all_chunks.extend(compiler.chunks.into_iter());

                    let mut vm = crate::vm::vm::VM::new(all_chunks.clone(), std::path::PathBuf::from("<repl>"), buffer.clone());
                    vm.globals = globals.clone();

                    buffer.clear();

                    if let Err(e) = vm.run(persistent_chunks.len()) {
                        println!("{}", e);
                        continue;
                    }

                    globals = vm.globals.clone();
                    persistent_chunks = vm.chunks;
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