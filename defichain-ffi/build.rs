use std::env;
use std::path::{Path, PathBuf};

const LIBDIR: &'static str = "lib";
fn build_leveldb() {
    println!("[leveldb] Building");

    let outdir = env::var("OUT_DIR").unwrap();
    let libdir = Path::new(&outdir).join(LIBDIR);

    env::set_var("NUM_JOBS", num_cpus::get().to_string());
    let mut config =
        cmake::Config::new("depend/ain/src/leveldb");
    config
        .define("LEVELDB_BUILD_TESTS", "OFF")
        .define("LEVELDB_BUILD_BENCHMARKS", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", &libdir);
    config.define("HAVE_SNAPPY", "OFF");
    let dest_prefix = config.build();

    assert_eq!(
        dest_prefix.join(LIBDIR),
        libdir,
        "CMake should build LevelDB in provided LIBDIR"
    );
    println!("cargo:rustc-link-search=native={}", libdir.display());
    println!("cargo:rustc-link-lib=static=leveldb");
}



fn main() {
    build_leveldb();
    // println!("cargo:rustc-link-search=native={}", dst.display());
    // println!("cargo:rustc-link-lib=static=foo");
    let mut base_config = cpp_build::Config::new();
    base_config
        .include("depend/ain/src")
        .include("depend/ain/src/univalue/include")
        .include("depend/cxx/include")
        .include("depend/ain/src/leveldb/helpers/memenv")
        .include("depend/ain/src/leveldb/include")
        //.include(format!("{}/lib", dst.display()))
        //.include(format!("{}/include", dst.display()))
        .define("__STDC_FORMAT_MACROS", None)
        .flag("-std=c++17");


    //TODO can be better?
    base_config.define("CLIENT_VERSION_MAJOR", Some("2"))
        .define("CLIENT_VERSION_MINOR", Some("7"))
        .define("CLIENT_VERSION_REVISION", Some("0"))
        .define("CLIENT_VERSION_BUILD", Some("0"))
        .define("CLIENT_VERSION_IS_RELEASE", Some("true"))
        .define("COPYRIGHT_HOLDERS", Some("\"The %s developers\""))
        .define("COPYRIGHT_HOLDERS_SUBSTITUTION", Some("\"The %s developers\""))
        .define("COPYRIGHT_YEAR", Some("2021"));

    base_config.file("depend/ain/src/flushablestorage.cpp");
    base_config.file("depend/ain/src/dbwrapper.cpp");
    base_config.file("depend/ain/src/script/script.cpp");
    base_config.file("depend/ain/src/masternodes/accounts.cpp");
    base_config.file("depend/ain/src/masternodes/masternodes.cpp");
    base_config.file("depend/ain/src/logging.cpp");
    base_config.file("depend/ain/src/fs.cpp");
    base_config.file("depend/ain/src/random.cpp");
    base_config.file("depend/ain/src/support/cleanse.cpp");
    base_config.file("depend/ain/src/util/strencodings.cpp");
    //base_config.file("depend/ain/src/util/system.cpp");
    base_config.file("depend/cxx/src/cxx.cc");
    base_config
        .file("depend/ain/src/crypto/sha256.cpp");
    //base_config.include()

    //


    base_config.build("src/lib.rs")
}