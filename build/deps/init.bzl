load("@crates//:defs.bzl", "crate_repositories")
load("@rules_perl//perl:deps.bzl", "perl_register_toolchains", "perl_rules_dependencies")

def init():
    perl_rules_dependencies()
    perl_register_toolchains()
    crate_repositories()
