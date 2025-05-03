use cbindgen::Language;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/server/bindings.rs");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // XXX not stable, where should we put header? https://github.com/rust-lang/cargo/issues/3946
    let out_dir = env::var("OUT_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(manifest_dir)
        .with_language(Language::C)
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(format!("{}/bindings.h", out_dir));
}
