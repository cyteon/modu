#![feature(internal_output_capture)]

mod ast;
mod evaulator;
mod lexer;
mod parser;
mod cli;
mod utils;
mod functions;
mod libraries;
mod builtins;

fn main() {
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
            println!("Modu v1.3.0");
        }

        action => {
            println!("Unknown command: {}", action);
        }
    }
}