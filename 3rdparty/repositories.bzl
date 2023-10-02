load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies")
load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_dependencies")

def build_dependencies():
    # Load the dependencies for the rust-project.json generator tool.
    # n.b., rust_register_toolchains in WORKSPACE ensure a rust_analyzer_toolchain is registered.
    #
    # To regenerate the rust-project.json file:
    #   bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
    rust_analyzer_dependencies()

    gazelle_dependencies()
