use std::{env, path::PathBuf};

pub fn main() {
    cc::Build::new()
        .cpp(false)
        .shared_flag(true)
        .static_flag(true)
        .file("mixbox/mixbox.cpp")
        .compile("mixbox");

    println!("cargo:rerun-if-changed=wrapper.hpp");

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .layout_tests(false);

    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let bindings = bindings
        .clang_args(&["-F", "mixbox"])
        .generate()
        .expect("Unable to generate bindings");

    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings");

    println!("cargo:rerun-if-changed=build.rs");
}
