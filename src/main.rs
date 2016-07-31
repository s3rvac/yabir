pub mod cli;

fn main() {
    let args = cli::get_args();
    if cli::should_print_usage(&args) {
        cli::print_usage();
        std::process::exit(0);
    }

    let prog_path = &args[0];
}
