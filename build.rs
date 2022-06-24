extern crate bindgen;

use std::env;
use std::path::PathBuf;
use cmake;


fn main() {

    let uv = cmake::build("libudx/vendor/libuv");
    //println!("cargo:rustc-link-search=native={}/lib", uv.display());
    //println!("cargo:rustc-link-lib=static=uv");

    let udx = cmake::build("libudx");
    println!("cargo:rustc-link-search=native={}/lib", udx.display());
    println!("cargo:rustc-link-lib=static=udx");
    println!("cargo:rustc-link-lib=static=uv_a");

    // Tell cargo to look for shared libraries in the specified directory
    //println!("cargo:rustc-link-search=libudx/build/Debug");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=udx");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    //println!("cargo:rerun-if-changed=libudx/include/udx.h");

    let bindings = bindgen::Builder::default()
        .header(format!("{}/include/udx.h", udx.display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_type("udx_.*")
        .allowlist_function("udx_.*")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}