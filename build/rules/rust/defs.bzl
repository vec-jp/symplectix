load(
    "@rules_rust//rust:defs.bzl",
    _rust_binary = "rust_binary",
    _rust_doc = "rust_doc",
    _rust_doc_test = "rust_doc_test",
    _rust_library = "rust_library",
    _rust_test = "rust_test",
    _rust_test_suite = "rust_test_suite",
)
load("//build/rules/rust:rust_fuzz_binary.bzl", _rust_fuzz_binary = "rust_fuzz_binary")

rust_binary = _rust_binary
rust_doc = _rust_doc
rust_doc_test = _rust_doc_test
rust_fuzz_binary = _rust_fuzz_binary
rust_library = _rust_library
rust_test = _rust_test
rust_test_suite = _rust_test_suite
