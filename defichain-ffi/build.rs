use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const LIBDIR: &'static str = "lib";

fn build_leveldb() {
    println!("[leveldb] Building ain");
    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    //TODO; BUILD AIN
    println!("cargo:rustc-link-search={}", project_dir.join("depend/ain/src/leveldb").display());
    println!("cargo:rustc-link-lib=static=leveldb");
    println!("cargo:rustc-link-lib=static=memenv");

    println!("cargo:rustc-link-search={}", project_dir.join("depend/ain/src/univalue/.libs").display());
    println!("cargo:rustc-link-lib=static=univalue");

    println!("cargo:rustc-link-search={}", project_dir.join("depend/ain/src/crc32c").display());
    println!("cargo:rustc-link-lib=static=crc32c");
    println!("cargo:rustc-link-lib=static=crc32c_sse42");

    println!("cargo:rustc-link-search={}", project_dir.join("depend/ain/src").display());
    // println!("cargo:rustc-link-lib=static=defi_server");
    // println!("cargo:rustc-link-lib=static=defi_common");
    // println!("cargo:rustc-link-lib=static=defi_spv");
    //println!("cargo:rustc-link-lib=static=defi_wallet");
    // println!("cargo:rustc-link-lib=static=defi_zmq");
    // println!("cargo:rustc-link-lib=static=defi_util");
    // println!("cargo:rustc-link-lib=static=defi_consensus");
    //
    //
    // println!("cargo:rustc-link-search={}",project_dir.join("depend/ain/src/crypto").display());
    // println!("cargo:rustc-link-lib=static=defi_crypto_avx2");
    // println!("cargo:rustc-link-lib=static=defi_crypto_base");
    // println!("cargo:rustc-link-lib=static=defi_crypto_shani");
    // println!("cargo:rustc-link-lib=static=defi_crypto_sse41");

    /// Dependencies
    Command::new("tar")
        .args([
            "-xf",
            project_dir.join("depend/ain/depends/built/x86_64-pc-linux-gnu/bdb/bdb-4.8.30-7fd992a3c53.tar.gz").to_str().unwrap(),
            "-C",
            outdir.to_str().unwrap()])
        .output()
        .unwrap();

    Command::new("tar")
        .args([
            "-xf",
            project_dir.join("depend/ain/depends/built/x86_64-pc-linux-gnu/boost/boost-1_77_0-b3e55a1245a.tar.gz").to_str().unwrap(),
            "-C",
            outdir.to_str().unwrap()
        ]).output().unwrap();
    Command::new("tar")
        .args([
            "-xf",
            project_dir.join("depend/ain/depends/built/x86_64-pc-linux-gnu/libevent/libevent-2.1.8-stable-ad73abaca87.tar.gz").to_str().unwrap(),
            "-C",
            outdir.to_str().unwrap()
        ]).output().unwrap();

    println!("cargo:rustc-link-search={}", outdir.join("lib").display());
    println!("cargo:rustc-link-lib=static=boost_atomic-mt-x64");
    println!("cargo:rustc-link-lib=static=boost_filesystem-mt-x64");
    println!("cargo:rustc-link-lib=static=event");
    //println!("cargo:rustc-link-lib=static=event_core");
    //println!("cargo:rustc-link-lib=static=event_extra");
    //println!("cargo:rustc-link-lib=static=event_pthreads");
    //println!("cargo:rustc-link-lib=static=db");
    //println!("cargo:rustc-link-lib=static=db-4.8");
    println!("cargo:rustc-link-lib=static=db_cxx");
   // println!("cargo:rustc-link-lib=static=db_cxx-4.8");
}


fn main() {
    build_leveldb();
    let target = env::var("TARGET").expect("TARGET was not set");

    let outdir = env::var("OUT_DIR").unwrap();
    let includedir = Path::new(&outdir).join("include");
    let mut base_config = cpp_build::Config::new();
    base_config
        .include("depend/ain/src")
        .include("depend/ain/src/leveldb/include")
        .include("depend/ain/src/univalue/include")
        .include("depend/cxx/include")
        .include("depend/ain/src/leveldb/helpers/memenv")
        .include(includedir.as_path())
        .define("__STDC_FORMAT_MACROS", None)
        .flag("-std=c++17");

    base_config.define("HAVE_CONFIG_H", Some("true"));

    // Master node files
    base_config.file("depend/ain/src/masternodes/accounts.cpp");
    // base_config.file("depend/ain/src/masternodes/accountshistory.cpp");
    // base_config.file("depend/ain/src/masternodes/anchors.cpp");
    // base_config.file("depend/ain/src/masternodes/auctionhistory.cpp");
    // base_config.file("depend/ain/src/masternodes/consensus/accounts.cpp");
    // base_config.file("depend/ain/src/masternodes/consensus/governance.cpp");
    // base_config.file("depend/ain/src/masternodes/consensus/icxorders.cpp");
    // base_config.file("depend/ain/src/masternodes/consensus/loans.cpp");
    // base_config.file("depend/ain/src/masternodes/consensus/masternodes.cpp");
    // base_config.file("depend/ain/src/masternodes/consensus/oracles.cpp");
    // base_config.file("depend/ain/src/masternodes/consensus/poolpairs.cpp");
    // base_config.file("depend/ain/src/masternodes/consensus/smartcontracts.cpp");
    // base_config.file("depend/ain/src/masternodes/consensus/tokens.cpp");
    // base_config.file("depend/ain/src/masternodes/consensus/txvisitor.cpp");
    // base_config.file("depend/ain/src/masternodes/consensus/vaults.cpp");
    // base_config.file("depend/ain/src/masternodes/govvariables/attributes.cpp");
    // base_config.file("depend/ain/src/masternodes/govvariables/icx_takerfee_per_btc.cpp");
    // base_config.file("depend/ain/src/masternodes/govvariables/loan_daily_reward.cpp");
    // base_config.file("depend/ain/src/masternodes/govvariables/loan_liquidation_penalty.cpp");
    // base_config.file("depend/ain/src/masternodes/govvariables/loan_splits.cpp");
    // base_config.file("depend/ain/src/masternodes/govvariables/lp_daily_dfi_reward.cpp");
    // base_config.file("depend/ain/src/masternodes/govvariables/lp_splits.cpp");
    // base_config.file("depend/ain/src/masternodes/govvariables/oracle_block_interval.cpp");
    // base_config.file("depend/ain/src/masternodes/govvariables/oracle_deviation.cpp");
    // base_config.file("depend/ain/src/masternodes/gv.cpp");
    // base_config.file("depend/ain/src/masternodes/icxorder.cpp");
    // base_config.file("depend/ain/src/masternodes/incentivefunding.cpp");
    // base_config.file("depend/ain/src/masternodes/loan.cpp");
    // base_config.file("depend/ain/src/masternodes/masternodes.cpp");
    // base_config.file("depend/ain/src/masternodes/oracles.cpp");
    // base_config.file("depend/ain/src/masternodes/poolpairs.cpp");
    // base_config.file("depend/ain/src/masternodes/skipped_txs.cpp");
    // base_config.file("depend/ain/src/masternodes/tokens.cpp");
    // base_config.file("depend/ain/src/masternodes/undos.cpp");
    // base_config.file("depend/ain/src/masternodes/vault.cpp");
    // base_config.file("depend/ain/src/masternodes/vaulthistory.cpp");
    // Wrapper
    // base_config.file("depend/wrapper/wrapper.cpp");
    //base_config.file("depend/ain/src/masternodes/rpc_accounts.cpp");

    base_config.file("depend/ain/src/flushablestorage.cpp");
    base_config.file("depend/ain/src/dbwrapper.cpp");
    base_config.file("depend/ain/src/script/script.cpp");
    base_config.file("depend/ain/src/logging.cpp");
    base_config.file("depend/ain/src/fs.cpp");
    base_config.file("depend/ain/src/random.cpp");
    // base_config.file("depend/ain/src/core_read.cpp");
    // base_config.file("depend/ain/src/core_write.cpp");
    // base_config.file("depend/ain/src/chainparams.cpp");
    base_config.file("depend/ain/src/chainparamsbase.cpp");

    base_config.file("depend/ain/src/support/cleanse.cpp");
    base_config.file("depend/ain/src/support/lockedpool.cpp");

    //util
    base_config.file("depend/ain/src/util/strencodings.cpp");
    base_config.file("depend/ain/src/util/string.cpp");
    base_config.file("depend/ain/src/util/time.cpp");
    base_config.file("depend/ain/src/util/system.cpp");
    base_config.file("depend/ain/src/util/threadnames.cpp");

    base_config.file("depend/cxx/src/cxx.cc");
    base_config.file("depend/ain/src/crypto/sha256.cpp");
    base_config.file("depend/ain/src/crypto/sha512.cpp");
    base_config.file("depend/ain/src/uint256.cpp");
    base_config.file("depend/ain/src/hash.cpp");
    base_config.file("depend/ain/src/crypto/sha1.cpp");

    base_config.build("src/lib.rs");

    // println!("cargo:rustc-link-lib=boost_system");
    // println!("cargo:rustc-link-lib=boost_filesystem");
}