load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies")
load("@rules_rust//crate_universe:defs.bzl", "splicing_config")
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_dependencies")
load("//3rdparty/crates:defs.bzl", "bin_crates", "crates")

def build_dependencies():
    # Load the dependencies for the rust-project.json generator tool.
    # n.b., rust_register_toolchains in WORKSPACE ensure a rust_analyzer_toolchain is registered.
    #
    # To regenerate the rust-project.json file:
    #   bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
    rust_analyzer_dependencies()

    # If the current version of rules_rust is not a release artifact,
    # you may need to set additional flags such as bootstrap = True.
    crate_universe_dependencies()

    # Cargo packages that contain a library. To generate Bazel targets for binaries,
    # you must annotate on the package. See defs.bzl for working examples.
    #
    # CARGO_BAZEL_REPIN=1 CARGO_BAZEL_REPIN_ONLY=crates bazel sync --only=crates
    crates.repository(
        splicing_config = splicing_config(
            # The resolver version to use in generated Cargo manifests.
            # This flag is only used when splicing a manifest from direct package definitions.
            # https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
            resolver_version = "2",
        ),
    )

    # The bin_crates repository is for cargo packages that contain binaries but no library.
    # https://bazelbuild.github.io/rules_rust/crate_universe.html#binary-dependencies
    #
    # CARGO_BAZEL_REPIN=1 CARGO_BAZEL_REPIN_ONLY=bin_crates bazel sync --only=bin_crates
    bin_crates.repository()

    gazelle_dependencies()
