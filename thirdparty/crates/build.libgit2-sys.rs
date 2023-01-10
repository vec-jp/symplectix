// https://github.com/rust-lang/git2-rs/blob/master/libgit2-sys/build.rs

use std::env;

fn main() {
    assert!(env::var("CARGO_FEATURE_HTTPS").is_ok());
    assert!(env::var("CARGO_FEATURE_SSH").is_ok());

    let target = env::var("TARGET").unwrap();

    if target.contains("apple") {
        println!("cargo:rustc-link-lib=iconv");
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    }

    if target.contains("windows") {
        println!("cargo:rustc-link-lib=winhttp");
        println!("cargo:rustc-link-lib=rpcrt4");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=crypt32");
    }
}
