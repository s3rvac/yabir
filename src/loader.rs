//! Loading of programs into memory.

use std::fs::File;
use std::io::Read;

///
/// Loads a program from the given path into memory.
///
/// # Arguments
///
/// * `path` - Path to the program.
///
pub fn load_prog(path: &String) -> Result<String, String> {
    let mut f = try!(File::open(path)
        .map_err(|err| err.to_string())
    );
    let mut s = String::new();
    try!(f.read_to_string(&mut s)
        .map_err(|err| err.to_string())
    );
    Ok(s)
}
