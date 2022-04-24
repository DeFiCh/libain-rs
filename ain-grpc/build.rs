use proc_macro2::TokenStream;

use std::str::FromStr;
use std::{env, fs, io};
use std::fs::{DirEntry, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn visit_files(dir: &Path, f: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_files(&path, f)?;
            } else {
                f(&entry);
            }
        }
    }
    Ok(())
}

const TYPE_ATTRS: &'static [(&'static str, &'static str)] = &[
    (".types", "#[derive(Serialize)] #[serde(rename_all=\"camelCase\")]"),
];

const FIELD_ATTRS: &'static [(&'static str, &'static str)] = &[
    // (".blockchain.version_hex", "#[serde(rename = \"versionHex\")]"),
];

fn generate_from_protobuf(dir: &Path) {
    let mut protos = vec![];
    visit_files(&dir, &mut |entry: &DirEntry| {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".proto") {
            println!("cargo:rerun-if-changed={}", path.display());
            protos.push(path);
        }
    }).expect("visiting files");

    let mut builder = tonic_build::configure();

    for (path, attrs) in TYPE_ATTRS {
        builder = builder.type_attribute(path, attrs);
    }

    for (path, attrs) in FIELD_ATTRS {
        builder = builder.field_attribute(path, attrs);
    }

    builder.compile(&protos, &[dir]).expect("compiling protobuf");
}

fn generate_cxx_glue(path: &Path, target_dir: &Path) {
    println!("cargo:rerun-if-changed={}", path.display());
    let mut contents = String::new();
    File::open(path).unwrap()
        .read_to_string(&mut contents).unwrap();
    let stream = TokenStream::from_str(&contents).unwrap();
    let codegen = cxx_gen::generate_header_and_cc(stream, &cxx_gen::Opt::default()).unwrap();
    File::create(target_dir.join("libain.hpp")).unwrap()
        .write_all(&codegen.header).unwrap();
    File::create(target_dir.join("libain.cpp")).unwrap()
        .write_all(&codegen.implementation).unwrap();
}

fn main() {
    let mut root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let lib_root = root.clone();
    root.pop();
    generate_cxx_glue(&lib_root.join("src").join("lib.rs"), &root.join("target"));
    generate_from_protobuf(&root.join("protobuf"));
}
