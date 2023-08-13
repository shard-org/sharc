use crate::parser::{Data, Token};
use crate::utils::*;
use crate::err;

const A: &At = &At::Compiler;

#[macro_export]
macro_rules! fmtln {
    ($ln:ident, $msg:expr) => {
        format!("{}: {}", $ln, $msg)
    };
}

trait Stuff<Token> {
    fn next(&mut self) -> Option<Token>;
}

impl<Token> Stuff<Token> for Vec<Token> {
    fn next(&mut self) -> Option<Token> {
        if self.is_empty() {
            return None;
        }

        Some(self.remove(0))
    }
}

pub fn compiler(mut tokens: Vec<Data>, debug: bool) -> Result<String, usize> {
    let mut e: usize = 0;               // err count
    let mut o: String = String::new();  // output str

    while let Some(data) = tokens.next() {
        let f = data.file; // f is the file
        let ln = data.line;
        match data.token {
            Token::Marker => {
                if data.scope.is_empty() {
                    logger(Level::Err, A, fmtln!(ln, "Markers Must be Within a Scope"));
                    err!(e);
                }

                o.push_str(&format!("{}:\n", data.text));
            },
            _ => (),
        }

    }

    if debug {
        o.split('\n').for_each(|l| logger(Level::Debug, A, l));
    }

    if e != 0 { return Err(e); }

    todo!();
    Ok(o)
}

