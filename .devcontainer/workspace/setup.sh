#!/usr/bin/env bash
set -eu -o pipefail -o posix
shopt -s inherit_errexit

for f in /tmp/scripts/*.sh; do
  bash "$f"
done
