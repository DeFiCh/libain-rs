use proc_macro2::TokenStream;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

fn main() {
    let mut root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let parent = root.clone();
    root.pop();
    let lib_path = &parent.join("src").join("lib.rs");
    let target_dir = &root.join("target");

    let mut content = String::new();
    File::open(lib_path)
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let tt: TokenStream = content.parse().unwrap();
    let codegen = cxx_gen::generate_header_and_cc(tt, &cxx_gen::Opt::default()).unwrap();

    let cpp_stuff = String::from_utf8(codegen.implementation).unwrap();
    File::create(target_dir.join("libain_core.hpp"))
        .unwrap()
        .write_all(&codegen.header)
        .unwrap();
    File::create(target_dir.join("libain_core.cpp"))
        .unwrap()
        .write_all(cpp_stuff.as_bytes())
        .unwrap();
}
