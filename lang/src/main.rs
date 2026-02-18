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
    server  [port] - DEPRECATED
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

        let bt = std::backtrace::Backtrace::capture();

        if bt.status() == std::backtrace::BacktraceStatus::Captured {
            eprintln!("  ├─ Backtrace:");
            eprintln!("{}", format!("{bt:?}").dimmed());
        } else {
            eprintln!("  ├─ {}", "Run with RUST_BACKTRACE=1 for more details".dimmed());
        }
        
        eprintln!("  └─ Please report this issue at https://github.com/cyteon/modu/issues");
    }
}