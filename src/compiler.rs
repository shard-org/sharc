use crate::parser::{Data, Token};
use crate::utils::{logger, Level, At};

macro_rules! fmtln {
    ($ln:ident, $msg:expr) => {
        &format!("{}| {}", $ln, $msg)
    };
}

pub fn compiler(tokens: Vec<Data>, debug: bool) -> Result<String, ()> {
    // TODO change the text field to a &str, prob by implementing a method
    let mut e: bool = false;             // error bool
    let mut o: String = String::new();   // output str
    let mut inc: Option<String> = None;  // include files str
    let a = At::Compiler;

    while let Some(data) = tokens.iter().next() {
        let ln = data.line;
        match data.token {
            Token::Directive => match data.text.as_str() {
                "use" => {
                    let mut fname = match tokens.iter().next().unwrap().text.trim().strip_suffix(".ox") {
                        Some(f) => f.to_string(),
                        None => { 
                            logger(Level::Err, &a, fmtln!(ln, "Filenames not ending with `.ox` are currently not Supported\nIf you Want this Feature, please File an Issue in the Github Repo.")); 
                            e = true;
                            continue;
                        }
                    };

                    // TODO: Implement linking files
                    // TODO: Compile into multiple asm files, and have ld link em
                    fname.replace_range(fname.len()-2.., "asm");
                    o.push_str(&format!(".include {}", fname));

                    todo!();
                },
                _ => todo!(),
            },
            _ => todo!(),
        }

    }

    if debug {
        eprintln!("{o}");
    }

    if e { return Err(()); }

    todo!();
    Ok(o)
}

