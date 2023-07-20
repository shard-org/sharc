use crate::parser::{Data, Token};

macro_rules! get_or_err {
    ($val:expr, $err:expr) => {
        match $val {
            Ok(v) => v,
            Err(_) => return Err($err),
        }
    };
}

pub fn compiler(tokens: Vec<Data>, debug: bool) -> Result<String, Vec<String>> {
    // TODO change the text field to a &str, prob by implementing a method
    let mut e: Vec<String> = Vec::new();
    let mut o: String = String::new();

    for data in tokens {
        match data.token {
            Token::Directive => match data.text {
                "use" => {
                        let filename = match data.next() {
                            Some(dat) => dat,
                            None => errfmt!(e, )
                        }

                        // TODO: Implement linking files
                        // TODO: Compile into multiple asm files, and have ld link em
                        o.push_str(format!(".include {}", ));

                        todo!();
                    }
                }
            }

            _ => todo!(),
        }

    }

    if debug {
        eprintln!("{o}");
    }

    todo!();

    if !e.is_empty() { return Err(e); }

    Ok(o);
}

