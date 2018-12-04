extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .flag("-std=c11")
        .flag("-march=native")
        .flag("-O3")
        .file("CRoaring/roaring.c")
        .compile("libroaring.a");

    let bindings = bindgen::Builder::default()
        .clang_arg("-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX10.14.sdk/usr/include")
        .blacklist_type("max_align_t")
        .header("CRoaring/roaring.h")
        .generate_inline_functions(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("croaring-sys.rs"))
        .expect("Couldn't write bindings!");
}
