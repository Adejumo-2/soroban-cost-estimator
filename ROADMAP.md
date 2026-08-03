# 🗺️ soroban-cost-estimator — Project Roadmap

> **Last updated:** 2026-08-03 (Session 4 — finish line executed)
>
> This document tracks build progress against the 20-step specification, logs
> conversation IDs for each logical unit of work, and describes the completion
> vision for Drips Wave submission. **As of Session 3 it also maps every phase
> of the "Remediation & Completion" prompt phase-by-phase** (see the
> Compliance Matrix below), with independently re-verified ground truth.
>
> **Priority scheme**: P0 = blocking (must ship), P1 = important (should ship),
> P2 = submission-ready (nice to have), P3 = stretch (post-submission).
>
> **Session 1** (CID-001 … CID-024) built the tool from scratch.
> **Session 2** (CID-025 … CID-037) was the remediation pass: fee-bug
> root-cause, live invocation cross-check, real WASM fixture, issue backlog
> script, git-rewrite plan.
> **Session 3** (CID-038 … CID-044) re-ran the ground truth, closed
> CID-033 (the `contractspecv0` framing fix), and produced the compliance
> report.
> **Session 4** (CID-045 … CID-052, this session) executed the finish-line
> prompt: `gh` unblocked, three polish units committed cleanly, branch
> renamed to `main` with the **first real green CI runs**, history split into
> **24 conventional commits**, topics + branch protection live, **6 backlog
> issues created**, `v0.1.0` tagged, publish dry-run clean, and the Drips
> supplementary-materials question answered (none required).

---

## 📊 Project Snapshot (updated 2026-08-03)

| Metric | Value |
|--------|-------|
| **Tests** | **53 total, 53 passing** locally and **green on GitHub Actions** (`build` job) against the rewritten history. |
| **Clippy / fmt** | ✅ `cargo fmt --check` and `cargo clippy --all-targets --all-features` (`all` + `pedantic` deny) all clean — enforced by CI, which is green. |
| **`unwrap()`/`expect()` in src/** | Only inside `#[cfg(test)]` modules — spec-compliant. |
| **CLI commands** | 5/5 wired; `estimate --fn --arg` verified against a deployed contract on testnet (CPU exact match, fee ≤0.011%) — reviewable record in `tests/fixtures/contract/README.md`. |
| **Git** | `main` = **24 conventional commits** (rewritten, force-pushed with lease); no batch commits; branch protection live with required check `build`. |
| **CI** | First-ever runs landed 2026-08-03 on `main`: #30789364471 (pre-rewrite) and #30789732360 (rewritten history) — **both success**. |
| **Fixture** | `tests/fixtures/contract.wasm` = 4,742-byte release build, SHA-256 `ea14bca9…e4ecd` matches the deployed testnet contract. |
| **Repo metadata** | Topics live (stellar, soroban, cli, developer-tooling, gas-estimation); LICENSE-MIT + LICENSE-APACHE added; Cargo.toml + CONTRIBUTING URLs fixed. |
| **Backlog issues** | **6 issues live** with the `Stellar Wave` label (Summary / AC / Tech Stack intact); #1/#2 carry implementation-status comments. |
| **Release** | Tag `v0.1.0` pushed; `cargo publish --dry-run` clean. **Real publish pending a crates.io token** (no `~/.cargo/credentials`). |
| **Completion estimate** | ~100% of MVP; repo Drips-ready except the crates.io publish token and the application itself (Step 10). |

---

## ✅ Ground Truth — independently re-verified 2026-08-03

Per the prompt's "before you touch anything" rule, the four commands were
re-run this session. Actual output:

```
$ cargo test --all
     Running unittests src/lib.rs             → 24 passed
     Running unittests src/main.rs            →  1 passed
     Running tests/cache_tests.rs             →  7 passed
     Running tests/cli_tests.rs               → 11 passed
     Running tests/config_diff_tests.rs       →  6 passed
     Running tests/parser_tests.rs            →  4 passed
test result: ok. 53 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo clippy --all-targets --all-features
error: matching over `()` is more explicit        [src/main.rs:728 — uncommitted watch code]
  → `clippy::ignored-unit-patterns` (deny via clippy::pedantic)
warning: unused `std::result::Result` that must be used  [src/main.rs:729]
  → `watch_poll_once(...)` result ignored; fix: `let _ = ...`
# Both findings are in UNCOMMITTED working-tree changes, not at HEAD.

$ grep -rn "unwrap()\|expect(" src/ | grep -v "^src/.*tests"
# no matches outside #[cfg(test)] blocks  → compliant

$ git log --oneline | wc -l
2     # 341bdc8 + c3eaa1c — contradicts the older "1 commit" snapshot;
      # a second (batch) commit landed in Session 2. Split still pending.
```

**Flagged contradictions vs prior claims (per prompt: don't quietly fix, report):**
1. Prior snapshot said "44 tests, 1 in flight (blocked)". Reality: **53/53 pass** — the `contractspecv0` framing fix (CID-033) has since landed and the parser test is green.
2. Prior snapshot said clippy clean. Reality: clean **at HEAD**, but the current working tree breaks the pedantic gate — must be fixed before committing.
3. Prior "1 commit" claim: now **2 commits** + a dirty tree.
4. `cargo fmt --check` was never claimed clean, and it isn't: fails on `gen_test_wasm.rs` at HEAD → CI step 1 is red even though nothing else changed.

---

## 📋 Remediation Prompt Compliance Matrix

Status legend: ✅ complete · 🔶 partial / in progress · ❌ not done · 🚧 blocked (external).

| Phase | Prompt ask | Status | Evidence (this repo) | Remaining work |
|-------|-----------|--------|----------------------|----------------|
| **0** | Run ground truth yourself before anything | ✅ | 4 commands re-run 2026-08-03 (above) | — |
| **1** | Root-cause the fee calc; remove/justify the clamp; regression test | ✅ | `src/report/fee_calc.rs` (no clamp; independent non-refundable; documented `.max(0)` floor); 6 fee-calc tests incl. the exact input that used to go negative (0 total / 100k insns / 1KB → non-ref 10,250, refundable 0); storage-I/O test pinned to live cross-check numbers (15,427 → 4,496 / 10,931) | None. One-sentence root cause below. |
| **2** | Prove `estimate --fn --arg` against a real deployed contract; cross-check vs `stellar contract invoke`; record both numbers reviewably | ✅ (record is thin) | Contract deployed to testnet (wasm hash `ea14bca9…` = current fixture). Cached estimate: ledger 3,898,102, 18,999 stroops, 524,389 CPU. fee_calc test comment: step=5 run, 15,427 stroops / 532,502 CPU. `c3eaa1c` message: "CPU exact match, fee within 0.011% of stellar CLI". | Consolidate into a reviewable artifact: `tests/fixtures/contract/README.md` with both numbers + reproduction steps (P0 #3). |
| **3** | Real Soroban fixture (spec section, typed args), `estimate-all` reads real specs; keep minimal.wasm | ✅ | `contract.wasm` release build (4,742 B, `contractspecv0` present); parser test asserts `has_spec`, `increment(step: i64)`, `param_count == 1`; `parse_contract_spec` decodes raw `ScSpecEntry` stream (framing fix); `minimal.wasm` kept for bare-WASM path; `estimate-all --json` aggregates spec-driven results | None. |
| **4** | Split git history into ~20 conventional commits, pushed incrementally | ❌ | 2 commits + dirty tree. `c3eaa1c` is a batch commit (violates one-per-unit). | Orphan-branch rewrite → ~23 commits; `--force-with-lease` push (P1 plan below). |
| **5** | Repo readiness: README pass, topics, branch protection, CONTRIBUTING/SECURITY content, tag v0.1.0, crates.io | 🔶 | README ✅ (URLs, `--id`/`--json`, Telegram contact, disclaimer). SECURITY.md ✅ (contact + "unaudited tooling" disclaimer). CONTRIBUTING ✅/⚠️ (content real; **clone URL is `stellar/soroban-cost-estimator`, references LICENSE files that don't exist**). Cargo.toml `repository` = wrong org (`stellar/…`). Topics ❌, branch protection ❌ (gh missing), CI trigger listens on `main` but branch is `master` (mismatch), tag ❌, crates.io ❌, fmt gate ❌. | P2 plan below. |
| **6** | Don't build docs site/video until Drips confirms | 🔶 gate respected | Nothing built; gate documented. `docs.drips.network/wave/maintainers/` not yet checked this session. | Re-check Drips maintainer docs before building any extras. |
| **7** | 5–10 real, scoped issues with Summary / Acceptance Criteria / Tech Stack, created via one gh batch | 🔶 | `scripts/create_issues.sh` (6 issues, reviewed & hardened, `bash -n` clean, chmod +x). **Issues not created: `gh` CLI not installed.** | Install/authenticate gh, run script, verify issues, set Medium/High tiers in Drips dashboard (P3 plan). |
| **8** | Polish: `--json` parity, human `ConfigSettingNotFound`, watch graceful shutdown, estimate-all progress/parallelism | 🔶 | `estimate-all --json` ✅ (committed); `config snapshot --json` ✅ (committed); human `ConfigSettingNotFound` ✅ (already via `human_name()` → `CONFIG_SETTING_CONTRACT_COMPUTE_V0` — the old "Debug output" note is stale); watch graceful shutdown 🔶 (uncommitted, clippy-red); estimate-all progress 🔶 (uncommitted); parallelize ❌ (drafted as issue); watch backoff ❌ (drafted as issue). | Fix + commit the working-tree units (P0 #1); parallelize/backoff as issues (P3). |
| **9** | Apply to Drips Wave (GitHub App, sync, apply; fresh re-checks before applying) | ❌ | Nothing applied yet. | Must come after Phases 1–5 & 7 (P5 plan). Pre-apply re-checks listed. |
| **Ongoing** | config diff after every protocol vote; re-verify XDR shapes on SDK bumps; fresh issues each cycle | 📋 documented | "Ongoing" section at the bottom. | Continuous. |

### Constraint checklist (from the prompt)

- [x] Fee calc bug has a root-cause fix, not just a clamp — clamp removed; `.max(0)` floor retained **with a comment naming the exact legit edge case** (RPC omits the fee → callers pass 0 while rates still yield a positive non-refundable).
- [x] At least one real invocation estimate cross-checked against `stellar contract invoke` — CPU exact match, fee ≤0.011% divergence (recorded in `c3eaa1c` + fee_calc test comment + cache file).
- [x] WASM test fixture is a real Soroban contract, not just valid WASM — release-built `contractspecv0` fixture; its hash matches the live deployed contract.
- [ ] Git history split into logical commits, pushed incrementally — **2 commits today**.
- [ ] Every "done/verified" claim personally run — this session re-ran tests/clippy/fmt/grep and cross-checked hashes & artifacts. Live-network numbers rest on recorded Session-2 artifacts (cache file, commit message, test constants) — reproducible, not inherited blindly.
- [x] No documentation site or demo video built without direct Drips confirmation — nothing built; Phase 6 gate respected.
- [ ] At least 5 real, scoped, Acceptance-Criteria-complete issues exist in the repo — **script ready, issues not yet created** (gh missing).
- [ ] Branch protection + CI job names match what `ci.yml` defines — job name `build` verified ✅; protection not enabled ❌; **workflow triggers on `main` but the branch is `master`** ❌.

---

## ✅ Session 1 — The Build Sequence (CID-001 … CID-024, from spec)

```
✅ = Done      🔶 = Partial
```

### Phase 1: Foundation

| CID | Step | Status | Problem Solved | What Was Built | Tests |
|-----|------|--------|---------------|----------------|-------|
| CID-001 | `chore(scaffold)` | ✅ | **Zero project structure** — couldn't compile, build, or run anything. Needed a Rust project with dependencies pinned to Protocol 27 ecosystem versions, a unified error type eliminating `unwrap()`, and a WASM fixture generator for CI. | `cargo init`, `Cargo.toml` with pinned deps (clap 4.6.4, tokio 1.53, reqwest 0.13, wasmparser 0.254, stellar-xdr 27.0.0, sha2, base64, hex, chrono, dirs, thiserror, comfy-table), `.gitignore`, empty module stubs with doc-comments, `gen_test_wasm` binary | — |
| CID-001 | `error.rs` | ✅ | **No unified error handling** — every RPC call, XDR decode, and file I/O needed to return a `Result` through the same enumeration. Without this, errors would be swallowed by `unwrap()` or inconsistently propagated. | Single `AppError` enum with 17 variants (incl. `ConfigSettingNotFound(String)`), `AppResult<T>` alias. No `unwrap()` outside tests. | — |

### Phase 2: CLI & WASM Parsing

| CID | Step | Status | Problem Solved | What Was Built | Tests |
|-----|------|--------|---------------|----------------|-------|
| CID-002 | `feat(cli)` | ✅ | **No user interface** — users had no way to interact with the tool. Needed a typed CLI with 5 subcommands (`estimate`, `estimate-all`, `config snapshot`, `config diff`, `watch`), sensible defaults (`--network testnet`), optional overrides (`--rpc-url`), and clear argument documentation. | `clap` derive definitions in `cli.rs`. Sensible defaults (`--network testnet`, `--interval 1h`). | — |
| CID-003 | `feat(wasm)` | ✅ | **WASM is opaque binary** — the tool needed to inspect compiled Soroban contracts to enumerate exported functions for batch estimation. Without this, `estimate-all` couldn't discover which functions exist or whether they need arguments. | `wasm/parser.rs`: WASM file loading + structural validation via `wasmparser::validate()`. Function enumeration via `wasmparser::Parser::parse_all()` — TypeSection (param/result counts per type index), FunctionSection (function index → type index), ExportSection (exported names). `WasmInfo` + `FunctionInfo`. (Spec decode added in Session 2, CID-032.) | **3 tests** |

### Phase 3: RPC Layer

| CID | Step | Status | Problem Solved | What Was Built | Tests |
|-----|------|--------|---------------|----------------|-------|
| CID-004 | `feat(rpc)` client | ✅ | **No network communication** — the tool couldn't talk to Soroban RPC endpoints. Needed a JSON-RPC 2.0 client that resolves well-known network names to URLs, sends POST requests, and extracts errors from JSON-RPC responses instead of crashing. | `rpc/client.rs`: `resolve_endpoint()` (testnet → `https://soroban-testnet.stellar.org`, mainnet, futurenet, custom override), `RpcClient` wrapping `reqwest::Client` with generic `call<T>(method, params)`. Extracts JSON-RPC error objects, checks for missing `result`, surfaces HTTP status codes. | — |
| CID-005 | `feat(rpc)` simulate | ✅ | **Can't get real cost data** — the tool needed to call `simulateTransaction` RPC with a constructed transaction envelope and parse the complex response (base64 XDR fields, string-encoded integers, optional fields, `error` field). | `rpc/simulate.rs`: `simulate_transaction()`, `SimulateTransactionResponse` with optional fields, `CostResult { cpu_insns, mem_bytes }` with flexible string/number deserialization (Session 2), `parse_resource_fee()`, `parse_transaction_data_resource_fee()` (Session 2 fallback), `parse_transaction_data_resources()`. | **9 tests** |
| CID-010 | `feat(rpc)` config | ✅ | **Network pricing is in on-chain XDR** — fee computation needs real network pricing rates stored as XDR-encoded `LedgerEntry` bytes in 6 `ConfigSetting` types. | `rpc/config.rs`: `ConfigSettingId` enum (6 variants), `human_name()` for human-readable errors, `LedgerKey::ConfigSetting` XDR encoding, `fetch_all_config_settings()` (all 6 keys in one batched `getLedgerEntries` call), `fetch_config_setting()`. | **2 tests** |

### Phase 4: Fee Math & Cost Reporting

| CID | Step | Status | Problem Solved | What Was Built | Tests |
|-----|------|--------|---------------|----------------|-------|
| CID-006 | `feat(report)` fee calc | ✅ | **Raw `min_resource_fee` isn't transparent** — users need the non-refundable (CPU + storage I/O + bandwidth) vs refundable breakdown. Fee rates come from the network config, not hardcoded constants. All math integer-only (stroops). | `report/fee_calc.rs`: `FeeBreakdown`, `FeeRates` (5 rates), `compute_fee_breakdown(total, cpu, read/write entries, read bytes, tx_size, rates)` — independent non-refundable derivation, `checked_mul`/`saturating_*` overflow safety, documented `.max(0)` refundable floor. `stroops_to_xlm()` / `xlm_to_stroops()` (integer-only). **Root-cause fix landed in Session 2 (CID-025).** | **6 tests** |
| CID-007 | `feat(report)` cost report | ✅ | **Simulation output isn't human-readable** — raw numbers don't tell a story. Machines need structured JSON. | `report/cost_report.rs`: `CostReport` (Serde), `format_report_table()` (comfy-table), `format_report_json()`. | — |

### Phase 5: XDR & Config Snapshot Model

| CID | Step | Status | Problem Solved | What Was Built | Tests |
|-----|------|--------|---------------|----------------|-------|
| CID-008 | `feat(xdr-helper)` | ✅ | **Stellar uses binary XDR everywhere** — config entries, resource fees, transaction envelopes are all base64-XDR. Need one place for encode/decode so a protocol schema change has a single update point. | `xdr_helper.rs`: `decode_config_entry_xdr()`, `begin_snapshot()` (chrono UTC + ledger), `apply_config_entry()` (all 6 variants), `build_simulation_tx_envelope()` (raw XDR bytes; `--id` required for invokes; real contract ID embedded; zeroed source — RPC doesn't validate signatures for simulation), `parse_contract_id()` (hex → 32 bytes), `parse_arg_scval()`. | **7 tests** |
| CID-011 | `feat(config-snapshot)` model | ✅ | **Config settings are complex typed structures** — each of the 6 `ConfigSetting` types has 1–15 fields. | `config_snapshot/model.rs`: `ConfigSnapshot` with 6 optional typed fields + network/timestamp/ledger; `ContractComputeV0` (4), `ContractLedgerCostV0` (15), `ContractHistoricalDataV0` (1), `ContractEventsV0` (2), `ContractBandwidthV0` (3), `StateArchivalV0` (10). Serde + PartialEq. | — |
| CID-012 | `feat(config-snapshot)` store | ✅ | **Snapshots need to persist to disk** — config drift detection requires comparing current config against a saved baseline. | `config_snapshot/store.rs`: `data_dir()` → `~/.soroban-cost-estimator/`, `save_snapshot()`, `load_latest_snapshot()`, `load_snapshot_from_path()`, `list_snapshots()`, `cache_dir()`. | — |

### Phase 6: Diff & Cache

| CID | Step | Status | Problem Solved | What Was Built | Tests |
|-----|------|--------|---------------|----------------|-------|
| CID-014 | `feat(config-snapshot)` diff | ✅ | **Need to detect when network pricing changed** — validators can vote config changes anytime. Users need to know exactly which fields moved and whether they affect pricing. | `config_snapshot/diff.rs`: `FieldDiff`, `ConfigDiff`, `diff_snapshots()` (field-by-field, pricing-sensitive fields flagged), `format_diff()` (emoji UI + ⚠️ pricing warning). | **6 tests** |
| CID-018 | `feat(cache)` | ✅ | **Cost estimates need persistent storage** — the core differentiator requires cross-referencing past estimates against current pricing. | `cache.rs`: `CachedEstimate` (wasm_hash, function, args_hash, network, ledger, total_stroops, cpu_instructions, memory_bytes, timestamp), `save_estimate()`, `load_estimate()`, `list_cached_estimates()`, `find_stale_estimates()` (ledger < current). Args hashed via SHA-256. | **7 tests** |

### Phase 7: Command Wiring (main.rs)

| CID | Step | Status | Problem Solved | What Was Built | Tests |
|-----|------|--------|---------------|----------------|-------|
| CID-009 | `feat(cli)` estimate | ✅ | **Components exist but aren't wired together** — WASM parser, RPC simulation, fee math, and reporting were isolated. | `cmd_estimate()`: load WASM → resolve endpoint → `fetch_fee_rates()` → build tx envelope → `simulateTransaction` → parse response → `compute_fee_breakdown()` with real config rates → `CostReport` → cache → table/JSON. Session 2: `--id` wired through, `parse_transaction_data_resource_fee` fallback, fail-loudly guard. | — |
| CID-013 | `feat(cli)` estimate-all | ✅ | **Batch estimation is tedious** — users need a single command that discovers all exported functions, estimates zero-arg ones, and clearly reports which need manual args. | `cmd_estimate_all()`: enumerate functions → skip ones with params ("Skipped — needs --fn/--arg (N param(s))"), simulate zero-arg ones, print per-function CPU/Mem/Fee/Ledger, cache each. Session 2: `--id` + `--json` wired; Session 3: `[i/N] name` progress line (uncommitted). | — |
| CID-015 | `feat(cli)` config snapshot | ✅ | **No way to capture network pricing** — no baseline for drift detection. | `cmd_config_snapshot()`: batched fetch → XDR decode → typed snapshot → save. `--json` prints + saves. **Verified live on testnet (6/6 settings).** | — |
| CID-016 | `feat(cli)` config diff | ✅ | **Can't tell if pricing changed** — and which cached estimates are now stale. | `cmd_config_diff()`: load snapshot → fetch current → diff → stale-estimate cross-ref → exit 1 if pricing changed. **Verified live on testnet.** | — |
| CID-016.1 | `feat(cli)` watch | ✅ | **Pricing changes happen asynchronously** — need a polling daemon for CI/cron. | `cmd_watch()`: polling loop, `--interval` with `s`/`m`/`h`/`d` suffixes (`parse_interval_secs`, Session 2). Session 3: graceful shutdown refactor (uncommitted). | **1 bin test** |

### Phase 8: CI, Docs & CLI Tests

| CID | Step | Status | Problem Solved | What Was Built | Tests |
|-----|------|--------|---------------|----------------|-------|
| CID-019 | `ci(workflow)` | ✅⚠️ | **No automated quality gate.** | `.github/workflows/ci.yml` (job `build`): checkout → rust-toolchain → rust-cache → `cargo fmt --check` → `cargo clippy --all-targets --all-features` → `cargo build --all-targets` → `cargo run --bin gen_test_wasm` → `cargo test`. **⚠️ Triggers on push to `main`; repo branch is `master` → CI never runs on push today. And `cargo fmt --check` currently fails at HEAD.** | — |
| CID-020 | `docs(readme)` | ✅ | **Zero onboarding documentation.** | README.md (badges, quick-start with testnet commands, differentiator, endpoint table, caching, `--id`/`--json` docs, Telegram contact), CONTRIBUTING.md, SECURITY.md (disclosure contact + "unaudited tooling" disclaimer). Session 2 pass (CID-030) fixed URLs/docs. ⚠️ CONTRIBUTING clone URL is `stellar/…`, references nonexistent LICENSE files. | — |
| CID-017 | CLI integration | ✅ | **No CLI test coverage.** | `tests/cli_tests.rs`: help for all commands, missing `--wasm` errors, unknown command, `--json` acceptance. | **11 tests** |

### Additional Fixes (Session 1)

| CID | Description | Problem Solved | Details |
|-----|-------------|---------------|---------|
| CID-021 | **Bug fix**: Non-refundable fee cap | **Negative refundable fee in reports.** | Session 1 patch: clamped non-refundable to the total. **Superseded by the Session 2 root-cause fix (CID-025).** |
| CID-022 | **Real WASM end-to-end** | **`estimate` never tested with real Soroban contract.** | Session 2 replaced the SDK-23.1.0 discovery with a real SDK-25.3.2 fixture contract (CID-031). |
| CID-023 | **Stale estimate verification** | **No integration test for stale cross-reference.** | Manual cache entry at ledger 1000 vs current ~3.4M proved the output. |
| CID-024 | **Code reviews (4 rounds)** | **No second set of eyes on any code.** | Config & XDR; xdr_helper tests; CLI tests; fee breakdown fix. All fixed and re-reviewed clean. |

---

## ✅ Session 2 — Remediation progress (CID-025 … CID-037)

```
CID-025 … CID-037 = Session 2 work items
✅ = Done & personally verified    🔶 = In progress / blocked
```

| CID | Item | Status | What changed | Where |
|-----|------|--------|--------------|-------|
| CID-025 | `fix(fee-calc)` root-cause | ✅ | **Clamp removed.** `compute_fee_breakdown` now derives `non_refundable` independently from config rates (CPU `(insns×rate)/10_000` + read/write entry fees + read-KB fee + bandwidth `(tx_size×rate)/1024`). `refundable = total − non_refundable`, floored at 0 via `.max(0)` — retained deliberately with a comment naming the exact legit edge case (RPC omits the fee → callers pass 0 while rates still yield positive non-refundable). **Root cause (one sentence):** `min_resource_fee` was clamped *to* the CPU+bandwidth sum, so a fee-less RPC response (0) minus a positive CPU+bw sum produced a negative refundable — and the clamp masked real rate/unit bugs; now non-refundable is rate-derived and refundable floors at 0. | `src/report/fee_calc.rs` + regression tests → 6 fee-calc tests total |
| CID-026 | `feat(simulate)` resource fee | ✅ | New `parse_transaction_data_resource_fee()` decodes `SorobanTransactionData.resource_fee` from `transaction_data` XDR (fallback when `minResourceFee` absent) + `parse_transaction_data_resources()` for modern RPC shape. | `src/rpc/simulate.rs` → 9 tests |
| CID-027 | `feat(xdr-helper)` invoke path | ✅ | `build_simulation_tx_envelope` returns raw XDR bytes, requires `--id` for invoke ops (embeds the real contract ID), new `parse_contract_id()` hex→32-byte helper. | `src/xdr_helper.rs` |
| CID-028 | `feat(estimate)` wiring | ✅ | `estimate`/`estimate-all` pass `--id` through and fall back to the transaction-data resource fee when `minResourceFee` is missing. Fail-loudly guard for misconfigured simulations. | `src/main.rs` |
| CID-029 | `fix(cli)` watch interval | ✅ | `parse_interval_secs()`: `"3600"`, `"3600s"`, `"30m"`, `"1h"`, `"1d"` (trim + lowercase, `saturating_mul`, default 3600 on garbage). Fixes docs-vs-code mismatch. | `src/main.rs` → 1 bin test |
| CID-030 | `docs(readme)` pass | ✅ | Repo URLs → `aigbagbobila/soroban-cost-estimator`; `--id` documented as required for `--fn`; `--arg` type-inference note; `estimate-all --id/--json`; `config snapshot --json`; interval suffixes; Rust 1.85; Telegram contact. | `README.md` |
| CID-031 | `feat(fixture)` real contract | ✅ | Built the real Soroban `increment(env, step: i64) -> i64` contract (soroban-sdk 25.3.2) → `tests/fixtures/contract.wasm`. **Session-2 update: switched from debug (3.7 MB) to release build (4,742 bytes)** — deployable under testnet size limits; `build.sh` documents the ~2–3 GiB RAM requirement. **Fixture SHA-256 matches the live deployed contract's wasm hash.** | `tests/fixtures/contract.wasm`, `tests/fixtures/contract/{build.sh,src/lib.rs,Cargo.toml}` |
| CID-032 | `feat(parser)` contract spec | ✅ | `parse_contract_spec()` decodes the `contractspecv0` section into typed params; `load_wasm` overrides `param_count` from spec (fixes env-pointer inflation: WASM type section says `increment(env, step)` = 2 params, spec knows real arity = 1). Degrades gracefully to bare exports if malformed. | `src/wasm/parser.rs` |
| CID-033 | `test(parser)` real fixture | ✅ **UNBLOCKED in Session 3** | The framing fix landed (with `c3eaa1c`): the `contractspecv0` payload is **not** a count-prefixed `VecM<ScSpecEntry>` — it's raw `ScSpecEntry` XDR values starting with the union discriminant `00 00 00 00` (FunctionV0). `parse_contract_spec` now decodes entries sequentially from a `Cursor` with a `Limited` reader, breaking (not failing) on trailing garbage. `test_load_real_soroban_contract_fixture` asserts `has_spec`, `increment` export, `param_count == 1`, `params[0] = {step, i64}`, `format_function` output. **Passes: 4/4 parser tests.** | `tests/parser_tests.rs`, `src/wasm/parser.rs` |
| CID-034 | `ops` toolchain | ✅ | Stellar CLI 27.0.0 installed; testnet key `test-key` generated; `--cost` flag on `stellar contract invoke` confirmed as the cross-check output channel. | toolchain |
| CID-035 | `test(live)` testnet | ✅ | `config snapshot --network testnet` verified end-to-end (6/6 ConfigSettings, ledger 3,470,630). `minimal.wasm` rejected by testnet (missing metadata) — proves the old 44-byte fixture is unrealistic. **Session 2 completed the Phase-2 loop: fixture contract deployed, `estimate --fn increment --arg step=…` run against it, cross-checked vs `stellar contract invoke --cost` (CPU exact, fee ≤0.011%).** | live testnet |
| CID-036 | `feat(issues)` backlog script | ✅ | `scripts/create_issues.sh` — one-shot `gh` batch creating **6 scoped sprint issues** (watch graceful shutdown, footprint read/write entries, parallelize estimate-all, spec-typed `--arg` validation, cache prune + info/clear, watch backoff), each with Summary / checkbox Acceptance Criteria / Tech Stack / Drips tier. Hardened: `gh` guard, label creation guard, tier caveat. `bash -n` clean, executable. **Execution blocked: `gh` not installed.** | `scripts/create_issues.sh` |
| CID-037 | `chore(git)` plan | ✅ (plan) | Rewrite plan confirmed viable: branch `master`, remote exists, git identity configured. **Execution still pending — now must absorb the second commit `c3eaa1c` and the dirty working tree.** | git |

### Key Session-2 findings worth remembering

1. **Fee bug root cause (one sentence):** `min_resource_fee` was clamped to the CPU+bandwidth sum, so when the RPC omitted the fee (0) the report showed an impossible negative refundable *and* the clamp masked real rate/unit bugs — now non-refundable is derived independently from config rates and refundable floors at 0, with regression tests for the exact input that used to go negative.
2. **`contractspecv0` framing:** the section payload is raw `ScSpecEntry` XDR (starts with the FunctionV0 union discriminant `00 00 00 00`), **not** a count-prefixed `VecM`. The parser decodes entries one at a time (CID-033 fixed & green).
3. **Release fixture is deployable:** release-built `contract.wasm` is 4,742 bytes (debug was 3.7 MB and is rejected on testnet upload); release needs ~2–3 GiB RAM to link.
4. **Testnet is strict:** bare WASM without Soroban metadata is rejected outright — a real contract fixture was mandatory for both Phase 2 and Phase 3.
5. **Live cross-check numbers (recorded):** ledger ~3,898,1xx — CPU exact match with native CLI, fee ≤0.011% divergence; cached estimate `ea14bc…-increment-…json` = 18,999 stroops / 524,389 CPU @ ledger 3,898,102; fee_calc test pins the step=5 run = 15,427 stroops / 532,502 CPU → non-refundable 4,496 / refundable 10,931.

---

## 🔶 Session 3 — Current working tree (uncommitted polish, CI-blocking)

Re-verified ground truth and found the following **uncommitted** work-in-progress
in `src/main.rs` / `src/rpc/simulate.rs`. It is functional but currently
**breaks the clippy gate** (pedantic is denied in `Cargo.toml`), and none of it
is committed yet. These are the first things to land (P0 below).

| Item | Status | What changed | Problem found |
|------|--------|--------------|---------------|
| Watch graceful shutdown | 🔶 uncommitted | `shutdown_signal()` (SIGINT/SIGTERM via `tokio::select` + `tokio::signal::unix`), `watch_poll_once()` extraction (one poll cycle), clean exit 0 on stop signal; in-flight poll cancelled instead of writing a partial snapshot. | ❌ `clippy::ignored-unit-patterns` (pedantic, deny): `_ = async { … }` → must be `() = async { … }` (src/main.rs:728). ❌ `unused_must_use`: `watch_poll_once(...)` result ignored → `let _ = …` (src/main.rs:729). |
| `estimate-all` progress | 🔶 uncommitted | `[i/N] function-name` progress line, suppressed under `--json`. | None — clippy-clean individually. |
| Fee-rate degradation warnings | 🔶 uncommitted | `fetch_fee_rates()` now records which `ConfigSetting*` sources failed to fetch/decode, zeroes only those rates, and prints a stderr warning (a silent zero rate would understate the non-refundable fee). | None. |
| simulate.rs | 🔶 uncommitted | Removed a stale duplicated doc comment. | None. |
| fmt gate | ❌ at HEAD | `cargo fmt --check` fails on `src/bin/gen_test_wasm.rs` (comment-alignment reformat). Not part of the working-tree diff — **present at HEAD**, so CI step 1 is red. | Run `cargo fmt` and review the diff. |
| CI trigger mismatch | ❌ repo-level | `.github/workflows/ci.yml` listens on `push: branches: [main]`; the repo branch is `master`. | Rename branch to `main` (or edit workflow). |

---

## ✅ Session 4 — Finish-line execution (CID-045 … CID-052)

| CID | Item | Status | What happened |
|-----|------|--------|---------------|
| CID-045 | `ops` gh CLI | ✅ | `gh` v2.97.0 installed to `~/.local/bin` (no sudo available); authenticated via the stored OAuth token (`repo` + `workflow` scopes; `read:org` missing but unneeded for repo-scoped ops). `gh auth status` ✓. |
| CID-046 | `fix(cli)` clippy + polish landing | ✅ | `_ = async` → `() = async` (ignored_unit_patterns) and `let _ = watch_poll_once(...)` fixed; `cargo fmt` applied across 16 files (toolchain rustfmt drift — broader than the known `gen_test_wasm.rs` issue); all three gates green; committed as `style(fmt)` + the three feature units + roadmap doc. |
| CID-047 | `chore(git)` branch rename | ✅ | `master` → `main` (local + remote), remote default branch updated, stale `master` deleted. **First real CI runs ever — both green.** |
| CID-048 | `chore(git)` history split | ✅ | Orphan rewrite → **24 conventional commits** in dependency order (scaffold → error → wasm → rpc client/simulate → fee-calc → report → rpc config → config-snapshot model/store/diff → cache → xdr-helper → cli estimate/estimate-all/snapshot/diff/watch/watch-shutdown → tests → ci → docs → fixture → scripts). Final tree **byte-identical** to the previous main; force-pushed with lease (`efdc9b8 → 48df5c9`); CI green on the rewritten history. |
| CID-049 | `chore(repo)` metadata | ✅ | CONTRIBUTING clone URL + Cargo.toml `repository` fixed to `aigbagbobila/…`; LICENSE-MIT + LICENSE-APACHE added; topics applied (5); branch protection live (required check `build`, strict, 1 review, enforce-admins). |
| CID-050 | `docs(fixture)` cross-check record | ✅ | `tests/fixtures/contract/README.md`: contract ID, both number sets side by side, reproduction steps. |
| CID-051 | `feat(issues)` backlog created | ✅ | `./scripts/create_issues.sh aigbagbobila/soroban-cost-estimator` ran: 6 issues live with `Stellar Wave` label; #1/#2 annotated with implementation-status comments. |
| CID-052 | `chore(release)` tag + dry-run | ✅/🚧 | `v0.1.0` tagged + pushed; `cargo publish --dry-run` clean (40 files, verify OK). **Real publish blocked on crates.io token** (user action: `cargo login` or `CARGO_REGISTRY_TOKEN`). |

### Finish-line findings worth recording

1. **Drips supplementary materials: none required** (Phase-6 gate closed — see P4 above).
2. **SCF "Soroban Resource Usage Reporter" 404s** — `stellar/soroban-resource-usage-reporter` returns HTTP 404 and GitHub search finds no match (2026-08-03). Re-confirm immediately before applying; the README's differentiator section currently links to this dead URL.
3. **rustfmt drift**: the committed code was fmt-dirty against the current toolchain's rustfmt across 16 files (the earlier `fmt --check` failure was broader than `gen_test_wasm.rs`); fixed in one `style(fmt)` commit and absorbed into the rewrite.
4. **Token scopes**: the stored OAuth token lacks `read:org` (gh warns on `auth status`) but has `repo` + `workflow` — sufficient for every operation in this session (push, force-with-lease, topics, branch protection, issues, tags).
5. **Branch protection is now live**: direct pushes to `main` require a green `build` check + review — future changes should go through PRs.

### What remains after Session 4

1. **`cargo publish` (crates.io)** — blocked on a token: run `cargo login` (or set `CARGO_REGISTRY_TOKEN`) with an account that owns the `soroban-cost-estimator` name. Everything else is dry-run-verified.
2. **Step 10 — Apply**: sign in to Drips Wave with GitHub, install the Drips Wave GitHub App on the org, sync and apply the repo to the Stellar Wave Program. Immediately before applying (fresh, not from memory): confirm the repo isn't already in the approved list, and re-confirm the SCF reporter's status (see finding #2).
3. **Ongoing duties**: re-run `config diff` after protocol votes; re-verify `ConfigSetting*` XDR shapes on SDK bumps; keep posting fresh issues each Wave cycle.

---

## 🟡 PARTIALLY COMPLETE — historical snapshot (Session 3 state; superseded by Session 4 above)

| Item | Status | Gap | Blocking Issue |
|------|--------|-----|----------------|
| **Git history split** | 🔶 5% | 2 commits + dirty tree. | Orphan rewrite → ~23 commits (P1). |
| **Uncommitted polish units** | 🔶 70% | Watch shutdown + progress + rate warnings are written but clippy-red and uncommitted. | 2-line clippy fix, then commit as 3 units (P0 #1). |
| **Live invocation cross-check record** | 🔶 80% | Numbers exist (cache file, test comment, commit msg) but not in one reviewable place. | Add `tests/fixtures/contract/README.md` (P0 #3). |
| **Repo readiness (topics / branch protection / tag / publish)** | 🔶 15% | gh missing; CI branch mismatch; CONTRIBUTING/Cargo.toml wrong org URL; no LICENSE files; no tag; not published. | P2 plan. |
| **Backlog issues created** | 🔶 40% | Script ready + reviewed; not run. | Install gh / authenticate (P3). |
| **`estimate-all` parallelism** | 🔶 10% | Progress line only. | Drafted as sprint issue (P3). |

**Resolved since the last revision:** CID-033 real-fixture parser test (was
blocked, now green); `ConfigSettingNotFound` human-readable errors (already
shipped via `human_name()` — the old "still shows Debug output" note was
stale); "44 tests, 1 in flight" → **53/53 passing**.

---

## 🔴 REMAINING — Session-3 implementation plan (execution status recorded in Session 4)

### P0: Critical path (do first, in this order)

| # | Task | Plan (how I'll implement) |
|---|------|---------------------------|
| 1 | **Land the uncommitted polish cleanly** | (a) Fix clippy in `src/main.rs`: change `_ = async {` → `() = async {` (satisfies `ignored-unit-patterns`) and `watch_poll_once(network, &mut first).await` → `let _ = watch_poll_once(network, &mut first).await;` (satisfies `unused_must_use`). (b) `cargo fmt` and review the diff (fixes the `gen_test_wasm.rs` gate failure; do NOT let it reformat fixture/test files unnecessarily). (c) Re-verify: `cargo fmt --check` + `cargo clippy --all-targets --all-features` + `cargo test --all` all green (53/53). (d) Commit as three conventional units, no `git add .`: `feat(cli): graceful shutdown for watch on SIGINT/SIGTERM`, `feat(cli): progress indicator in estimate-all`, `fix(rpc): warn when a fee-rate config source is unavailable`. |
| 2 | **Fix the CI trigger mismatch** | `git branch -M main` (renames `master` → `main`, matching `ci.yml` and GitHub defaults; update the remote ref with `git push origin main` / delete stale `master`) **or** change the workflow to `branches: [master]`. Recommend the rename; then a push to `main` should light up the `build` job — first green run validates the fmt gate fix from #1. |
| 3 | **Record the Phase-2 cross-check reviewably** | Add `tests/fixtures/contract/README.md`: deployed contract ID, tool vs native numbers (CPU exact match, fee ≤0.011% divergence; cache: 18,999 stroops / 524,389 CPU @ ledger 3,898,102; step=5: 15,427 stroops / 532,502 CPU), and reproduction steps: `stellar contract install` → `stellar contract create` → `soroban-cost-estimator estimate --wasm … --id … --fn increment --arg step=5` → cross-check `stellar contract invoke --network testnet --id … --source test-key -- increment --step 5 --cost`. This closes Phase 2's "recorded somewhere reviewable" DoD permanently. |

### P1: Git history split (Phase 4)

| Task | Plan |
|------|------|
| **Split into ~23 conventional commits** | Checkout an orphan branch (`git checkout --orphan rewrite && git reset`), then re-stage **specific files only** (never `git add .`) in dependency order and commit with `type(scope): description`. Order: `chore(scaffold)` → `feat(error)` → `feat(wasm)` → `feat(rpc: client)` → `feat(rpc: simulate)` → `fix(fee-calc)` → `feat(report)` → `feat(rpc: config)` → `feat(config-snapshot: model)` → `feat(config-snapshot: store)` → `feat(config-snapshot: diff)` → `feat(cache)` → `feat(cli: estimate)` → `feat(cli: estimate-all)` → `feat(cli: config snapshot)` → `feat(cli: config diff)` → `feat(cli: watch)` → `feat(cli: watch shutdown)` → `test(cli/cache/diff/parser)` → `ci(workflow)` → `docs(readme/roadmap/log)` → `feat(fixture)` → `chore(scripts/issues)`. This absorbs `341bdc8`, `c3eaa1c`, and the uncommitted polish. Verify the **final tree** with `cargo test --all` + clippy, then `git branch -M master` and `git push --force-with-lease origin master` (remote is `aigbagbobila/soroban-cost-estimator`, branch `master` — force-with-lease only; never plain `--force` over unknown remote state; confirm remote refs first). Trade-off: per-commit compilation isn't guaranteed (files reference later modules); if Drips review demands per-commit builds, do an interactive-rebase pass afterward. |
| **Post-split hygiene** | Confirm `git log --oneline` reads as an honest ordered history (~23 lines); confirm the CI `build` job + branch protection names match the workflow. |

### P2: Repo readiness (Phase 5)

| Task | Plan |
|------|------|
| GitHub topics | `gh repo edit --add-topic stellar --add-topic soroban --add-topic cli --add-topic developer-tooling --add-topic gas-estimation` (after gh is installed/authenticated). |
| Branch protection | `gh api -X PUT repos/aigbagbobila/soroban-cost-estimator/branches/main/protection` requiring PRs + required status checks with the **real CI job name `build`** (verified from `.github/workflows/ci.yml` — do not guess). |
| CONTRIBUTING.md fixes | Change clone URL `https://github.com/stellar/soroban-cost-estimator.git` → `https://github.com/aigbagbobila/soroban-cost-estimator.git`; add `LICENSE-MIT` + `LICENSE-APACHE` files (or drop the references) since Cargo.toml declares `license = "MIT OR Apache-2.0"`. |
| Cargo.toml metadata | `repository = "https://github.com/stellar/soroban-cost-estimator"` → `aigbagbobila/…` (wrong org would surface on crates.io). |
| Tag | `git tag -a v0.1.0 -m "v0.1.0 — MVP + remediation pass"` once P0+P1 land; `git push origin v0.1.0`. |
| crates.io | `cargo publish --dry-run` first (catches metadata issues), `cargo login`, then `cargo publish`. After, not instead of, the above. |
| Cosmetic | Banner/logo + contrib.rocks last or skipped — not worth trading against P0–P1 time. |

### P3: Backlog issues + remaining polish (Phases 7 & 8 leftovers)

| Task | Plan |
|------|------|
| Create the 6 issues | Install `gh` (apt/brew or GitHub's installer), `gh auth login`, then run `./scripts/create_issues.sh aigbagbobila/soroban-cost-estimator` in one batch. Verify each issue has Summary / checkbox Acceptance Criteria / Tech Stack; set Medium/High complexity tiers in the Drips dashboard (GitHub-label issues default to Trivial). Fallback if gh can't be installed: create the issues via the GitHub web UI from the script's templates. |
| `estimate-all` parallelize | `FuturesUnordered` with bounded concurrency (8), in-place progress line, disabled for `--json`/non-TTY, deterministic sorted output. Already drafted as a sprint issue. |
| Watch backoff | Extract `backoff_interval(failures, base, cap)` helper (unit-testable) + exponential backoff on repeated fetch failures. Drafted as a sprint issue. |
| Spec-typed `--arg` validation | Validate `--arg` values against the `contractspecv0` param types before building the envelope (drafted as an issue). |
| `estimate-all --fn` subset filter / `contractmeta` parsing | P3 stretch. |

### P4: Phase 6 gate — ANSWERED 2026-08-03 (see Session 4)

**No supplementary materials required.** `docs.drips.network/wave/maintainers/`
was checked via web research: repo review evaluates the GitHub repository
itself — no documentation site, demo video, or on-chain contract verification
is expected. Point system confirmed: Trivial 100 / Medium 150 / High 200
(matches `scripts/create_issues.sh`). **Do not build a GitBook docs site or a
demo video.**

### P5: Apply (Phase 9) — only after P0–P3

1. Sign in to the Drips Wave app with GitHub; install the Drips Wave GitHub App on the org hosting this repo; sync and apply this repo to the Stellar Wave Program.
2. **Immediately before applying** (not days before): re-confirm the project isn't already in the approved-repo list, and re-confirm the SCF-funded "Soroban Resource Usage Reporter" hasn't expanded into config-drift tracking. Both facts change; check fresh.
3. If rejected, use Drips' appeal process rather than treating it as final.
4. Keep posting fresh issues each Wave cycle — a maintainer who applies once and stops adding issues stops being useful even while technically approved.

---

## 🏁 Completion Vision

### What this project looks like when fully done

```
soroban-cost-estimator/
├── Cargo.toml                  # Published on crates.io (v0.1.0)
├── README.md                   # Badges, quick-start, architecture, Telegram contact
├── CONTRIBUTING.md             # PR guidelines, commit convention, correct URLs
├── SECURITY.md                 # Disclosure contact, disclaimer
├── LICENSE-MIT / LICENSE-APACHE
├── ROADMAP.md                  # This file
├── CONVERSATION_LOG.md         # Session-1/2 evidence log
├── scripts/create_issues.sh    # One-shot gh batch for the sprint backlog
├── .github/workflows/ci.yml    # fmt → clippy → build → test (job: build, on main)
├── src/                        # main, cli, lib, error, xdr_helper, cache,
│                               # wasm/, rpc/, config_snapshot/, report/
├── tests/                      # cli/cache/config_diff/parser + real fixture
└── tests/fixtures/
    ├── minimal.wasm            # 44-byte structural-parse fixture (bare-WASM path)
    ├── contract.wasm           # REAL release-built Soroban contract (contractspecv0)
    └── contract/               # source + build.sh + cross-check README
```

### The unique differentiator (unchanged)

> Every other Soroban cost tool tells you what your contract costs *today*.
> soroban-cost-estimator tells you when your cost report is *lying to you*
> because the network changed its prices.

### User workflows (updated for the real invocation path)

```bash
# 📊 Single invocation cost estimate (real contract, deployed)
soroban-cost-estimator estimate --wasm contract.wasm --id <contract-id> --fn increment --arg step=5

# 🔍 Batch estimate all functions (with [i/N] progress; --json for machines)
soroban-cost-estimator estimate-all --wasm contract.wasm --id <contract-id> --network testnet [--json]

# 📸 Snapshot the network's pricing config
soroban-cost-estimator config snapshot --network testnet [--json]

# 🔬 Check if pricing has changed since last snapshot
soroban-cost-estimator config diff --network testnet

# 👀 CI-friendly monitoring (graceful shutdown on Ctrl-C/SIGTERM)
soroban-cost-estimator watch --network mainnet --interval 1h
```

---

## 📋 Conversation Map — All IDs

```
CID-001 … CID-024   Session 1 — initial build (see Session-1 tables above)
CID-025  fix(fee-calc)       ← Negative refundable: root-cause fix, clamp removed
CID-026  feat(simulate)      ← parse_transaction_data_resource_fee fallback
CID-027  feat(xdr-helper)    ← raw XDR bytes, --id required for invokes, parse_contract_id
CID-028  feat(estimate)      ← --id + resource-fee fallback wired into estimate/estimate-all
CID-029  fix(cli)            ← watch --interval suffix parser (s/m/h/d) + test
CID-030  docs(readme)        ← remediation pass (URLs, --id, --json, interval docs, contact)
CID-031  feat(fixture)       ← real Soroban contract fixture, release-built (4,742 B)
CID-032  feat(parser)        ← contractspecv0 typed-param decode + param_count override
CID-033  test(parser)        ← real-fixture parser test — UNBLOCKED, green (4/4)
CID-034  ops                 ← Stellar CLI 27.0.0 + testnet key + --cost cross-check flag
CID-035  test(live)          ← config snapshot E2E on testnet; invocation cross-checked vs native CLI
CID-036  feat(issues)        ← create_issues.sh (6 sprint issues, hardened; run pending gh)
CID-037  chore(git)          ← history-rewrite plan confirmed (execution pending)
CID-038  feat(cli)           ← watch graceful shutdown (SIGINT/SIGTERM) — uncommitted, clippy fix pending
CID-039  feat(cli)           ← estimate-all [i/N] progress indicator — uncommitted
CID-040  fix(rpc)            ← fee-rate degradation warnings — uncommitted
CID-041  chore(ci)           ← fmt gate fix (gen_test_wasm.rs) + branch trigger mismatch — pending
CID-042  chore(git)          ← history split execution (~23 commits) — pending
CID-043  chore(repo)         ← topics / branch protection / tag / publish / metadata — pending
CID-044  chore(issues)       ← create issues via gh batch — pending
```

### Test suite summary (53, all passing — verified 2026-08-03)

```
53 tests total
├── 24 lib unit tests
│   ├── 6  report::fee_calc        (incl. Session-2 root-cause regressions)
│   ├── 9  rpc::simulate           (resource fee/resources parsing, flexible numbers)
│   ├── 2  rpc::config             (LedgerKey XDR encoding)
│   └── 7  xdr_helper              (snapshot creation, config mapping, env/args)
├──  1 bin unit test               (parse_interval_secs — 10 cases)
├── 11 CLI integration tests
├──  7 cache integration tests
├──  6 config diff tests
└──  4 parser tests                (incl. real-contract fixture test — GREEN)
```

---

## 🔁 Ongoing — after approval, this doesn't stop

- Re-run `config diff` after every Stellar protocol vote (Protocol 26 live May 2026; Protocol 27 vote July 2026 — roughly every 3–4 months). A drift alert on this tool is a **release trigger**, not a background curiosity — stale cached estimates actively mislead users until addressed.
- Re-verify `ConfigSetting*` XDR shapes on ecosystem SDK bumps — a protocol upgrade occasionally restructures a config type, which would break decoding silently if untested.
- Keep posting fresh issues each Wave cycle. A maintainer who applies once and stops adding issues stops being useful to the program.
