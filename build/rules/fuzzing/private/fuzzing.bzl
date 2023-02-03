def _fuzzing_impl(ctx):
    entrypoint = ctx.actions.declare_file(ctx.label.name)

    entrypoint_template = """\
{exports}
RUNFILES_DIR="$0.runfiles" \
exec "{fuzzing}" "{command}" \
{envs} \
-- \
"{target}" \
"$@"
"""

    entrypoint_content = entrypoint_template.format(
        exports = "\n".join([
            "export %s='%s'" % (key, val)
            for key, val in ctx.attr.envs.items()
        ]),
        fuzzing = ctx.executable._fuzzing.short_path,
        command = ctx.attr.command,
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

    runfiles = ctx.runfiles() \
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
            allow_single_file = True,
            cfg = "exec",
        ),
        "command": attr.string(
            mandatory = True,
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

def fuzzing_run(**kwargs):
    _fuzzing(
        command = "run",
        **kwargs
    )
