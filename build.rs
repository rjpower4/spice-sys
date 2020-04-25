use std::process::Command;

fn main() {
    Command::new("sh")
        .arg("scripts/getspice.sh")
        .output()
        .expect("Failed to execute get spice process");
    println!("cargo:rustc-link-search=native=thirdparty/cspice/lib/");
    println!("cargo:rustc-link-lib=static=cspice");
    println!("cargo:rustc-link-lib=static=csupport");
}
