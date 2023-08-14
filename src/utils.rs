use std::process::exit;
use std::fs;
use crate::logger::{logger, Level, Debug};

static DBGR: &Debug = &Debug::Reader;
pub fn reader(filename: &str) -> String {
    // Check if the file exists
    if fs::metadata(filename).is_err() {
        logger(Level::Err, None, DBGR, format!("File {} does not exist", filename));
        exit(1);
    }

    // read the file
    let file = fs::read_to_string(filename).unwrap_or_else(|_| {
        logger(Level::Err, None, DBGR, format!("Could not read file {}", filename));
        exit(1);
    });

    // Check if it's empty
    if file.replace(char::is_whitespace, "").is_empty() {
        logger(Level::Warn, None, DBGR, format!("File {} is empty", filename));
        exit(1);
    }

    file
}

static DBGW: &Debug = &Debug::Writer;
pub fn writer(filename: &str, contents: &str) {
    // Write the file
    if let Err(e) = fs::write(filename, contents) {
        logger(Level::Err, None, DBGW, format!("Could not write to file {}: {e}", filename));
        exit(1);
    }
}
