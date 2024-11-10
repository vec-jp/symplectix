load("@rules_go//go:def.bzl", "go_library", "go_test")
load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test", "rust_test_suite")

rust_library(
    name = "bits_trait",
    srcs = glob( [ "**/*.rs", ], exclude = ["**/*_test.rs"], ),
    visibility = ["//visibility:public"],
    deps = [
        "//bits/bit:bit_rs",
        "@crates//:num",
    ],
)

rust_test(
    name = "bits_trait_lib_test",
    timeout = "short",
    crate = ":bits_trait",
)

rust_doc_test(
    name = "bits_trait_doc_test",
    timeout = "short",
    crate = ":bits_trait",
)
