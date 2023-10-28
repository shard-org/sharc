use std::process::Command;

// QBE custom build
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=./external/qbe/*");
    let sources = vec![
        "./external/qbe/main.c",
        "./external/qbe/abi.c",
        "./external/qbe/alias.c",
        "./external/qbe/amd64/emit.c",
        "./external/qbe/amd64/isel.c",
        "./external/qbe/amd64/sysv.c",
        "./external/qbe/amd64/targ.c",
        "./external/qbe/arm64/abi.c",
        "./external/qbe/arm64/emit.c",
        "./external/qbe/arm64/isel.c",
        "./external/qbe/arm64/targ.c",
        "./external/qbe/cfg.c",
        "./external/qbe/copy.c",
        "./external/qbe/emit.c",
        "./external/qbe/fold.c",
        "./external/qbe/live.c",
        "./external/qbe/load.c",
        "./external/qbe/mem.c",
        "./external/qbe/parse.c",
        "./external/qbe/rega.c",
        "./external/qbe/util.c",
    ];
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new().files(sources).compile("qbe");
}
