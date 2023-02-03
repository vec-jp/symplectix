def _fuzzing_impl(ctx):
    entrypoint = ctx.actions.declare_file(ctx.label.name)

    entrypoint_template = """\
{exports}
RUNFILES_DIR="$0.runfiles" \
exec "{fuzzing}" "{command}" \
{envs} \
-- \
"{target}" \
"{corpus}" \
"$@"
"""

    entrypoint_content = entrypoint_template.format(
        exports = "\n".join([
            "export %s='%s'" % (key, val)
            for key, val in ctx.attr.envs.items()
        ]),
        fuzzing = ctx.executable._fuzzing.short_path,
        command = ctx.attr.command,
        corpus = ctx.file.corpus.short_path,
        envs = " ".join([
            "\"--env\" '%s'" % key
            for key in ctx.attr.envs.keys()
        ]),
        target = ctx.executable.target.short_path,
    )

    ctx.actions.write(
        entrypoint,
        entrypoint_content,
        is_executable = True,
    )

    runfiles = ctx.runfiles(files = [ctx.file.corpus]) \
        .merge(ctx.attr._fuzzing[DefaultInfo].default_runfiles) \
        .merge(ctx.attr._fuzzing[DefaultInfo].data_runfiles) \
        .merge(ctx.attr.target[DefaultInfo].default_runfiles) \
        .merge(ctx.attr.target[DefaultInfo].data_runfiles)

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
            cfg = "exec",
        ),
        "command": attr.string(
            mandatory = True,
        ),
        "corpus": attr.label(
            mandatory = True,
            allow_single_file = True,
        ),
        "envs": attr.string_dict(
            default = {},
            mandatory = False,
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

def fuzzing_binary(**kwargs):
    _fuzzing(
        command = "run",
        **kwargs
    )

def _fuzzing_corpus_impl(ctx):
    output = ctx.actions.declare_directory(ctx.attr.name)

    corpus_list_args = ctx.actions.args()
    corpus_list_args.set_param_file_format("multiline")
    corpus_list_args.use_param_file("--corpus-list=%s", use_always = True)
    corpus_list_args.add_all(ctx.files.srcs)

    output_args = ctx.actions.args()
    output_args.add("--output=" + output.path)

    ctx.actions.run(
        executable = ctx.executable._fuzzing,
        arguments = ["prep", "corpus", output_args, corpus_list_args],
        inputs = ctx.files.srcs,
        outputs = [output],
    )

    return [DefaultInfo(
        runfiles = ctx.runfiles(files = [output]),
        files = depset([output]),
    )]

fuzzing_corpus = rule(
    implementation = _fuzzing_corpus_impl,
    doc = """
This rule creates a directory collecting all the corpora files
specified in the srcs attribute.
""",
    attrs = {
        "_fuzzing": attr.label(
            default = Label("@//build/rules/fuzzing:fuzzing"),
            executable = True,
            cfg = "exec",
        ),
        "srcs": attr.label_list(
            doc = "The corpus files for the fuzzing test.",
            allow_files = True,
        ),
    },
)
