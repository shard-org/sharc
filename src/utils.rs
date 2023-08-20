use std::process::exit;
use std::path::{Path, PathBuf};
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
    let Ok(file) = fs::read_to_string(filename) else {
        logger(Level::Err, None, DBGR, format!("Could not read file {}", filename));
        exit(1);
    };

    // Check if it's empty
    if file.replace(char::is_whitespace, "").is_empty() {
        logger(Level::Warn, None, DBGR, format!("File {} is empty", filename));
        exit(1);
    }

    file
}

pub fn rec_reader(path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let entries = fs::read_dir(path).unwrap_or_else(|_| {
        logger(Level::Err, None, DBGR, "Could not read Project Directory");
        exit(1);
    });

    for entry in entries {
        // FIXME dont unwrap
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            files.append(&mut rec_reader(&path));
            continue;
        }

        files.push(path);
    }

    files
}

static DBGW: &Debug = &Debug::Writer;
pub fn writer(filename: &str, contents: &str) {
    // Write the file
    if let Err(e) = fs::write(filename, contents) {
        logger(Level::Err, None, DBGW, format!("Could not write to file {}: {e}", filename));
        exit(1);
    }
}
