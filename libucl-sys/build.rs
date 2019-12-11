use std::process::Command;
use std::env;
use std::io::ErrorKind;
use std::path::PathBuf;

fn main() {
    let src = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    let dst = PathBuf::from(&env::var("OUT_DIR").unwrap());

    let mut cmd = Command::new("autoreconf");
    cmd.current_dir(&src.join("libucl"));
    run(cmd.arg("-i"), "autoreconf");

    let mut cmd = Command::new("/bin/bash");
    cmd
        .current_dir(&src.join("libucl"))
        .arg("configure")
        .arg(&format!("--prefix={}", dst.display()));
    run(cmd.arg("--enable-regex")
           .arg("--disable-shared")
           .arg("--disable-dependency-tracking")
           .arg("--with-pic"), "configure");

    let mut cmd = Command::new("make");
    cmd
        .current_dir(&src.join("libucl"));
    run(cmd.arg("install"), "make");

    println!("cargo:rustc-link-lib=static=ucl");
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            fail(&format!("failed to execute command: {}\nis `{}` not installed?",
                          e, program));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!("command did not execute successfully, got: {}", status));
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}
