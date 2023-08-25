use std::process::exit;
use std::path::{Path, PathBuf};
use std::fs;

use crate::logger::{logger, Level, Debug};
use crate::logerr;

const DBGR: &Debug = &Debug::Reader;
pub fn reader(filename: &str) -> String {
    // Check if the file exists
    if fs::metadata(filename).is_err() {
        logerr!(DBGR, format!("File {} does not exist", filename));
        exit(1);
    }

    // read the file
    let Ok(file) = fs::read_to_string(filename) else {
        logerr!(DBGR, format!("Could not read file {}", filename));
        exit(1);
    };

    // Check if it's empty
    if file.replace(char::is_whitespace, "").is_empty() {
        logerr!(DBGR, format!("File {} is empty", filename));
        exit(1);
    } file
}

pub fn rec_reader(path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    let entries = fs::read_dir(path).unwrap_or_else(|_| {
        logerr!(DBGR, format!("Could not read Directory {:?}", path));
        dbg!(path);
        exit(1);
    });

    for entry in entries {
        // FIXME dont unwrap
        let path = entry.unwrap().path();

        if path.is_dir() {
            files.append(&mut rec_reader(&path));
            continue;
        }

        files.push(path);
    } files
}

pub fn read_dir(path: &Path) -> Vec<PathBuf> {
    let entries = fs::read_dir(path).unwrap_or_else(|_| {
        logerr!(DBGR, format!("Could not Read {:?}", path));
        exit(1);
    });

    entries.map(|e| e.unwrap().path()).collect::<Vec<PathBuf>>()
}

const DBGW: &Debug = &Debug::Writer;
pub fn writer(filename: &str, contents: &str) {
    // Write the file
    if let Err(e) = fs::write(filename, contents) {
        logerr!(DBGW, format!("Could not write to file {}: {e}", filename));
        exit(1);
    }
}
