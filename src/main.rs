pub mod cli;
pub mod lexer;
pub mod loader;

fn main() {
    let args = cli::get_args();
    if cli::should_print_usage(&args) {
        cli::print_usage();
        std::process::exit(0);
    }

    let prog_path = &args[0];
    let prog = loader::load_prog(&prog_path).unwrap();
    let prog_tokens = lexer::tokenize(&prog);
}
