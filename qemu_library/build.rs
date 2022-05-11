use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the FFI header changes
    println!("cargo:rerun-if-changed=ffi/cannoli.h");

    // The bindgen::Builder is the main entry point to bindgen, and lets you
    // build up options for the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("ffi/cannoli.h")

        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))

        // Make `size_t`s in the header show up as `usize`
        .size_t_is_usize(true)

        // Finish the builder and generate the bindings.
        .generate()

        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/ffi_bindings.rs file.
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("ffi_bindings.rs"))
        .expect("Couldn't write bindings!");
}

