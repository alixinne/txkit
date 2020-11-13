use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Setup environment
    let tmpdir = tempdir::TempDir::new("txkit-tests").expect("failed to create temp dir");
    env::set_var("OUT_DIR", tmpdir.as_ref());
    env::set_var("TARGET", txkit_tests::config::TARGET);
    env::set_var("OPT_LEVEL", txkit_tests::config::OPT_LEVEL);
    env::set_var("HOST", txkit_tests::config::HOST);

    let libdir = PathBuf::from(txkit_tests::config::OUT_DIR)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("deps");

    let target_filename = PathBuf::from(tmpdir.as_ref()).join("debug");

    // Build shared library
    println!(
        "GCC Output: {}",
        String::from_utf8_lossy(
            &Command::new("gcc")
                .args(&[
                    "-Wall",
                    "-Werror",
                    "-g",
                    "-I../include",
                    &format!("-L{}", libdir.to_string_lossy().as_ref()),
                    &format!("-Wl,-rpath={}", libdir.to_string_lossy().as_ref()),
                    "-ltxkit_builtin",
                    "-ltxkit_core",
                    "-o",
                    target_filename.to_string_lossy().as_ref(),
                    "examples/debug.c",
                ])
                .output()
                .expect("failed to compile code")
                .stderr
        )
    );

    std::mem::forget(tmpdir);

    // Load it
    println!(
        "Program output: {}",
        String::from_utf8_lossy(
            &Command::new(target_filename)
                .output()
                .expect("running program failed")
                .stderr
        )
    );
}
