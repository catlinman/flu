extern crate pkg_config;
extern crate gcc;

use std::process::Command;

fn main() {
    match pkg_config::find_library("luajit5.1") {
        Ok(_) => return,
        Err(..) => {}
    };

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to run batchfile")
    } else {
        Command::new("make")
            .args(&["-C", "LuaJit/"])
            .output()
            .expect("failed to run makefile")
    };

    match output.status.success() {
        true => {
            println!("cargo:rustc-link-lib=static={}", "luajit");
            println!("cargo:rustc-link-search=native={}", "LuaJit/src/");
        },
        false => {
            panic!("{}", ::std::str::from_utf8(&output.stderr).unwrap());
        }
    }
}
