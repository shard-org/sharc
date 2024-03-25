use super::*;
use std::fs;
use std::process::exit;

pub fn open<T>(filename: T) -> fs::File
where
    T: AsRef<std::path::Path> + std::fmt::Display + Copy,
{
    debug!("Opening file: `{}`", filename);

    // read the file
    match fs::File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            fatal!("Could not open `{}`: {}", filename, e);
            exit(1);
        },
    }
}

//
// parsing
use std::error::Error;
pub fn parse_int(mut st: String) -> Result<usize, Box<dyn Error>> {
    match st.pop() {
        Some('0') => {
            let num = match st.pop() {
                Some('b') => usize::from_str_radix(&st, 2)?,
                Some('o') => usize::from_str_radix(&st, 8)?,
                Some('x') => usize::from_str_radix(&st, 16)?,
                Some(_) => st.parse::<usize>()?,
                None => 0,
            };
            Ok(num)
        },
        Some(c) => Ok(format!("{}{}", c, st).parse::<usize>()?),
        None => Err("Empty".into()),
    }
}

pub trait MapIf {
    fn map_if<F>(self, f: F) -> Option<Self>
    where
        Self: Sized,
        F: FnOnce(&Self) -> bool;
}

impl<T> MapIf for T {
    fn map_if<F: FnOnce(&T) -> bool>(self, f: F) -> Option<Self> {
        if f(&self) {
            return Some(self);
        }
        None
    }
}
