//
// Project:   yabir
// Copyright: (c) 2016 by Petr Zemek <s3rvac@gmail.com> and contributors
// License:   Apache 2.0 or MIT, see the README file for more details
//

//! Command-line interface.

use std::env;

///
/// Returns program arguments, excluding program name.
///
pub fn get_args() -> Vec<String> {
    env::args().skip(1).collect()
}

///
/// Should program usage be printed?
///
/// # Arguments
///
/// * `args` - Program arguments, excluding program name.
///
pub fn should_print_usage(args: &Vec<String>) -> bool {
    args.len() != 1 || args[0] == "-h" || args[0] == "--help"
}

///
/// Prints program usage to the standard output.
///
pub fn print_usage() {
    println!("usage: yabir PROG");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_print_usage_returns_true_on_no_arguments() {
        assert!(should_print_usage(&vec![]));
    }

    #[test]
    fn test_should_print_usage_returns_true_on_help_short_form() {
        assert!(should_print_usage(&vec!["-h".to_string()]));
    }

    #[test]
    fn test_should_print_usage_returns_true_on_help_long_form() {
        assert!(should_print_usage(&vec!["--help".to_string()]));
    }

    #[test]
    fn test_should_print_usage_returns_false_when_prog_is_given() {
        assert!(!should_print_usage(&vec!["prog.bf".to_string()]));
    }
}
