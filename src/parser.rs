use crate::utils::*;

// TODO: clean this up to be more efficient
// maybe for the scope provide a list of ranges, scope doesn't change every line
// same with file ^^^^
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
    Subroutine,
    ExtExport(),
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

macro_rules! test {
    ($chk:expr, $g:ident) => {
        match $chk {
            Ok(e) => e,
            Err(b) => {
                let (i, f, mut e) = $g;
                logger(Level::Err, &At::Parser, &logfmt(i, f, &b));
                e += 1;
                continue
            }
        }
    };
}

// this is just to help my obsession with shortening everything
// hate useless verbocity, like `.to_string()`, 12 chars for a really common method?!?
macro_rules! st {
    ($cnv:expr) => {
        $cnv.to_string()
    };
} 

macro_rules! testo {
    ($chk:expr, $err:block) => {
        match $chk {
            Some(e) => e,
            None => {
                $err
            }
        }
    };
}

macro_rules! bail {
    ($expr:expr) => {
        return Err($expr)
    };
}

// if we apply the todos above the out of this func would be:
// Result<(Vec<usize>, Vec<String>, Vec<Data>), usize>
// or we might wanna use a struct..? idk
pub fn parser(file_contents: String, debug: bool) -> Result<Vec<Data>, usize> {
    let mut d: Vec<Data> = Vec::new();
    let mut scope: Option<String> = None;
    let mut f: &str = "";   // current file
    let mut e: usize = 0;   // num of errors
    let a = At::Parser;

    for (i, s) in file_contents.lines().enumerate() {
        let mut s = s.trim();
        let g = (&i, f, e);

        //
        // comments section
        if s.is_empty() || s.starts_with("//") { continue; }

        if let Some(pos) = s.find("//") {
            s = s[..pos];
        }


        if s.chars().any(|c| !c.is_ascii()) {
            // originally to save mem, dunno if thats actually needed
            logger(Level::Err, &a, &logfmt(&i, f, "Only ASCII characters allowed for now!"));
            err!(e);
        }

        // handle file changes
        // FIXME: this clearly isn't *the way*, but yeah
        else if let Some(file) = s.strip_prefix("; @FILENAME ") {
            f = file;
            continue;
        }

        // directives
        else if let Some(s) = s.strip_prefix(".") {
            let (dir, args) = test!(parse_directive(s), g);

            d.push(Data::new(Token::Directive, &i, f, &scope, dir));
            d.push(Data::new(Token::Argument, &i, f, &scope, args));
        }

        // markers
        else if let Some(s) = s.strip_prefix("@") {
            d.push(Data::new(Token::Marker, &i, f, &scope, test!(parse_marker(s), g)));
        }

        // subroutine defs
        // TODO: make the scope a stack, allowing for defs inside of defs
        // FIXME: the current code here is only to make the compiler shut up
        // replace with parse_subroutine_def(), defined below
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
        // TODO: prob have a func for the scope stack, (check todo above)
        else if s == "}" { 
            match scope {
                Some(_) => scope = None,
                None => { 
                    logger(Level::Err, &a, &logfmt(&i, f, "Unmatched Bracket"));
                    err!(e);
                },
            }
        }

        // TODO: finish this
        // subroutine calls
        else if s.chars().next().unwrap().is_alphabetic() && scope.is_some() {
            let s: Vec<&str> = s.split_whitespace().collect();

            if !validate_str(s[0]) { 
                logger(Level::Err, &a, &logfmt(&i, f, "Invalid Character"));
                err!(e);
            }
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

fn parse_marker(s: &str) -> Result<&str, String> {
    if s.is_empty() {
        bail!(st!("Marker Needs an Identifier"));
    }

    if !validate_str(s) {
        bail!(st!("Invalid Character"));
    }

    Ok(s)
}

fn parse_directive(s: &str) -> Result<(&str, &str), String> {
    let (dir, args) = testo!(s.split_once(' '), {
        bail!(st!("Directive Missing an Argument!"));
    });

    let dir = match dir {
        "use" | "def" | "mac" | "ent" => dir,
        &_ => bail!(format!("Invalid Directive `{}`", dir)),
    };

    Ok((dir, args))

}

// // FIXME: big todo, no idea how to do this
// fn parse_subroutine_def(s: &str) -> Result<&str, String> {
//     let s: Vec<&str> = s.split_whitespace().collect();
//     let (ident, vars) = testo!(s[0].split_once('<'), {
//         bail!(format!("Unrecognized Token `{}`", s[0]));
//     });
//
//     if !validate_str(ident) {
//         bail!(st!("Invalid Character"));
//     }
//
//     if !s.last().unwrap().ends_with('{') {
//         bail!(st!("Expected an Opening Bracket"));
//     }
//
//     d.push(Data::new(Token::SubroutineDef, &i, f, &scope, ident));
//
//     // i dont even rember what this does
//     let vars: Vec<&str> = vars[0..(vars.len() - 1)]
//         .split(',')
//         .map(|a| a.trim())
//         .filter(|&e| !e.is_empty())
//         .collect();
//
//     if !vars.is_empty() {
//         // FIXME: SOMEHOW IMPLEMENT THIS WITH THE NEW FUNC SYSTEM
//         // might return a Vec<Data> and append that to the `d` var?
//         vars.iter().for_each(|v| d.push(Data::new(Token::Argument, &i, f, &scope, v)));
//     }
//
//     if s.len() > 2 {
//         for arg in &s[1..(s.len())] {
//             // TODO: same as above
//             d.push(Data::new(Token::HeaderArg, &i, f, &scope, &arg.replace('{', "")));
//         }
//     }
//
//     // TODO: this is also a mistery... maybe have the whole func return
//     // Result<(Option<String>, Vec<Data>), String> ??!?!??!?
//     if !s[0].contains('<') {
//         scope = Some(s[0].to_string());
//         continue;
//     }
//
//     scope = Some(ident.to_string());
//
// }

// TODO: this is a mess... make it return the exact token that it failed on, prob Option<char>
fn validate_str(s: &str) -> bool {
    if s.chars().any(|c| 
        !(c.is_ascii_alphabetic() || c == '_') || 
        (c.is_uppercase() && s.chars().any(|pc| pc.is_lowercase()))
    ) {
        logger(Level::Warn, &At::Parser, "Use snake_case or ANGRY_SNAKE_CASE");
        return false;
    }

    true
}
