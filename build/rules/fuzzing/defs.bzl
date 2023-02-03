load("@rules_rust//rust:defs.bzl", "rust_binary")

def _fuzzing_impl(ctx):
    entrypoint = ctx.actions.declare_file(ctx.label.name)

    entrypoint_template = """
RUNFILES_DIR="$0.runfiles" \
exec "{fuzzing}" "{command}" \
-- \
"{target}" \
"$@"
"""

    entrypoint_content = entrypoint_template.format(
        # environment = "\n".join([
        #     "export %s='%s'" % (var, file.short_path)
        #     for var, file in binary_info.engine_info.launcher_environment.items()
        # ]),
        fuzzing = ctx.executable._fuzzing.short_path,
        command = ctx.attr.command,
        target = ctx.executable.target.short_path,
    )

    ctx.actions.write(
        entrypoint,
        entrypoint_content,
        is_executable = True,
    )

    runfiles = ctx.runfiles()
    runfiles = runfiles.merge(ctx.attr._fuzzing[DefaultInfo].default_runfiles)
    runfiles = runfiles.merge(ctx.attr.target[DefaultInfo].default_runfiles)

    return [
        DefaultInfo(
            executable = entrypoint,
            runfiles = runfiles,
        ),
    ]

_fuzzing = rule(
    implementation = _fuzzing_impl,
    executable = True,
    attrs = {
        "_fuzzing": attr.label(
            default = Label("@//build/rules/fuzzing:fuzzing"),
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
        "command": attr.string(
            mandatory = True,
        ),
        "target": attr.label(
            doc = "The executable of the fuzz test to run.",
            executable = True,
            allow_single_file = True,
            cfg = "target",
            mandatory = True,
        ),
    },
)

def rust_fuzz_binary(
        name,
        sanitizer = None,
        **bin_kwargs):
    """Helps to fuzzing.
    """

    rustc_flags = [
        "--cfg=fuzzing",
        "-Cinstrument-coverage",
        "-Cpasses=sancov-module",
        "-Cllvm-args=-sanitizer-coverage-level=4",
        "-Cllvm-args=-sanitizer-coverage-inline-8bit-counters",
        "-Cllvm-args=-sanitizer-coverage-pc-table",
        "-Cllvm-args=-sanitizer-coverage-trace-compares",
        "-Zsanitizer={}".format(sanitizer),
    ]

    rust_binary(
        name = name + "_bin",
        rustc_flags = select({
            "@rules_rust//rust/toolchain/channel:nightly": rustc_flags,
            "//conditions:default": [],
        }),
        # target_compatible_with = [
        #     "@rules_rust//rust/platform/channel:nightly",
        # ],
        **bin_kwargs
    )

    _fuzzing(
        name = name,
        command = "run",
        target = name + "_bin",
    )
