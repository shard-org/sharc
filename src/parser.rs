use crate::utils::*;

#[derive(Debug)]
pub struct Data {
    pub line: usize,
    pub file: String,
    pub token: Token,
    pub text: String,
    pub scope: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Token {
    Directive,
    SubroutineDef, // FIXME dont wrap everything in the definition
    Logic(),
    Argument,
    HeaderArg,
    VariableDef(),
    Return,
    Marker,
    Operator(),
    Bracket,
    Macro,
    Subroutine(),
    ExtExport,
}

impl Data {
    fn new(token: Token, line: &usize, file: &str, scope: &Option<String>, text: &str) -> Self {
        Data {
            line: line.to_owned(),
            file: file.to_string(),
            token,
            text: text.to_string(),
            scope: scope.to_owned(),
        }
    }
}

#[macro_export]
macro_rules! err {
    ($e:ident) => {
        $e += 1;
        continue
    };
}

pub fn parser(file_contents: String, debug: bool) -> Result<Vec<Data>, usize> {
    let mut d: Vec<Data> = Vec::new();
    let mut scope: Option<String> = None;
    let mut f: &str = "";   // current file
    let mut e: usize = 0;   // num of errors
    let a = At::Parser;

    for (i, s) in file_contents.lines().enumerate() {
        let s = s.trim();

        if s.is_empty() || s.starts_with("//") { continue; }

        else if s.chars().any(|c| !c.is_ascii()) {
            logger(Level::Err, &a, &logfmt(&i, f, "Only ASCII characters allowed for now!"));
            err!(e);
        }

        // handle the precompiler flags
        else if let Some(file) = s.strip_prefix("; @FILENAME ") {
            f = file;
            continue;
        }

        // directives
        // TODO: move all of these to the precompiler
        else if s.starts_with('.') {
            let (dir, args) = match s.split_once(' ') {
                Some(s) => s,
                None => {
                    logger(Level::Err, &a, &logfmt(&i, f, "Directive Missing an Argument!"));
                    err!(e);
                },
            };

            let dir_type = match dir.get(0..4) {
                Some(dir) => match &dir[1..] {
                    "use" | "def" | "mac" | "ent" => &dir[1..], // skip the trailing period
                    &_ => {
                        logger(Level::Err, &a, &logfmt(&i, f, &format!("Invalid Directive `{}`", dir)));
                        err!(e);
                    },
                },
                None => {
                    logger(Level::Err, &a, &logfmt(&i, f, "Expected a Directive!"));
                    err!(e);
                },
            };

            d.push(Data::new(Token::Directive, &i, f, &scope, &dir_type[1..]));
            d.push(Data::new(Token::Argument, &i, f, &scope, args));
        }

        // markers
        else if s.starts_with('@') {
            if s.len() <= 1 {
                logger(Level::Err, &a, &logfmt(&i, f, "Marker Needs an Identifier"));
                err!(e);
            }

            let s = &s[1..];
            if !validate_str(s) {
                logger(Level::Err, &a, &logfmt(&i, f, "Invalid Character"));
                err!(e);
            }

            d.push(Data::new(Token::Marker, &i, f, &scope, s));
        }

        // subroutine defs
        else if s.chars().next().unwrap().is_alphabetic() && scope.is_none() {
            let s: Vec<&str> = s.split_whitespace().collect();
            let (ident, vars) = match s[0].split_once('<') {
                Some(v) => v,
                None => {
                    logger(Level::Err, &a, &logfmt(&i, f, &format!("Unrecognized Token `{}`", s[0])));
                    err!(e);
                },
            };

            if !validate_str(ident) {
                logger(Level::Err, &a, &logfmt(&i, f, "Invalid Character"));
                err!(e);
            }

            if !s.last().unwrap().ends_with('{') {
                logger(Level::Err, &a, &logfmt(&i, f, "Expected an Opening Bracket"));
                err!(e);
            }

            d.push(Data::new(Token::SubroutineDef, &i, f, &scope, ident));

            let vars: Vec<&str> = vars[0..(vars.len() - 1)]
                .split(',')
                .map(|a| a.trim())
                .filter(|&e| !e.is_empty())
                .collect();

            if !vars.is_empty() {
                vars.iter().for_each(|v| d.push(Data::new(Token::Argument, &i, f, &scope, v)));
            }

            if s.len() > 2 {
                for arg in &s[1..(s.len())] {
                    d.push(Data::new(Token::HeaderArg, &i, f, &scope, &arg.replace('{', "")));
                }
            }

            if !s[0].contains('<') {
                scope = Some(s[0].to_string());
                continue;
            }

            scope = Some(ident.to_string());
        }

        // scope down
        else if s == "}" { 
            scope = match scope {
                Some(_) => None,
                None => { 
                    logger(Level::Err, &a, &logfmt(&i, f, "Unmatched Bracket"));
                    err!(e);
                },
            }
        }

        else if s.chars().next().unwrap().is_alphabetic() && scope.is_some() {
            let s: Vec<&str> = s.split_whitespace().collect();

            if !validate_str(s[0]) { 
                logger(Level::Err, &a, &logfmt(&i, f, "Invalid Character"));
                err!(e);
            }
        }
         
        else if s.starts_with("*") {
            // FIXME: prob split before that operator, leavin it to the daisy chain func
            let (export, args) = match s.split_once("") {
                Some(ex) => ex,
                None => {
                    logger(Level::Err, &a, &logfmt(&i, f, "Expected a Directional Operator!"));
                    err!(e);
                },
            };

            if !(export == "stdout" || export == "stderr") {
                logger(Level::Err, &a, &logfmt(&i, f, &format!("Unknown External Export `{s}`")));
                err!(e);
            }

            d.push(Data::new(Token::ExtExport, &i, f, &scope, export))

            // TODO: call the daisy chain func here
        }

        else { 
            logger(Level::Err, &a, &logfmt(&i, f, &format!("Unrecognized Token `{s}`")));
            err!(e);
        }
    }

    if let Some(s) = scope {
        logger(Level::Err, &a, &format!("Unmached Delimiter for Subroutine `{}`", s));
        e += 1;
    }

    if debug {
        d.iter().for_each(|d| 
            logger(Level::Debug, &a, &format!("{d:?}"))
        );
    }

    if e != 0 { 
        return Err(e); 
    }

    Ok(d)
}

fn validate_str(s: &str) -> bool {
    let s = s.trim();

    if s == "stdout" || s == "stderr" ||
        s.chars().any(|c| !(c.is_ascii_alphabetic() || c == '_') || 
        (c.is_uppercase() && s.chars().any(|pc| pc.is_lowercase()))
    ) {
        return false;
    }

    true
}
