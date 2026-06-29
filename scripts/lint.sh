#!/usr/bin/env bash
# Rust lint + format check for the workspace (constitution: clear seams, simplicity).
set -euo pipefail
cd "$(dirname "$0")/.."
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
