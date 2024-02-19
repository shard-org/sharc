use super::*;
use std::fs;
use std::process::exit;

pub fn reader<T>(filename: T) -> String 
where
    T: AsRef<std::path::Path> + std::fmt::Display + Copy,
{
    debug!("Reading file: {}", filename);

    // Check if the file exists
    if fs::metadata(filename).is_err() {
        fatal!("File {} does not exist", filename);
        exit(1);
    }

    // read the file
    let Ok(file) = fs::read_to_string(filename) else {
        fatal!("Could not read file {}", filename);
        exit(1);
    };

    // Check if it's empty
    if file.replace(char::is_whitespace, "").is_empty() {
        fatal!("File {} is empty", filename);
        exit(1);
    } file
}

pub fn writer(filename: &str, contents: &str) {
    debug!("Writing file: {}", filename);

    // Write the file
    if let Err(e) = fs::write(filename, contents) {
        fatal!("Could not write to file {}: {e}", filename);
        exit(1);
    }
}
