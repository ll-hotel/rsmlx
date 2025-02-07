extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    if !std::process::Command::new("make")
        .args(&mut ["-C", "minilibx-linux"])
        .output()
        .expect("failed to spawn make command")
        .status
        .success()
    {
        panic!("failed to make C minilibx-linux library");
    }

    println!("cargo:rustc-link-search=minilibx-linux");
    println!("cargo:rustc-link-lib=static=mlx_Linux");
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=Xext");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
