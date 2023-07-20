use crate::utils::{logger, At, Level};

#[derive(Debug)]
pub struct Data {
    pub line: usize,
    pub token: Token,
    pub text: String,
    pub scope: Option<String>,
}

#[derive(Debug)]
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
    Subroutine,
}

// struct TokenIterator {
    // data
// }
//
// impl Iterator for TokenIterator {
//
// }

#[macro_export]
macro_rules! errfmt {
    ($i:ident, $errs:ident, $err:expr) => {
        $errs.push(format!("Line {}: {}", $i + 1, $err));
        continue
    };
    ($i:ident, $errs:ident, $err:expr, $spec:expr) => {
        $errs.push(format!("Line: {}: {} `{}`", $i + 1, $err, $spec));
        continue
    };
}

macro_rules! get_or_errfmt {
    ($val:expr, $i:ident, $errs:ident, $err:expr) => {
        match $val {
            Some(v) => v,
            None => {
                $errs.push(format!("Line {}: {}", $i + 1, $err));
                continue
            },
        }
    };
    ($val:expr, $i:ident, $errs:ident, $err:expr, $spec:expr) => {
        match $val {
            Some(v) => v,
            None => {
                $errs.push(format!("Line {}: {} `{}`", $i + 1, $err, $spec));
                continue
            },
        }
    };
}

macro_rules! push {
    ($data:ident, $ln:expr, $token:expr, $text:expr, $scope:expr) => {
        $data.push(Data {
            line: $ln + 1,
            token: $token,
            text: $text.to_string(),
            scope: $scope,
        })
    };
}

pub fn parser(file_contents: String, debug: bool) -> Result<Vec<Data>, Vec<String>> {
    if file_contents.chars().any(|c| !c.is_ascii()) {
        return Err(vec!["Only ascii Chars Allowed For Now".to_string()]);
    }

    let mut data: Vec<Data> = Vec::new();
    let mut scope: Option<String> = None;
    let mut e: Vec<String> = Vec::new();
    let a = At::Parser;

    for (i, s) in file_contents.lines().enumerate() {
        let s = s.trim();

        if s.starts_with("//") { continue; }

        else if s.is_empty() { continue; }

        else if s.starts_with('.') {
            let s = get_or_errfmt!(s.split_once(' '), i, e, "Directive missing an Argument!");

            let dir_type = match s.0.get(0..4) {
                Some(dir) => match &dir[1..] {
                    "use" | "def" | "mac" | "ent" => dir,
                    &_ => {
                        errfmt!(i, e, "Invalid Directive", s.0);
                    },
                },
                None => {
                    errfmt!(i, e, "Expected a Directive, Found", s.0);
                },
            };

            push!(data, i, Token::Directive, dir_type[1..], scope.clone());
            push!(data, i, Token::Argument, s.1, scope.clone());
        }

        else if s.starts_with('@') {
            if s.len() <= 1 {
                errfmt!(i, e, "Marker Needs an Identifier");
            }

            if !validate_str(&s[1..]) {
                errfmt!(i, e, "Unexpected Character");
            }

            push!(data, i, Token::Marker, s[1..], scope.clone());
        }

        else if s.chars().next().unwrap().is_alphabetic() && scope.is_none() {
            let s: Vec<&str> = s.split_whitespace().collect();
            let (ident, vars) = s[0].split_once('<').unwrap();

            if !validate_str(ident) {
                errfmt!(i, e, "Unexpected Character");
            }

            if !s.last().unwrap().ends_with('{') {
                errfmt!(i, e, "Expected an Opening Bracket");
            }

            push!(data, i, Token::SubroutineDef, ident, scope.clone());

            let vars: Vec<&str> = vars[0..(vars.len() - 1)]
                .split(',')
                .map(|a| a.trim())
                .filter(|&e| e != "")
                .collect();

            if !vars.is_empty() {
                vars.iter().for_each(|v| push!(data, i, Token::Argument, v, scope.clone()))
            }

            if s.len() > 2 {
                for arg in &s[1..(s.len())] {
                    push!(data, i, Token::HeaderArg, arg.replace('{', ""), scope.clone());
                }
            }

            if s[0].contains('<') {
                scope = Some(ident.to_string());
            } else {
                scope = Some(s[0].to_string());
            }
        }

        else if s == "}" { 
            match scope {
                Some(_) => scope = None,
                None => { errfmt!(i, e, "Unmatched Delimiter"); },
            }
        }

        else if s.chars().next().unwrap().is_alphabetic() && scope.is_some() {
            let s: Vec<&str> = s.split_whitespace().collect();

            if !validate_str(s[0]) { errfmt!(i, e, "Unexpected Character"); }
        }
         

        else { errfmt!(i, e, "Unrecognized Token", s); }
    }

    if let Some(s) = scope {
        e.push(format!("Unmached Delimiter for Subroutine `{}`", s));
    }

    if debug {
        data.iter().for_each(|d| 
            logger(Level::Debug, &a, &format!("{d:?}"))
        );
    }

    if !e.is_empty() { return Err(e); }

    Ok(data)
}

fn validate_str(s: &str) -> bool {
    if s.chars().any(|c| !(c.is_ascii_alphabetic() || c == '_')) {
        return false;
    }
    true
}
