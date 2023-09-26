use super::*;
use crate::parser::*;

use std::collections::HashSet;

macro_rules! add {
    ($out:ident, $($fmt:tt)*) => {
        $out.push_str(&format!($($fmt)*))
    };
}

// TODO create an instruction enum thats architecture based and impl Display for it

pub fn compiler((tokens, meta): (Vec<FatToken>, Metadata)) -> String {
    let mut o_ro = String::new();   // section .rodata
    let mut o_d = String::new();    // section .data
    let mut o_t = String::from(".text\n");    // section .text
    let mut o_b = String::new();    // section .bss
	
    // add entry point
    o_t.push_str(&match meta.entry {
        Some(name) => format!(".globl {}\n", name),
        None => String::from(".globl main\n"),
    });

    let mut labels = HashSet::new();

    for (token, at) in tokens {
        match token {
            Token::Label(name) => {
                if labels.contains(&name) {
                    log_at!(ERR, at, "Label `{}` already defined", name);
                }
                labels.insert(name.clone());
                add!(o_t, "{}:\n", name);
            },

            Token::Ret(r) => match r{
                Some(r) => log_at!(FATAL, at, "returning with value not yet implemented"),
                None => add!(o_t, "ret\n"),
            },

            _ => todo!(),
        }
    }

    todo!()
}
