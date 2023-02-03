load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//build/rules/fuzzing/private:fuzzing.bzl", "fuzzing_run")

def rust_fuzz_binary(
        name,
        sanitizer,
        envs = None,
        **kwargs):
    """Helps to fuzzing.
    """

    target_name = name + "_bin"

    fuzzing_run(
        name = name,
        envs = envs,
        target = target_name,
        tags = ["manual"],
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

    kwargs.setdefault("tags", []).extend([
        "manual",
    ])

    rust_binary(
        name = target_name,
        # TODO: do not compile on stable
        # target_compatible_with = [
        #     "@rules_rust//rust/platform/channel:nightly",
        # ],
        **kwargs
    )
