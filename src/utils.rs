use super::*;
use std::fs;
use std::process::exit;

pub fn open<T>(filename: T) -> fs::File 
where
    T: AsRef<std::path::Path> + std::fmt::Display + Copy,
{
    debug!("Opening file: `{}`", filename);

    // read the file
    match fs::File::open(filename){
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
pub fn parse_int(st: String) -> Result<usize, Box<dyn Error>> {
    let mut chars = st.chars();
    match chars.next() {
        Some('0') => {
            let num = match chars.next() {
                Some('b') => usize::from_str_radix(&chars.collect::<String>(), 2)?,
                Some('o') => usize::from_str_radix(&chars.collect::<String>(), 8)?,
                Some('x') => usize::from_str_radix(&chars.collect::<String>(), 16)?,
                Some(_) => st.parse::<usize>()?,
                None => 0,
            }; Ok(num)
        },
        Some(_) => Ok(st.parse::<usize>()?),
        None => Err("Empty?!".into()),
    }
}





pub trait MapIf {
    fn map_if<F>(self, f: F) -> Option<Self> 
    where Self: Sized, F: FnOnce(&Self) -> bool;
}

impl<T> MapIf for T {
    fn map_if<F: FnOnce(&T) -> bool>(self, f: F) -> Option<Self> {
        if f(&self) { 
            return Some(self);
        } None
    }
}
