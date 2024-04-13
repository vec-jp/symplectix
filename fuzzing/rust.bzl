load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//fuzzing/internal:fuzzing.bzl", "fuzz_corpus", "fuzz_test")

def rust_fuzz_test(
        name,
        sanitizer = "address",
        corpus = None,
        envs = None,
        **kwargs):
    """A helper macro for fuzzing.
    """

    test_name = name + "_fuzz_test"
    corpus_name = name + "_fuzz_corpus"

    kwargs.setdefault("tags", []).extend([
        "fuzzing",
    ])

    fuzz_test(
        name = name,
        envs = envs,
        executable = test_name,
        corpus = corpus_name,
        tags = kwargs["tags"],
    )

    fuzz_corpus(
        name = corpus_name,
        srcs = corpus,
        tags = kwargs["tags"],
    )

    kwargs.setdefault("rustc_flags", []).extend([
        "--cfg=fuzzing",
        "-Cinstrument-coverage",
        "-Cpasses=sancov-module",
        "-Cllvm-args=-sanitizer-coverage-level=4",
        "-Cllvm-args=-sanitizer-coverage-inline-8bit-counters",
        "-Cllvm-args=-sanitizer-coverage-pc-table",
        "-Cllvm-args=-sanitizer-coverage-trace-compares",
        "-Zsanitizer={}".format(sanitizer),
    ])

    rust_binary(
        name = test_name,
        target_compatible_with = [
            "@rules_rust//rust/platform/channel:nightly",
        ],
        **kwargs
    )
