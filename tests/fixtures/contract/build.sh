#!/usr/bin/env bash
# Rebuild the real Soroban contract fixture (`../contract.wasm`).
#
# The fixture must be a structurally real Soroban contract (contractspecv0
# custom section + typed params), not just valid WASM, so the parser tests
# exercise the same input shape a real submission would use.
set -euo pipefail
cd "$(dirname "$0")"

rustup target add wasm32v1-none >/dev/null 2>&1 || true

# Release build: a real fixture must be deployable, so it must fit under
# Soroban's wasm/transaction size limits (~129 KB tx max on testnet). The
# debug build is ~3.7 MB and gets rejected on upload; release is ~5 KB.
# Release needs ~2-3 GiB of RAM to link the soroban-sdk tree — do not run
# concurrently with other heavy builds.
cargo build --release --target wasm32v1-none

cp target/wasm32v1-none/release/increment_fixture.wasm ../contract.wasm
echo "Fixture written to ../contract.wasm ($(wc -c < ../contract.wasm) bytes)"
