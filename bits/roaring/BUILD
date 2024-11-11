load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "roaring",
    srcs = glob(
        ["*.rs"],
        exclude = ["*_test.rs"],
    ),
    visibility = ["//bits:__subpackages__"],
    deps = [
        "//bits/bit",
        "//bits/bits_core",
        "//bits/bits_mask",
        "@crates//:num",
        "@crates//:smallvec",
    ],
)

rust_test(
    name = "roaring_lib_test",
    timeout = "short",
    crate = ":roaring",
)

rust_doc_test(
    name = "roaring_doc_test",
    timeout = "short",
    crate = ":roaring",
)
