#[derive(Debug)]
pub struct Data<'a> {
    line: usize,
    token: Token,
    text: Option<&'a str>,
}

#[derive(Debug)]
enum Token {
    Directive(Dir),
    SubroutineDef(Stuff),
    Argument,
}

#[derive(Debug)]
enum Dir {
    Use, // include
    Def, // definition
    Mac, // macro
    Ent, // entrance point
}

#[derive(Debug)]
enum Stuff {
    Subroutine,
    Comment,
    Variable,
    Return,
    Logic,
    Marker,
}

macro_rules! errfmt {
    ($ln:expr, $err:expr) => {
        Err(format!("Line {}: {}", $ln, $err))
    };
    ($ln:expr, $err:expr, $spec:expr) => {
        Err(format!("Line: {}: {} `{}`", $ln, $err, $spec))
    };
}

pub fn parser<'a>(file_contents: String) -> Result<Vec<Data<'a>>, String> {
    let mut data: Vec<Data> = Vec::new();

    for (i, line) in file_contents.lines().enumerate() {
        match line.trim() {
            s if s.starts_with("//") => { continue; },
            s if s.starts_with(".") => { 
                let s = match s.split_once(" ") {
                    Some(ln) => ln,
                    None => return errfmt!(i, "Directive missing an Argument!")
                };

                let dir_type = match s.0.get(0..4) {
                    Some(dir) => match dir {
                        ".use" => Dir::Use,
                        ".def" => Dir::Def,
                        ".mac" => Dir::Mac,
                        ".ent" => Dir::Ent,
                        &_ => return errfmt!(i, "Invalid Directive", s.0),
                    },
                    None => return errfmt!(i, "Expected a Directive, Found", s.0),
                };

                data.push(Data {
                    line: i,
                    token: Token::Directive(dir_type),
                    text: None,
                });

                data.push(Data {
                    line: i,
                    token: Token::Argument,
                    text: Some(s.1),
                });
            },
            &_ => (),
        }
    }


    for dat in data {
        eprint!("\n{:?}", dat);
    }
    eprint!("\n");
    todo!();

    Ok(data)
}
