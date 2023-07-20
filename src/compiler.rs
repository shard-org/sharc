use crate::parser::{Data, Token};
use crate::utils::{logger, Level, At};

#[macro_export]
macro_rules! fmtln {
    ($ln:ident, $msg:expr) => {
        &format!("{}: {}", $ln, $msg)
    };
}

trait SafeRemove<Token> {
    fn next(&mut self) -> Option<Token>;
}

impl<Token> SafeRemove<Token> for Vec<Token> {
    fn next(&mut self) -> Option<Token> {
        if self.is_empty() {
            return None;
        }

        Some(self.remove(0))
    }
}

pub fn compiler(mut tokens: Vec<Data>, debug: bool) -> Result<String, ()> {
    // TODO change the text field to a &str, prob by implementing a method
    let mut e: bool = false;             // error bool
    let mut o: String = String::new();   // output str
    let mut inc: Option<String> = None;  // include files str
    let a = At::Compiler;

    while let Some(data) = tokens.next() {
        let ln = data.line;
        match data.token {
            Token::Directive => match data.text.as_str() {
                "use" => {
                    logger(Level::Err, &a, fmtln!(ln, "Nested Includes aren't Yet Supported!\nIf you Want this Feature, please donate to this project.")); 
                    e = true;
                    continue;
                },
                _ => (),
            },
            Token::Marker => {
                if data.scope.is_none() {
                    logger(Level::Err, &a, fmtln!(ln, "Markers Must be Within a Scope"));
                    e = true;
                    continue;
                }

                o.push_str(&format!("{}:\n", data.text));
            }
            _ => (),
        }

    }

    if debug {
        o.split('\n').for_each(|l| logger(Level::Debug, &a, l));
    }

    if e { return Err(()); }

    todo!();
    Ok(o)
}

