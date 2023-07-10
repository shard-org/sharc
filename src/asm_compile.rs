use std::io::{self, BufRead};
use std::process::{Command, Stdio};
use std::thread;

use crate::defs::*;

// compiles the asm to machine code
// TODO: add other os/arch than linux/x86
fn post_compile(out_file: &str) -> Result<(), ()> {
    //
    // NASM stuff
    eprint!("Compiling asm... ");
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
        Err(_) => {
            eprint!("ERR: Failed to Run nasm!");
            return Err(());
        },
    };

    // stdout here ====================
    let stdout = match nasm.stdout.take() {
        Some(out) => out,
        None => {
            eprint!("ERR: Failed to Read Stdout!\n");
            return Err(());
        },
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
        None => {
            eprint!("ERR: Failed to Read Stderr!\n");
            return Err(());
        }
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
    let status = nasm.wait().unwrap();
    if !status.success() {
        eprint!("ERR: nasm exited with: {}!", status);
    }
    
    // 
    // runnin threads
    match stdout_thread.join() {
        Ok(()) => (),
        Err(_) => {
            eprint!("ERR: Failed to Spawn Stdout Thread!\n");
            return Err(());
        },
    }

    match stderr_thread.join() {
        Ok(()) => (),
        Err(_) => {
            eprint!("ERR: Failed to Spawn Stderr Thread!\n");
            return Err(());
        },
    }

    eprint!("Done!\n");

    // 
    // Linker stuff
    eprint!("Linking Object Files... ");
    match Command::new("ld").arg(TEMP_OBJ).arg("-o").arg(out_file).spawn() {
        Ok(_) => eprint!("Done!\n"),
        Err(why) => eprint!("ERR!\n{}", why),
    }

    Ok(())
}
