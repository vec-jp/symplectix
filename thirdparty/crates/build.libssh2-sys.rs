// https://github.com/alexcrichton/ssh2-rs/blob/master/libssh2-sys/build.rs

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.contains("windows") {
        if env::var("CARGO_FEATURE_OPENSSL_ON_WIN32").is_ok() {
            println!("cargo:rustc-link-lib=static=libssl");
            println!("cargo:rustc-link-lib=static=libcrypto");
        }
    }
}
