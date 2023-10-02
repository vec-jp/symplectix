# buildifier: disable=unnamed-macro
def build_dependencies_setup():
    native.register_toolchains("//toolchains:prost_toolchain")
