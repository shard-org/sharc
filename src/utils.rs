use super::*;

use std::fs;

pub fn reader<T>(filename: T) -> String 
where
    T: AsRef<std::path::Path> + std::fmt::Display + Copy,
{
    // Check if the file exists
    if fs::metadata(filename).is_err() {
        log!(FATAL, "File {} does not exist", filename);
        unreachable!();
    }

    // read the file
    let Ok(file) = fs::read_to_string(filename) else {
        log!(FATAL, "Could not read file {}", filename);
        unreachable!();
    };

    // Check if it's empty
    if file.replace(char::is_whitespace, "").is_empty() {
        log!(FATAL, "File {} is empty", filename);
        unreachable!();
    } file
}

pub fn writer(filename: &str, contents: &str) {
    // Write the file
    if let Err(e) = fs::write(filename, contents) {
        log!(FATAL, "Could not write to file {}: {e}", filename);
        unreachable!();
    }
}
