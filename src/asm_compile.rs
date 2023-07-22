use std::io::{self, BufRead};
use std::process::{Command, Stdio};
use std::thread;

use crate::defs::*;

// compiles the asm to machine code
// TODO: add other os/arch than linux/x86
//
// FIXME: the printing of nasm output is only for debug, remove for release
pub fn post_compiler() -> Result<(), &'static str> {
    //
    // NASM stuff
    let mut nasm = match Command::new("nasm")
        .args(["-f", "elf64", TEMP_ASM, "-o", TEMP_OBJ])
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
        reader.lines().flatten().for_each(|l| println!("{l}"))
    });

    // stderr here ====================
    let stderr = match nasm.stderr.take() {
        Some(out) => out,
        None => return Err("Failed to Read Stderr!"),
    };

    let stderr_thread = thread::spawn(move || {
        let reader = io::BufReader::new(stderr);
        reader.lines().flatten().for_each(|l| eprintln!("{l}"))
    });

    //
    // running n stuff
    if !nasm.wait().unwrap().success() {
        return Err("nasm exited with Error!");
    }

    if stdout_thread.join().is_err() {
        return Err("Failed to Spawn Stdout Thread!");
    }

    if stderr_thread.join().is_err() {
        return Err("Failed to Spawn Stderr Thread!");
    }

    Ok(())
}

pub fn linker(out_file: Option<String>) -> Result<(), ()> {
    let out_file = match out_file {
        Some(file) => file,
        None => "output".to_string(),
    };

    match Command::new("ld")
        .args([TEMP_OBJ, "-o", &out_file])
        .output()
    {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
