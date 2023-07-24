use crate::utils::*;
use crate::{fmtln, reader};

// TODO: concat all lines ending with `\` together, for multiline code like in lua
// any other char works, just has to be something not used anywhere else
pub fn pre_compiler(contents: String, debug: bool, main_file: &str) -> Result<String, ()> {
    let mut e: bool = false;
    let a = At::PreCompiler;
    let mut clean_contents: String = contents.split('\n').filter(|l| !l.trim().starts_with(".use")).collect::<Vec<&str>>().join("\n");
    clean_contents.insert_str(0, &format!("; @FILENAME {main_file}\n"));
    
    for (i, ln) in contents.split('\n').filter_map(|l| l.trim().strip_prefix(".use")).rev().enumerate() {
        let ln = ln.trim();

        if ln.is_empty() {
            logger(Level::Err, &a, fmtln!(i, "`.use` Directive Missing a Path Argument"));
            e = true;
            continue;
        }

        if debug {
            logger(Level::Debug, &a, &format!("Path {i}: {ln:?}"));
        }
        
        let incl_contents = match reader(ln) {
            Ok(c) => format!("; @FILENAME {ln}\n {c}\n"),
            Err(why) => {
                logger(Level::Err, &At::Reader, &why);
                e = true;
                continue;
            },
        };

        clean_contents.insert_str(0, &incl_contents);
    }

    if e { return Err(()); }

    Ok(clean_contents)
}
