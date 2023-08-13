use crate::utils::*;
use crate::{testo, st, bail};
use crate::defs::STD;

const A: &At = &At::Parser;

// TODO: clean this up to be more efficient
// maybe for the scope provide a list of ranges, scope doesn't change every line
// same with file ^^^^
#[derive(Debug)]
pub struct Data {
    pub line: usize,
    pub file: String,
    pub token: Token,
    pub text: String,
    pub scope: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Token {
    Directive, // TODO: pretty sure only the `ent` directive has to be passed to the compiler
    SubroutineDef, // FIXME dont wrap everything in the definition
    Argument,
    HeaderArg,
    VariableDef(),
    Return,
    Marker,
    Operator(),
    Operation(Vec<Op>),
    Bracket,
    // Macro,
    Subroutine,
    ExtExport(), // idk what this
    Import(Import),
}

#[derive(Debug, Clone)]
pub enum Op {
    Add(String), // +
    Sub(String), // -
    Mul(String), // *
    Div(String), // /
    Mod(String), // %

    Get(String), // >=
    Let(String), // <=
    Gtt(String), // >
    Ltt(String), // <
    Equ(String), // =
}

// prob add a utf-8 lib in the future
#[derive(Debug, Clone)]
pub enum Import {
    Std,
    Extern(String),
}

impl Data {
    fn new(token: Token, line: &usize, file: &str, scope: &[&str], text: &str) -> Self {
        Data {
            line: line.to_owned(),
            file: file.to_string(),
            token,
            text: text.to_string(),
            scope: scope.iter().map(|e| e.to_string()).collect::<Vec<String>>(),
        }
    }

    fn from_faux(faux_data: (Token, String), file: &str, scope: &[&str], line: &usize) -> Self {
        let (token, text) = faux_data;
        Data {
            line: line.to_owned(),
            file: file.to_string(),
            token,
            text,
            scope: scope.iter().map(|e| e.to_string()).collect::<Vec<String>>(),
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
                let (i, f, e) = $g;
                logger(Level::Err, &At::Parser, logfmt(i, f, b));
                *e += 1;
                continue
            }
        }
    };
}

    use std::thread;
    use std::time::Duration;

    fn break() {
        println!("Be Right Back!");
        Stream::pause();

        // eta: 5 minutes
        thread::sleep(Duration::from_secs(300));
        
        Stream::resume();
    }



// if we apply the todos above the out of this func would be:
// Result<(Vec<usize>, Vec<String>, Vec<Data>), usize>
// or we might wanna use a struct..? idk
pub fn parser(file_concat: String, debug: bool) -> Result<Vec<Data>, usize> {
    let mut line_stack: Vec<usize> = vec![1];
    let mut file_stack = Vec::new();            
    let mut e: usize = 0;              // total error count
    let mut d: Vec<Data> = Vec::new(); // data out
    let mut scope: Vec<&str> = vec![]; // scope stack

    for ln in file_concat.lines() {
        line_stack[0] += 1;
        let i = line_stack[0];
        let f = file_stack[0];
        let g = (&i, f, &mut e);  // abbrv tuple

        /*
         * filestuff logic
         */     
        if let Some(ln) = ln.strip_prefix("~~FILESTART") {
            file_stack.push(ln.trim());
            line_stack.push(1);
            continue;
        }

        if ln == "~~FILEEND" {
            file_stack.pop();
            line_stack.pop();
            continue;
        }

        if ln.starts_with("~~") {
            logger(Level::Warn, A, logfmt(&i, f, format!("Unknown Internal Directive `{ln}`, Skipping!")));
            continue;
        }

        /*
         * Ascii Filtering
         */
        if ln.chars().any(|c| !c.is_ascii()) {
            logger(Level::Err, A, logfmt(&i, f, "For now Only ASCII allowed"));
            err!(e);
        }

        /*
         * Actual Tokenizing
         */

        //
        // directives
        if let Some(ln) = ln.strip_prefix('.') {
            let (dir, args) = test!(parse_directive(ln), g);

            d.push(Data::new(Token::Directive, &i, f, &scope, dir));
            d.push(Data::new(Token::Argument, &i, f, &scope, args));
            continue;
        }


        //
        // markers
        if let Some(ln) = ln.strip_prefix('@') {
            if scope.is_empty() {
                d.push(Data::from_faux(test!(parse_import(ln), g), f, &scope, &i));
                continue;
            }

            d.push(Data::new(Token::Marker, &i, f, &scope, test!(parse_marker(ln), g)));
            continue;
        }


        //
        // subroutines
        if ln.chars().next().unwrap().is_alphabetic() {
            //
            // subroutine defs
            if ln.ends_with('{') {
                let (name, data) = test!(parse_subroutine_def(ln), g);

                data.iter().for_each(|e| d.push(Data::from_faux(e.clone(), f, &scope, &i)));
                scope.push(name);
                continue;
            }

            //
            //subroutine calls
            // TODO: finish this
            if !scope.is_empty() {
                test!(parse_subroutine(ln), g);
                continue;
            }

            logger(Level::Err, A, logfmt(&i, f, format!("Unrecognized Token `{ln}`"))); 
            err!(e); 
        }

        //
        // scope end
        if ln == "}" {
            if !scope.is_empty() {
                scope.pop();
                continue;
            }

            logger(Level::Err, A, logfmt(&i, f, "Unmatched Bracket"));
            err!(e);
        }

        logger(Level::Err, A, logfmt(&i, f, format!("Unrecognized Token `{ln}`")));
        err!(e);

    }

    // delimiters not matching
    if !scope.is_empty() {
        scope.iter().for_each(|s| {
            logger(Level::Err, A, format!("Unmached Delimiter for Subroutine `{}`", s));
            e += 1;
        });
    }

    // debug info
    if debug {
        d.iter().for_each(|d|
            logger(Level::Debug, A, format!("{d:?}"))
        );
    }

    if e != 0 {
        return Err(e);
    }

    Ok(d)
}

fn parse_subroutine(s: &str) -> Result<&str, String> {
    let s: Vec<&str> = s.split_whitespace().collect();

    // if !validate_str(s[0]) {
        // logger(Level::Err, A, logfmt(&i, f, "Invalid Character"));
        // err!(e);
    // }
    todo!();

}

fn parse_marker(s: &str) -> Result<&str, String> {
    if !validate_str(s) {
        bail!(st!("Invalid Character"));
    }

    Ok(s)
}

// 40 + 8 * 9 + (6 - foo)

// TODO
// arithmetic logic
fn al_parser(s: &str)  -> Result<(Vec<Token>, String), String> {
    if s.is_empty() {
        bail!(st!("Empty Arithmetic Operation"));
    }

    let args: Vec<&str> = s.split_whitespace().collect();
    
    if args.len() < 2 {
        bail!(st!("Arythmetic Operations Require at least two Arguments"));
    }


    let first = args[0];


    todo!();
}

// TODO maybe in the future have a package manager?!?!??
// for now we're testing against a static list
// TODO have a mechanism where the libraries are stored 1 file per module and that entire file is
// added on import, and if not found localy a lib will be searched for on the repo
fn parse_import(s: &str) -> Result<(Token, String), String> {
    let s: Vec<&str> = s.split_whitespace().collect();
    if s.len() < 2 {
        bail!(st!("the Module needs to be Specified"));
    } 

    let lib = s[0];
    let modules = &s[1..];

    //TODO: have this dynamically updated
    let import = match lib {
        "std"  => {
            for module in modules {
                if !STD.contains(module) {
                    bail!(format!("Module `{module}` not found in std"));
                }
            }
            Import::Std
        },
        _ => { 
            // TODO: search for the library
            todo!("External Libraries not yet Supported");
            Import::Extern(lib.to_string())
        },
    };

    Ok((Token::Import(import), modules.join(" ")))
}

fn parse_directive(s: &str) -> Result<(&str, &str), String> {
    let (dir, args) = testo!(s.split_once(' '), {
        bail!(st!("Directive Missing an Argument!"));
    });

    let dir = match dir {
        "def" | "mac" | "ent" => dir,
        &_ => bail!(format!("Invalid Directive `{}`", dir)),
    };

    Ok((dir, args))

}

fn parse_subroutine_def(s: &str) -> Result<(&str, Vec<(Token, String)>), String> {
    let mut d: Vec<(Token, String)> = Vec::new();
    let s: Vec<&str> = s.split_whitespace().collect();

    let (ident, vars) = testo!(s[0].split_once('<'), {
        bail!(format!("Unrecognized Token `{}`", s[0]));
    });

    if !validate_str(ident) {
        bail!(st!("Invalid Character"));
    }

    d.push((Token::SubroutineDef, ident.to_string()));

    let vars: Vec<&str> = vars[0..(vars.len() - 1)]
        .split(',')
        .map(|a| a.trim())
        .filter(|&e| !e.is_empty())
        .collect();

    if !vars.is_empty() {
        vars.iter().for_each(|v| d.push((Token::Argument, v.to_string())));
    }

    if s.len() > 2 {
        s.iter().skip(1)
            .map(|a| a.replace('{', ""))
            .for_each(|a| d.push((Token::HeaderArg, a)));
    }

    let subr_name = match !s[0].contains('<') {
        true => s[0],
        _ => ident,
    };

    Ok((subr_name, d))
}

