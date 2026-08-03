#!/usr/bin/env bash
# Create the soroban-cost-estimator sprint backlog in one batch.
#
# Requires: gh CLI authenticated with access to the repo
# (https://github.com/aigbagbobila/soroban-cost-estimator).
#
# Complexity tiers follow the Drips Wave point system:
#   Trivial = 100 pts | Medium = 150 pts | High = 200 pts
# NOTE: issues added via the GitHub label default to Trivial in Drips;
# set Medium/High complexity in the Drips dashboard after creation.
#
# Usage:
#   ./scripts/create_issues.sh [owner/repo]
set -euo pipefail
REPO="${1:-aigbagbobila/soroban-cost-estimator}"

if ! command -v gh >/dev/null 2>&1; then
    echo "error: gh CLI not found — install and authenticate it first (gh auth login)" >&2
    exit 1
fi

# The Drips program label must exist before --label works on issue creation.
gh label create "Stellar Wave" \
    --repo "$REPO" \
    --color 1d76db \
    --description "Drips Stellar Wave sprint" \
    --force >/dev/null 2>&1 || true

gh issue create --repo "$REPO" --title "feat(watch): graceful shutdown on SIGINT/SIGTERM" --label "Stellar Wave" --body "$(cat <<'EOF'
**Summary**
`watch` polls the network config in an infinite loop and can only be stopped by killing the process. A `Ctrl-C` should exit cleanly (flush the in-flight poll, no partial snapshots written).

**Acceptance Criteria**
- [ ] `Ctrl-C` (SIGINT) and `kill <pid>` (SIGTERM) exit with code 0
- [ ] A second signal force-exits (code 130) if shutdown hangs
- [ ] No partial/invalid snapshot is written if the signal arrives mid-fetch
- [ ] Integration test sends SIGINT to a running `watch` and asserts clean exit

**Tech Stack**: Rust, tokio (`tokio::signal`), `watch` command in `src/main.rs`
**Complexity**: Medium (150 pts)
EOF
)"

gh issue create --repo "$REPO" --title "feat(report): populate read/write entries and bytes from the simulation footprint" --label "Stellar Wave" --body "$(cat <<'EOF'
**Summary**
The cost report hardcodes `read_entries`, `write_entries`, `read_bytes`, and `write_bytes` to `0`. The `simulateTransaction` response's `SorobanTransactionData.resources.footprint` carries the real read-only/read-write ledger keys — decode it and report the true counts and byte sizes.

**Acceptance Criteria**
- [ ] `estimate --json` reports real read/write entry counts and byte sizes (not zeros) for a contract that touches storage
- [ ] Table output shows the same values
- [ ] Tests: `estimate --fn` against the fixture contract (which writes one entry) shows write_entries ≥ 1
- [ ] `minimal.wasm` (upload path, no footprint) still reports zeros without error

**Tech Stack**: Rust, `stellar-xdr` 27 (`SorobanResourceFootprint`), `src/report/cost_report.rs`
**Complexity**: Medium (150 pts)
EOF
)"

gh issue create --repo "$REPO" --title "feat(estimate-all): parallelize per-function simulations with a progress indicator" --label "Stellar Wave" --body "$(cat <<'EOF'
**Summary**
`estimate-all` simulates functions sequentially; contracts with many functions take minutes with no feedback. Run per-function `simulateTransaction` calls concurrently (bounded, e.g. 8 at a time) and print a live progress line.

**Acceptance Criteria**
- [ ] Functions simulate concurrently (wall time < sum of individual calls for a multi-function contract)
- [ ] `--json` output stays deterministic (sorted by function name) despite parallelism
- [ ] Progress indicator shows `done/total` and updates in place; disabled for `--json` and non-TTY
- [ ] Tests pass unchanged; no clippy-pedantic violations

**Tech Stack**: Rust, tokio (`futures::stream`/`FuturesUnordered`), `estimate-all` in `src/main.rs`
**Complexity**: Medium (150 pts)
EOF
)"

gh issue create --repo "$REPO" --title "feat(estimate): validate and coerce --arg values against contract-spec types" --label "Stellar Wave" --body "$(cat <<'EOF'
**Summary**
`--arg` values are currently type-inferred (bool/i64/u64/string). The WASM parser already decodes typed params from the `contractspecv0` section — use them to validate `--arg` values and error early on mismatch (e.g. `--arg step=abc` for an `i64` param).

**Acceptance Criteria**
- [ ] `estimate --fn increment --arg step=5` on the fixture contract validates against the spec's `i64`
- [ ] Wrong-type args produce a clear error naming the parameter and expected type
- [ ] Type inference falls back to spec type when inference is ambiguous (e.g. `symbol`)
- [ ] Unit tests for coercion (i64, u64, bool, string, symbol)

**Tech Stack**: Rust, `ScSpecFunctionV0` decoding in `src/wasm/parser.rs`, `parse_arg_scval` in `src/xdr_helper.rs`
**Complexity**: Medium (150 pts)
EOF
)"

gh issue create --repo "$REPO" --title "feat(cache): prune stale estimates and add cache stats command" --label "Stellar Wave" --body "$(cat <<'EOF'
**Summary**
The estimate cache grows forever (keyed by wasm hash + function + args) and there is no way to inspect or clear it. Add pruning of estimates older than a configurable ledger delta, plus a `cache info`/`cache clear` subcommand.

**Acceptance Criteria**
- [ ] `config diff` output includes cache size and prune suggestion when > N entries
- [ ] New `cache info` prints count, total size, oldest/newest ledger
- [ ] New `cache clear` (with `--confirm`) removes the cache directory
- [ ] Cache tests cover prune-on-load and clear

**Tech Stack**: Rust, `src/cache.rs`, clap subcommands in `src/cli.rs`
**Complexity**: Medium (150 pts)
EOF
)"

gh issue create --repo "$REPO" --title "fix(watch): exponential backoff on RPC failures" --label "Stellar Wave" --body "$(cat <<'EOF'
**Summary**
`watch` logs `Warning: failed to fetch config` and immediately re-polls on the same interval, hammering a down RPC endpoint. Back off exponentially (e.g. 1s → 2s → … → cap at the configured interval) and reset on success.

**Acceptance Criteria**
- [ ] After N consecutive failures the effective poll interval grows to a capped maximum
- [ ] A successful fetch resets the backoff
- [ ] Behavior is covered by a unit test on the backoff logic (extracted helper)

**Tech Stack**: Rust, `watch` in `src/main.rs`, extracted `backoff_interval(failures, base, cap)` helper
**Complexity**: Trivial (100 pts)
EOF
)"

echo "Created 6 issues in $REPO."
