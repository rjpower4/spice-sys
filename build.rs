use std::process::Command;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("sh")
        .arg("scripts/getspice.sh")
        .output()
        .expect("Failed to execute get spice process");
    println!("{}", &format!("cargo:rustc-link-search=native={}/thirdparty/cspice/lib/", out_dir));
    println!("cargo:rustc-link-lib=static=cspice");
    println!("cargo:rustc-link-lib=static=csupport");
}
