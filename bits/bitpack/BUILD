load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test", "rust_test_suite")

rust_library(
    name = "bitpack",
    srcs = ["bitpack.rs"],
    crate_root = "bitpack.rs",
    visibility = ["//bits:__subpackages__"],
    deps = ["//bits"],
)

rust_test(
    name = "bitpack_lib_test",
    timeout = "short",
    crate = ":bitpack",
)

rust_doc_test(
    name = "bitpack_doc_test",
    timeout = "short",
    crate = ":bitpack",
)

rust_test_suite(
    name = "bitpack_tests",
    timeout = "short",
    srcs = glob(["*_test.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    rustc_flags = [
        "-Copt-level=3",
    ],
    deps = [
        "//bits",
        "//bits/bitaux",
        "//bits/bitpack",
        "@crates//:quickcheck",
    ],
)
