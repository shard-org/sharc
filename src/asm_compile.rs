use std::io::{self, BufRead};
use std::process::{Command, Stdio};
use std::thread;

use crate::defs::*;

// compiles the asm to machine code
// TODO: add other os/arch than linux/x86
//
// FIXME: the printing of nasm output is only for debug, remove for release
pub fn post_compile() -> Result<(), &'static str> {
    //
    // NASM stuff
    let mut nasm = match Command::new("nasm")
        .arg("-f elf64")
        .arg(TEMP_ASM)
        .arg("-o")
        .arg(TEMP_OBJ)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn() 
    {
        Ok(child) => child,
        Err(_) => return Err("Failed to Run nasm!"),
    };

    // stdout here ====================
    let stdout = match nasm.stdout.take() {
        Some(out) => out,
        None => return Err("Failed to Read Stdout!"),
    };

    let stdout_thread = thread::spawn(move || {
        let reader = io::BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    });

    // stderr here ====================
    let stderr = match nasm.stderr.take() {
        Some(out) => out,
        None => return Err("Failed to Read Stderr!"),
    };

    let stderr_thread = thread::spawn(move || {
        let reader = io::BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                eprintln!("{}", line);
            }
        }
    });

    //
    // running n stuff
    if !nasm.wait().unwrap().success() {
        return Err("nasm exited with Error!");
    }
    
    if let Err(_)  = stdout_thread.join() {
        return Err("Failed to Spawn Stdout Thread!");
    }

    if let Err(_) = stderr_thread.join() {
        return Err("Failed to Spawn Stderr Thread!");
    }

    Ok(())
}

pub fn post_link(out_file: Option<String>) -> Result<(), ()> {
    let out_file = match out_file {
        Some(file) => file,
        None => "output".to_string(),
    };

    match Command::new("ld")
        .arg(TEMP_OBJ)
        .arg("-o")
        .arg(out_file)
        .output() 
    {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
