pub mod args;

use std::io::{stdin, stdout, Write};
use std::num::ParseIntError;

pub fn get_stdin_number() -> Result<usize, ParseIntError> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().parse::<usize>()
}

/// Get a confirmation input from stdin, that supports default value on pressing enter.
/// `default_value` can either be:
/// - `None` if you want to disable enter for default option
/// - `true` and `false` for default value to those
pub fn get_stdin_confirm(default_value: Option<bool>) -> bool {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_lowercase();
    if input == "y" { return true; } // only accepts y for yes
    // default case
    if default_value.is_some() && input.is_empty() { return default_value.unwrap(); }
    // otherwise no
    false
}

pub fn flush_stdout() {
    stdout().flush().unwrap()
}