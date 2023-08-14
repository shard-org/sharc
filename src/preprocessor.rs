use crate::args_parser::ARGS;
use crate::utils;
use std::path::Path;
use std::fmt::Write;

pub fn process() -> String {
    let infile = &ARGS.infile;

    // Read the file
    let file = utils::reader(infile);

    // Get the parent directory
    let parent_dir = match Path::new(infile).parent() {
        Some(p) => p.to_str().unwrap(),
        None => "",
    };
    
    let file = clear_junk(&file);


    // // remove comments, empty lines, and trim
    // let file = file.lines()
    //     .filter(|l| !l.starts_with("//"))
    //     .filter(|l| !l.is_empty())
    //     .map(|l| l.trim())
    //     .collect::<Vec<&str>>()
    //     .join("\n");
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

fn clear_junk(input: &str) -> String {
    let input = input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.starts_with("//") && !l.is_empty());
 
    let mut state = State::Normal;
    let mut out = String::new();

    for ln in input { 
        let mut chars = ln.chars().peekable();
        let mut out_line = String::new();

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

                State::InSingleLineComment => {
                    // check for end line
                    if c == '\n' {
                        state = State::Normal;
                    }
                },

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

        if state != State::InComment && !out_line.is_empty() {
            writeln!(out, "{}", out_line);
        }
    }

    println!("{}", out);
    todo!();
}

fn append_lines() {

}
