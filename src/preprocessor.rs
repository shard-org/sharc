use std::path::{Path, PathBuf};
use std::fmt::Write;
use std::rc::Rc;

use crate::args_parser::ARGS;
use crate::utils;
use crate::logger::{Level, Debug, logger, At};

const DBG: &Debug = &Debug::PreProcessor;

pub fn process() -> String {
    let infile = &ARGS.infile;

    let path: Rc<Path> = Rc::from(PathBuf::from(infile).as_path());
    
    // trim, clean, handle comments, and backticks
    let content = utils::reader(infile).basic_process_pass();

    // debug loging
    if ARGS.debug {
        logger(Level::Debug, None, DBG, format!("\n{}", &content));
    }

    // FIXME prob shouldnt unwrap here
    let fname = path.file_name().unwrap().to_str().unwrap();

    let mut files = vec![(path.clone(), content.clone())];

    //
    // parse project files
    {
        // if there's no parent just read the current dir
        let mut files_to_parse = utils::rec_reader(
            path.parent().unwrap_or(Path::new("."))
        );

        // read all the files
        for file in files_to_parse {
            let path = Rc::from(file.as_path());
            let content = utils::reader(file.to_str().unwrap()).basic_process_pass();

            files.push((path, content));
        }
    }

    // parse directives
    for (i, line) in content.lines().enumerate() {
        let i = i + 1;

        // check if the line is a directive
        if !line.starts_with('.') {
            continue;
        }

        let Some((directive, args)) = line.split_once(' ') else {
            logger(Level::Err, At::new(&i, fname), DBG, "Malformed directive");
            continue;
        };

        match directive.trim() {
            "inc" => {
                todo!("Include directive")
            },

            // TODO add moar directives

            _ => {
                logger(Level::Err, At::new(&i, fname), DBG, format!("Invalid directive `{}`", directive));
                continue;
            },
        }
        
    }

    // // remove all directives after we're done with it
    // let content = content.lines()
    //     .filter(|l| !l.starts_with('.'))
    //     .collect::<String>();

    todo!();
}

#[derive(PartialEq)]
enum State {
    Normal,
    InComment,
    InSingleLineComment,
    InString,
    InStringEscape,
}

trait FileProcess {
    fn basic_process_pass(self) -> String;
}

impl FileProcess for String {
    fn basic_process_pass(self) -> String {
        // surface level cleaning
        let file = self.lines()
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
}
