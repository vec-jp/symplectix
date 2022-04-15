load("@rules_rust//crate_universe:defs.bzl", "crate")

packages = {
    # Async
    "tokio": crate.spec(
        version = "1.17",
        features = ["full"],
    ),
    # Proto
    "prost": crate.spec(
        version = "0.10",
    ),
    "prost-types": crate.spec(
        version = "0.10",
    ),
    "prost-build": crate.spec(
        version = "0.10",
    ),
    # gRPC
    "tonic": crate.spec(
        version = "0.7",
    ),
    "tonic-types": crate.spec(
        version = "0.5",
    ),
    "tonic-build": crate.spec(
        version = "0.7",
    ),
    # A framework for serializing/deserializing data structures
    "serde": crate.spec(
        version = "1.0",
        features = ["derive"],
    ),
    # JSON support
    "serde_json": crate.spec(
        version = "1.0",
    ),
    # Utilities to generate random numbers
    "rand": crate.spec(
        version = "0.8.5",
    ),
    # Logging
    "log": crate.spec(
        version = "0.4.16",
    ),
    "env_logger": crate.spec(
        version = "0.9",
    ),
    # Tracing
    "tracing": crate.spec(
        version = "0.1",
    ),
    "tracing-subscriber": crate.spec(
        version = "0.2",
    ),
    # Parses arguments
    "clap": crate.spec(
        features = ["derive"],
        version = "3.1",
    ),
    # Error handling
    "anyhow": crate.spec(
        version = "1.0",
    ),
    "thiserror": crate.spec(
        version = "1.0",
    ),
}
