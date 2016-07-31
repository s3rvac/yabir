pub mod usage;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if usage::should_print_usage(&args) {
        usage::print_usage();
        std::process::exit(0);
    }
}
