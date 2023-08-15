use std::path::{Path, PathBuf};
use std::fmt::Write;

use crate::args_parser::ARGS;
use crate::utils;
use crate::logger::{Level, Debug, logger, At};

const DBG: &Debug = &Debug::PreProcessor;

pub fn process() -> String {
    let infile = &ARGS.infile;

    let path = PathBuf::from(infile).into_boxed_path();
    
    // trim, clean, handle comments, and backticks
    let file = basic_process_pass(utils::reader(infile));

    // debug loging
    if ARGS.debug {
        logger(Level::Debug, None, DBG, format!("\n{}", &file));
    }

    let mut file = File::new(path, file);

    let _ = parse_preprocess_directives(&mut file);

    todo!();
}

struct File {
    path: Box<Path>, // dunno if this needs option
    outer_modules: Option<Vec<File>>, // modules of external libraries
    inner_modules: Option<Vec<File>>, // modules of the current file
    contents: String,
}

impl File {
    fn new(path: Box<Path>, contents: String) -> File {
        File {
            path,
            outer_modules: None,
            inner_modules: None,
            contents,
        }
    }
}

fn parse_preprocess_directives(file: &mut File) -> &mut File {

    // FIXME prob shouldnt unwrap here
    let name = file.path.file_name().unwrap().to_str().unwrap();

    for (i, line) in file.contents.lines().enumerate() {
        // check if the line is a directive
        if !line.starts_with('.') {
            continue;
        }

        if line == "." {
            logger(Level::Err, At::new(&i, name), DBG, "Missing Directive after `.`");
            continue;
        }

        let Some((directive, line)) = line.split_once(' ') else {
            logger(Level::Err, At::new(&i, name), DBG, "Directive missing argument");
            continue;
        };

        match directive.trim() {

            "use" => {
                // check for module
                let Some((library, modules)) = line.trim().split_once(' ') else {
                    logger(Level::Err, At::new(&i, name), DBG, "`use` directive missing module argument");
                    continue;
                };

                let mut temp_modules: Vec<String> = Vec::new();
                let modules = modules.trim();

                // if more than one module
                if let Some(mods) = modules.strip_prefix('{') {
                    // look for closing bracket
                    if let Some(mods) = mods.strip_suffix('}') {
                        let mut mods = mods.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>();
                        temp_modules.append(&mut mods);
                    }
                    else {
                        logger(Level::Err, At::new(&i, name), DBG, "Missing a Closing Bracket `}`");
                        continue;
                    }
                }
                else { // if just a single one
                    temp_modules.push(modules.to_string());
                }

                // open n check if the library exists
                

                todo!()
            },

            "mod" => {
                todo!()
            },

            _ => {
                logger(Level::Err, At::new(&i, name), DBG, format!("Invalid directive `{}`", directive));
                continue;
            },
        }
        


            // // check if exists
            // if !library.exists() {
            //     logger(Level::Err, At::new(&i, &file), DBG, format!("Library `{}` not found in the Directory", library.display()));
            //     continue;
            // }
            //
            // // check if it is a directory
            // if !library.is_dir() {
            //     logger(Level::Err, At::new(&i, &file), DBG, format!("Library `{}` is not a directory", library.display()));
            //     continue;
            // }
    }

    // remove all diorectives after we're done with it
    file.contents = file.contents.lines()
        .filter(|l| !l.starts_with('.'))
        .collect::<String>();

    todo!()
}


#[derive(PartialEq)]
enum State {
    Normal,
    InComment,
    InSingleLineComment,
    InString,
    InStringEscape,
}

fn basic_process_pass(file: String) -> String {
    // surface level cleaning
    let file = file.lines()
        .map(|l| l.trim())
        .filter(|l| !l.starts_with("//") && !l.is_empty());
 
    let mut state = State::Normal;
    let mut out = String::new();
    let mut continue_line = false;

    for mut ln in file { 
        let mut out_line = String::new();

        if State::Normal == state {
            // check for line continuation
            if ln.starts_with('`') {
                continue_line = true;
                ln = ln.trim_start_matches('`');
            }
        }

        let mut chars = ln.chars().peekable();

        while let Some(c) = chars.next() {
            match state {
                State::Normal => {
                    // check for comment
                    if c == '/' {
                        match chars.peek() {
                            Some(&'/') => {
                                state = State::InSingleLineComment;
                                chars.next();
                                continue;
                            },
                            Some(&'*') => {
                                state = State::InComment;
                                chars.next();
                                continue;
                            },
                            _ => out_line.push(c),
                        }
                    }

                    out_line.push(c);

                    // if not comment check for string
                    if c == '"' {
                        state = State::InString;
                    }
                },

                State::InComment => {
                    // check for end of comment
                    if c == '*' && chars.peek() == Some(&'/') {
                        state = State::Normal;
                        chars.next();
                    }
                },

                // do nothing in single line comments
                State::InSingleLineComment => (),

                State::InString => {
                    out_line.push(c);

                    // check for escape
                    if c == '\\' {
                        state = State::InStringEscape;
                        continue;
                    }

                    // check for end of string
                    if c == '"' {
                        state = State::Normal;
                    }
                },

                State::InStringEscape => {
                    out_line.push(c);
                    state = State::InString;
                },
            }
        }

        // reset single line comments at the end of the line
        if state == State::InSingleLineComment {
            state = State::Normal;
            continue;
        }

        if !out_line.is_empty() {
            // append to the last line if we are continuing
            if continue_line {
                continue_line = false;

                out.push(' ');
                out.push_str(&out_line);

                continue;
            }

            write!(out, "\n{}", out_line);
        }
    }

    out.trim_start().to_string()
}
