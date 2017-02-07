extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::{PathBuf, Path};

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}",
             Path::new(&dir).join("lib").display());

    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .header("wrapper.hpp")
        .enable_cxx_namespaces()
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .clang_arg("-I/usr/local/Cellar/glib/2.50.2/include/glib-2.0")
        .clang_arg("-I/usr/local/Cellar/glib/2.50.2/lib/glib-2.0/include")
        .clang_arg("-I/usr/local/opt/gettext/include")
        .clang_arg("-I/usr/local/Cellar/pcre/8.39/include")
        .clang_arg("-I/usr/local/opt/openssl/include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
