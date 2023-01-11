#!/usr/bin/env bash
set -euo pipefail -o posix

CARGO_AUDIT_BIN="${CARGO_AUDIT_BIN:-cargo-audit}"
CARGO_LOCK_FILE="${1:?Path to the target to be audited}"

"${CARGO_AUDIT_BIN}" audit --file "${CARGO_LOCK_FILE}"
