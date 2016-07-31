//! Loading of programs into memory.

use std::io::prelude::*;
use std;

///
/// Loads a program from the given path into memory.
///
/// # Arguments
///
/// * `path` - Path to the program.
///
pub fn load_prog<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    let mut f = try!(std::fs::File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}
