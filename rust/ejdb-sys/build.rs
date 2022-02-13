extern crate bindgen;
extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    pkg_config::Config::new().probe("zlib").unwrap();

    let dst = Config::new("ejdb-upstream")
        .cflag("-w")
        .profile("Debug")
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("ENABLE_HTTP", "OFF")
        .build();

    Command::new("make").status().expect("failed to make!");

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib64").display()
    );
    println!("cargo:rustc-link-lib=static=ejdb2-2");
    println!("cargo:rustc-link-lib=static=facilio-1");
    println!("cargo:rustc-link-lib=static=iowow-1");

    println!("cargo:include={}",dst.join("include/ejdb2").display());
    println!("debug={}",format!("-I{}", dst.join("include").as_path().to_str().unwrap()));

    let bindings = bindgen::Builder::default()
        .header(dst.join("include/ejdb2/ejdb2.h").as_path().to_str().unwrap())
        // Hide duplicated types
        .clang_arg( format!("-I{}", dst.join("include").as_path().to_str().unwrap()) )
        .blacklist_item("FP_NAN")
        .blacklist_item("FP_INFINITE")
        .blacklist_item("FP_ZERO")
        .blacklist_item("FP_SUBNORMAL")
        .blacklist_item("FP_NORMAL")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
