/*
#[allow(non_camel_case_types)]

extern crate bindgen;
use std::path::PathBuf;

const HEADER_PATH: &str = "/usr/lib/swi-prolog/include/SWI-Prolog.h";

fn main() {
    // Tell cargo to tell rustc to link the system swipl
    // shared library.
    println!("cargo:rustc-link-lib=swipl");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}",HEADER_PATH);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(HEADER_PATH)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("./src/bindings");
    bindings
        .write_to_file(out_path.join("sys.rs"))
        .expect("Couldn't write bindings!");
}
*/

fn main()
{
    // Tell cargo to tell rustc to link the system swipl
    // shared library.
    println!("cargo:rustc-link-lib=swipl");
}
