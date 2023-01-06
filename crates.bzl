load("@rules_rust//crate_universe:defs.bzl", "crate")

_annotations = {
    "openssl-sys": [crate.annotation(
        build_script_data = [
            "@openssl//:gen_dir",
            "@openssl//:openssl",
        ],
        build_script_data_glob = ["build/**/*.c"],
        build_script_env = {
            "OPENSSL_DIR": "$(execpath @openssl//:gen_dir)",
            "OPENSSL_STATIC": "1",
        },
        data = ["@openssl"],
        deps = ["@openssl"],
    )],
    "libssh2-sys": [crate.annotation(
        gen_build_script = False,
        deps = ["@libssh2"],
    )],
    "libz-sys": [crate.annotation(
        gen_build_script = False,
        deps = ["@zlib"],
    )],
}

_packages = {
    # openssl
    "openssl": crate.spec(
        version = "0.10.45",
    ),

    # libssh2
    "ssh2": crate.spec(
        version = "0.9",
    ),

    # "arbitrary": crate.spec(
    #     version = "1",
    #     features = ["derive"],
    # },
    "quickcheck": crate.spec(
        version = "1",
    ),
    "quickcheck_macros": crate.spec(
        version = "1",
    ),

    # "rand": crate.spec(
    #     version = "0.8.5",
    # ),
}

crates = struct(
    annotations = _annotations,
    packages = _packages,
)
