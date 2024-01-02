use std::{process, str};


pub fn get_sysroot() -> String {
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .expect("TODO: Handle error");
    str::from_utf8(&out.stdout)
        .expect("TODO: Handle error")
        .trim()
        .into()
}
