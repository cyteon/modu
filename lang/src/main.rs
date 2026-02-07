#![feature(backtrace_frames)]

use std::panic::{catch_unwind, AssertUnwindSafe};
use colored::Colorize;

mod ast;
mod eval;
mod lexer;
mod parser;
mod cli;
mod utils;
mod libraries;
mod builtins;

fn main() {
    std::panic::set_hook(Box::new(|_| {}));

    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Commands:
    run     <file> - Run a Modu file
    repl           - Start the Modu REPL
    server  [port] - Start the Modu server, default port is 2424
    init           - Initialize a new Modu package
    login          - Login with Modu Packages
    publish        - Publish a Modu package
    install <name> - Install a Modu package
    uninstall <name> - Uninstall a Modu package");
        return;
    }

    let action = &args[1];

    let result = catch_unwind(AssertUnwindSafe(|| {
        match action.as_str() {
            "run" => cli::run::run(),
            "repl" => cli::repl::repl(),
            "server" => cli::server::server(),
            "login" => cli::login::login(),
            "init" => cli::init::init(),
            "publish" => cli::publish::publish(),
            "install" => cli::install::install(),
            "uninstall" => cli::uninstall::uninstall(),
            "--version" => {
                println!("Modu v{}", env!("CARGO_PKG_VERSION"));
            }

            action => {
                println!("Unknown command: {}", action);
            }
        }
    }));

    if let Err(panic) = result {
        let msg = panic
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| panic.downcast_ref::<String>().map(String::as_str))
            .unwrap_or("Unknown internal error");
        
        eprintln!("{}", "Internal interpreter error".red().bold());
        eprintln!("  ├─ {}", msg.yellow());
        if cfg!(debug_assertions) {
            let bt = std::backtrace::Backtrace::capture();
            
            if bt.frames().is_empty() {
                eprintln!("  ├─ {}", "Run with RUST_BACKTRACE=1 to see a backtrace".dimmed());
            } else {
                eprintln!("  ├─ Backtrace:");
                for line in bt.frames().iter() {
                    eprintln!("  │   {}", format!("{:?}", line).dimmed());
                }
            } 
        }
        eprintln!("  └─ {}", "Please report this issue at https://github.com/cyteon/modu/issues".dimmed());
    }
}