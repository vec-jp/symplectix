load("//thirdparty/libssh2:libssh2_repositories.bzl", "libssh2_repositories")
load("//thirdparty/openssl:openssl_repositories.bzl", "openssl_repositories")
load("//thirdparty/zlib:zlib_repositories.bzl", "zlib_repositories")

def repositories():
    openssl_repositories()
    libssh2_repositories()
    zlib_repositories()
