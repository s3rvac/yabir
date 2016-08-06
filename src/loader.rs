//
// Project:   yabir
// Copyright: (c) 2016 by Petr Zemek <s3rvac@gmail.com> and contributors
// License:   Apache 2.0 or MIT, see the README file for more details
//

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
pub fn load_prog(path: &String) -> Result<String, String> {
    let mut f = try!(std::fs::File::open(path)
        .map_err(|err| err.to_string())
    );
    let mut s = String::new();
    try!(f.read_to_string(&mut s)
        .map_err(|err| err.to_string())
    );
    Ok(s)
}
