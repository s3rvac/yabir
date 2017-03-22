//! Yet Another Brainfuck Interpreter in Rust.

pub mod cli;
pub mod lexer;
pub mod loader;
pub mod parser;
pub mod runner;

use std::io::Write;

fn main() {
    let args = cli::get_args();
    if cli::should_print_usage(&args) {
        cli::print_usage();
        std::process::exit(0);
    }

    run_prog(&args[0]).unwrap_or_else(|err| {
        print_error(&err);
        std::process::exit(1);
    })
}

fn run_prog(prog_path: &String) -> Result<(), String> {
    let prog = loader::load_prog(&prog_path)?;
    let prog_tokens = lexer::tokenize(&prog);
    let prog_ops = parser::parse(&prog_tokens)?;
    let mut input = std::io::stdin();
    let mut output = std::io::stdout();
    runner::run(prog_ops, &mut input, &mut output)
}

fn print_error(err: &String) {
    writeln!(&mut std::io::stderr(), "error: {}", err)
        .expect("failed printing an error");
}
