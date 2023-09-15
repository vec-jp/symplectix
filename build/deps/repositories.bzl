load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")
load("@rules_rust//crate_universe:defs.bzl", "crates_repository", "splicing_config")
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
load("//build/deps:versions.bzl", "RUST_STABLE_VERSION")
load("//build/deps/crates/lib_crates:lib_crates.bzl", "lib_crates")

def build_dependencies():
    # openssl
    maybe(
        http_archive,
        name = "openssl",
        build_file = Label("//build/deps/openssl:BUILD.openssl.bazel"),
        sha256 = "cf3098950cb4d853ad95c0841f1f9c6d3dc102dccfcacd521d93925208b76ac8",
        strip_prefix = "openssl-1.1.1w",
        urls = [
            "https://mirror.bazel.build/www.openssl.org/source/openssl-1.1.1w.tar.gz",
            "https://www.openssl.org/source/openssl-1.1.1w.tar.gz",
            "https://github.com/openssl/openssl/archive/OpenSSL_1_1_1w.tar.gz",
        ],
    )
    maybe(
        http_archive,
        name = "nasm",
        build_file = Label("//build/deps/openssl:BUILD.nasm.bazel"),
        sha256 = "f5c93c146f52b4f1664fa3ce6579f961a910e869ab0dae431bd871bdd2584ef2",
        strip_prefix = "nasm-2.15.05",
        urls = [
            "https://mirror.bazel.build/www.nasm.us/pub/nasm/releasebuilds/2.15.05/win64/nasm-2.15.05-win64.zip",
            "https://www.nasm.us/pub/nasm/releasebuilds/2.15.05/win64/nasm-2.15.05-win64.zip",
        ],
    )

    # zlib
    maybe(
        http_archive,
        name = "zlib",
        build_file = Label("//build/deps/zlib:BUILD.zlib.bazel"),
        sha256 = "b3a24de97a8fdbc835b9833169501030b8977031bcb54b3b3ac13740f846ab30",
        strip_prefix = "zlib-1.2.13",
        urls = [
            "https://zlib.net/zlib-1.2.13.tar.gz",
            "https://storage.googleapis.com/mirror.tensorflow.org/zlib.net/zlib-1.2.13.tar.gz",
        ],
    )

    # In order to depend on a Cargo package that contains binaries but no library:
    # * use http_archive to import its source code,
    # * use crates_repository to make build targets for its dependencies
    # * create build targets for the binary
    #
    # https://bazelbuild.github.io/rules_rust/crate_universe.html#binary-dependencies
    http_archive(
        name = "geckodriver",
        build_file = "//build/deps/crates/bin_crates:BUILD.geckodriver.bazel",
        sha256 = "6847d9046206c0f0189857d356991b9b225554045241cb0d33b43c1c83d732b7",
        strip_prefix = "geckodriver-0.33.0",
        type = "tar.gz",
        urls = ["https://crates.io/api/v1/crates/geckodriver/0.33.0/download"],
    )

    # n.b., If the current version of rules_rust is not a release artifact,
    # you may need to set additional flags such as bootstrap = True.
    crate_universe_dependencies()

    # Repinning crates_repository:
    # CARGO_BAZEL_REPIN=1 CARGO_BAZEL_REPIN_ONLY=bin_crates bazel sync --only=bin_crates
    # CARGO_BAZEL_REPIN=1 CARGO_BAZEL_REPIN_ONLY=lib_crates bazel sync --only=lib_crates

    # This is for binary-only packages.
    crates_repository(
        name = "bin_crates",
        cargo_lockfile = "//build/deps/crates/bin_crates:Cargo.lock",
        lockfile = "//build/deps/crates/bin_crates:Cargo.Bazel.lock",
        manifests = [
            "@geckodriver//:Cargo.toml",
        ],
    )

    crates_repository(
        name = "lib_crates",
        annotations = lib_crates.annotations,
        cargo_lockfile = "//build/deps/crates/lib_crates:Cargo.lock",
        lockfile = "//build/deps/crates/lib_crates:Cargo.Bazel.lock",
        packages = lib_crates.packages,
        # The version of Rust the currently registered toolchain is using.
        rust_version = RUST_STABLE_VERSION,
        splicing_config = splicing_config(
            resolver_version = "2",
        ),
    )
