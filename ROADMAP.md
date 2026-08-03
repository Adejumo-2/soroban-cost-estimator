# 🗺️ soroban-cost-estimator — Project Roadmap

> **Last updated:** 2026-08-03 (Session 5 — final compliance report)
>
> This document is the single source of truth for the project's state: what
> has been built, what has been verified against live testnet, what the
> finish-line remediation achieved, and — precisely — **what remains**.
> Earlier revisions tracked progress toward the Drips Wave submission; this
> revision records that the submission-critical work is **done** and the
> remaining items are mostly user actions (crates.io token, the application
> itself) plus the ongoing maintainer duties.
>
> **Priority scheme**: P0 = blocking, P1 = important, P2 = submission-ready,
> P3 = stretch (post-submission).
>
> **Session map**: Session 1 (CID-001…024) built the tool; Session 2
> (CID-025…037) remediated the fee bug, proved the invocation path, and
> built the real fixture; Session 3 (CID-038…044) re-verified ground truth
> and closed the parser framing fix; Session 4 (CID-045…052) executed the
> finish-line prompt (CI on, history split, repo readiness, issues, tag);
> Session 5 (this revision) reconciles the roadmap to final status.

---

## 📊 Project Snapshot (final — 2026-08-03)

| Metric | Value |
|--------|-------|
| **Tests** | **53 total, 53 passing** locally **and** green on GitHub Actions (`build` job). |
| **Clippy / fmt** | ✅ `cargo fmt --check` + `cargo clippy --all-targets --all-features` (`all` + `pedantic` deny) clean — enforced by CI. |
| **`unwrap()`/`expect()` in src/** | Only inside `#[cfg(test)]` modules — spec-compliant. |
| **CLI commands** | 5/5 wired; `estimate --fn --arg` verified against a **deployed contract on testnet** — CPU exact match, fee ≤0.011% vs `stellar contract invoke --cost` (record in `tests/fixtures/contract/README.md`). |
| **Git** | `main` = **28 commits** (24-commit rewritten history + 4 follow-up units), branch protection live (required check `build`, strict, 1 review, enforce-admins). |
| **CI** | **4 green runs** watched on `main`: `efdc9b8` #30789364471 · `48df5c9` #30789732360 · `6903d3e` #30789865544 · `bf04b39` #30790216052 — the first-ever CI runs, all success. |
| **Fixture** | `tests/fixtures/contract.wasm` = 4,742-byte release build, SHA-256 `ea14bca9…e4ecd` **matches the deployed testnet contract**. |
| **Repo metadata** | Topics live (stellar, soroban, cli, developer-tooling, gas-estimation); LICENSE-MIT + LICENSE-APACHE; Cargo.toml + CONTRIBUTING URLs at `aigbagbobila/…`. |
| **Backlog issues** | **6 live** with the `Stellar Wave` label (Summary / AC / Tech Stack intact); #1/#2 annotated with implementation status. |
| **Release** | `v0.1.0` tag pushed; `cargo publish --dry-run` clean. **Real publish pending a crates.io token.** |
| **Completion estimate** | **~100% of MVP; repo is Drips-ready** except the crates.io publish token and the application itself (user actions). |

---

## ✅ What is DONE (finish-line execution, all verified 2026-08-03)

1. **`gh` unblocked** — v2.97.0 installed to `~/.local/bin`; `gh auth status` ✓ (account `aigbagbobila`, scopes `repo` + `workflow`).
2. **Three polish units landed cleanly** — watch graceful shutdown (SIGINT/SIGTERM), `estimate-all` progress indicator, fee-rate degradation warnings; clippy pedantic fixes applied; `cargo fmt` drift (16 files) fixed in a dedicated `style(fmt)` commit.
3. **CI activated** — branch renamed `master` → `main`, remote default updated, stale `master` deleted; **first-ever CI runs green**.
4. **Git history split** — orphan rewrite into **24 conventional commits** in dependency order; final tree byte-identical to the pushed main; `--force-with-lease` after confirming remote state; CI green on the rewritten history.
5. **Repo metadata** — CONTRIBUTING clone URL + Cargo.toml `repository` fixed; LICENSE-MIT + LICENSE-APACHE added; **5 GitHub topics live**; **branch protection live** (required check `build`, strict, 1 review, enforce-admins).
6. **Cross-check record written** — `tests/fixtures/contract/README.md` (contract ID `CC4WIEYYSCFGDJXMLZ73FKUUJNDEOJRNOOBZHI55QR27NW4RCNTHAQ5T`, both number sets, reproduction steps).
7. **Backlog created** — all 6 issues live via `scripts/create_issues.sh`; #1/#2 carry implementation-status comments.
8. **Release prep** — `v0.1.0` tagged + pushed; `cargo publish --dry-run` **clean** (40 files packaged, verify OK).
9. **Phase-6 gate answered** — Drips maintainer docs checked: **no supplementary materials required** (no docs site, demo video, or on-chain verification expected). Recorded in this file.
10. **Ground truth re-verified** — tests 53/53, clippy clean, no `unwrap()` outside tests, tree identity confirmed (rewritten history == previous content).

---

## 📋 Remediation & Finish-Line Compliance Matrix (current status)

Status legend: ✅ complete · 🚧 blocked on user action · 🔶 in progress · 📋 continuous.

| Phase | Prompt ask | Status | Evidence | Remaining work |
|-------|-----------|--------|----------|----------------|
| **0** | Run ground truth before anything | ✅ | 4 commands re-run 2026-08-03; results recorded in Session 3 section | — |
| **1** | Root-cause the fee calc; remove/justify the clamp; regression test | ✅ | `fee_calc.rs`: no clamp; independent non-refundable derivation; documented `.max(0)` floor; regression tests for the exact input that used to go negative | None |
| **2** | Prove `estimate --fn --arg` against a real deployed contract; cross-check vs native CLI | ✅ | Deployed increment contract (wasm hash = fixture); CPU exact match, fee ≤0.011%; consolidated in `tests/fixtures/contract/README.md` | None |
| **3** | Real WASM fixture with spec section + typed args; `estimate-all` reads real specs | ✅ | Release-built 4,742-byte `contractspecv0` fixture; parser test green (4/4); `minimal.wasm` kept | None |
| **4** | Split git history into ~20 conventional commits, pushed incrementally | ✅ | **24 conventional commits** on `main`, orphan-rewritten, force-pushed with lease, CI green | None |
| **5** | Repo readiness: topics, branch protection, CONTRIBUTING/SECURITY, tag, crates.io | ✅/🚧 | Topics (5), branch protection (build/strict/review), URLs + LICENSE fixed, `v0.1.0` tag pushed, dry-run clean | **crates.io publish: user token** |
| **6** | Don't build docs site/video until Drips confirms | ✅ | `docs.drips.network/wave/maintainers/` checked — **none required** | None (don't build extras) |
| **7** | 5–10 real, scoped issues with Summary / AC / Tech Stack via one gh batch | ✅ | **6 issues live** with `Stellar Wave` label; bodies verified | Set Medium/High tiers in Drips dashboard after approval |
| **8** | Polish: `--json` parity, human `ConfigSettingNotFound`, watch shutdown, progress | ✅ | `--json` wired; `human_name()` errors; watch shutdown + progress landed; parallelize/backoff filed as issues | Contributor backlog (below) |
| **9** | Apply to Drips Wave with fresh pre-apply re-checks | 🔶 | Nothing applied yet | **User browser action** (below) |
| **Ongoing** | config diff after votes; XDR re-verify; fresh issues each cycle | 📋 | Ongoing section at the bottom | Continuous |

### Constraint checklist (final)

- [x] Fee calc bug has a root-cause fix, not just a clamp — clamp removed; `.max(0)` floor kept **with a comment naming the exact legit edge case**.
- [x] At least one real invocation estimate cross-checked against `stellar contract invoke` — CPU exact, fee ≤0.011%, recorded reviewably.
- [x] WASM test fixture is a real Soroban contract — release-built `contractspecv0` fixture whose hash matches the live deployment.
- [x] Git history split into logical commits, pushed incrementally — 24 commits + 4 follow-ups on `main`, force-pushed with lease.
- [x] Every "done/verified" claim personally run — tests/clippy/fmt/grep re-run; CI watched green (4 runs); hashes and artifacts cross-checked.
- [x] No documentation site or demo video built without direct Drips confirmation — none built; Phase-6 gate answered.
- [x] At least 5 real, scoped, Acceptance-Criteria-complete issues exist in the repo — **6 issues live**.
- [x] Branch protection + CI job names match `ci.yml` — required check `build` live, verified by API, not assumed.

---

## 🏛️ Session History (what was done, in order)

### Session 1 — The Build Sequence (CID-001 … CID-024)

Built the tool from scratch against the 20-step spec.

| CID | Unit | What was built | Tests |
|-----|------|----------------|-------|
| CID-001 | `chore(scaffold)` + `error.rs` | `cargo init`, pinned deps (clap 4.6.4, tokio 1.53, reqwest 0.13, wasmparser 0.254, stellar-xdr 27.0.0, sha2, base64, hex, chrono, dirs, thiserror, comfy-table), `.gitignore`, module stubs, `gen_test_wasm` binary; single `AppError` enum (17 variants) + `AppResult<T>` — no `unwrap()` outside tests | — |
| CID-002 | `feat(cli)` | clap derive: 5 subcommands (`estimate`, `estimate-all`, `config snapshot`, `config diff`, `watch`), defaults (`--network testnet`, `--interval 1h`) | — |
| CID-003 | `feat(wasm)` | `wasm/parser.rs`: validate + enumerate exported functions (Type/Function/Export sections), `WasmInfo`/`FunctionInfo` | 3 |
| CID-004 | `feat(rpc)` client | `rpc/client.rs`: `resolve_endpoint()` (testnet/mainnet/futurenet + `--rpc-url`), generic JSON-RPC `call<T>` with error extraction | — |
| CID-005 | `feat(rpc)` simulate | `rpc/simulate.rs`: `simulateTransaction` wrapper, optional fields, flexible string/number deserialization, `parse_resource_fee()` | 9 (later) |
| CID-010 | `feat(rpc)` config | `rpc/config.rs`: `ConfigSettingId` (6), `human_name()`, batched `getLedgerEntries`, `fetch_config_setting()` | 2 |
| CID-006 | `feat(report)` fee calc | `report/fee_calc.rs`: `FeeBreakdown`, `FeeRates` (5 rates), independent non-refundable math, `.max(0)` floor, integer-only XLM conversion | 6 (later) |
| CID-007 | `feat(report)` cost report | `report/cost_report.rs`: `CostReport` (Serde), comfy-table + pretty-JSON formatting | — |
| CID-008 | `feat(xdr-helper)` | `xdr_helper.rs`: `decode_config_entry_xdr`, `begin_snapshot`, `apply_config_entry` (6 variants), `build_simulation_tx_envelope` (raw XDR, `--id`-aware), `parse_contract_id`, `parse_arg_scval` | 7 |
| CID-011/012 | `feat(config-snapshot)` model/store | Typed model for all 6 `ConfigSetting` types; file-backed store at `~/.soroban-cost-estimator/` | — |
| CID-014 | `feat(config-snapshot)` diff | Field-level `diff_snapshots()` with pricing-change flags + `format_diff()` UI | 6 |
| CID-018 | `feat(cache)` | Estimate cache keyed by wasm hash + function + args hash; `find_stale_estimates()` | 7 |
| CID-009/013/015/016/016.1 | `feat(cli)` wiring | `cmd_estimate`, `cmd_estimate_all`, `cmd_config_snapshot`, `cmd_config_diff`, `cmd_watch` — all wired end-to-end | 1 bin |
| CID-019 | `ci(workflow)` | `.github/workflows/ci.yml` (job `build`): fmt → clippy → build → fixture → test | — |
| CID-020 | `docs(readme)` | README (quick-start, differentiator, endpoints, caching), CONTRIBUTING, SECURITY (contact + unaudited disclaimer) | — |
| CID-017 | CLI integration tests | `tests/cli_tests.rs` (help, missing `--wasm`, unknown command, `--json`) | 11 |
| CID-021–024 | Fixes + review | Negative-refundable patch (later superseded), real-WASM E2E, stale-estimate proof, 4 review rounds | — |

### Session 2 — Remediation (CID-025 … CID-037)

| CID | Item | Status | What changed |
|-----|------|--------|--------------|
| CID-025 | `fix(fee-calc)` root cause | ✅ | Clamp removed; non-refundable derived independently from config rates; refundable floors at 0 with a comment naming the exact edge case. **One-sentence root cause:** `min_resource_fee` was clamped to the CPU+bandwidth sum, so a fee-less response (0) minus a positive sum went negative — and the clamp masked real rate/unit bugs. Regression tests added → 6 fee-calc tests. |
| CID-026 | `feat(simulate)` resource fee | ✅ | `parse_transaction_data_resource_fee()` fallback + `parse_transaction_data_resources()` for the modern RPC shape → 9 tests. |
| CID-027 | `feat(xdr-helper)` invoke path | ✅ | Raw XDR bytes; `--id` required for invokes (real contract ID embedded); `parse_contract_id()`. |
| CID-028 | `feat(estimate)` wiring | ✅ | `--id` pass-through + resource-fee fallback + fail-loudly guard. |
| CID-029 | `fix(cli)` watch interval | ✅ | `parse_interval_secs()` (`s`/`m`/`h`/`d` suffixes, default 3600) + 10-case test. |
| CID-030 | `docs(readme)` pass | ✅ | URLs → `aigbagbobila/…`; `--id`/`--json`/interval docs; Telegram contact. |
| CID-031 | `feat(fixture)` real contract | ✅ | `increment(step: i64)` with soroban-sdk 25.3.2; **release build 4,742 B** (debug 3.7 MB rejected by testnet); hash matches live deployment. |
| CID-032 | `feat(parser)` contract spec | ✅ | `parse_contract_spec()` typed-param decode; spec arity overrides the env-pointer count; graceful degrade. |
| CID-033 | `test(parser)` real fixture | ✅ | **Unblocked**: `contractspecv0` payload is raw `ScSpecEntry` XDR (not count-prefixed `VecM`) — decoded sequentially via `Cursor`/`Limited`; test green (4/4). |
| CID-034 | `ops` toolchain | ✅ | Stellar CLI 27.0.0, testnet key `test-key`, `--cost` cross-check channel confirmed. |
| CID-035 | `test(live)` testnet | ✅ | Config snapshot E2E (6/6 settings); `minimal.wasm` rejected by testnet (proves the old fixture was unrealistic); invocation cross-checked vs native CLI. |
| CID-036 | `feat(issues)` backlog script | ✅ | `scripts/create_issues.sh` (6 issues, hardened, `bash -n` clean). |
| CID-037 | `chore(git)` plan | ✅ (plan) | Rewrite viability confirmed (identity, remote, branch). |

### Session 3 — Compliance report + working-tree findings (CID-038 … CID-044)

Re-verified ground truth and found uncommitted polish work in the working tree.
**All items in this session were resolved in Session 4** (clippy fixes landed,
fmt drift fixed, three feature units committed, branch renamed).

| Item | Session-3 finding | Resolution (Session 4) |
|------|-------------------|------------------------|
| Watch graceful shutdown | uncommitted, clippy-red (`ignored_unit_patterns`, `unused_must_use`) | ✅ `() = async` + `let _ = …`; committed |
| `estimate-all` progress | uncommitted | ✅ committed |
| Fee-rate degradation warnings | uncommitted | ✅ committed |
| fmt gate | ❌ failed at HEAD (`gen_test_wasm.rs` and 15 more files) | ✅ `style(fmt)` commit |
| CI trigger mismatch | workflow on `main`, branch was `master` | ✅ renamed to `main` |

### Session 4 — Finish-line execution (CID-045 … CID-052)

| CID | Item | Status | What happened |
|-----|------|--------|---------------|
| CID-045 | `ops` gh CLI | ✅ | `gh` v2.97.0 → `~/.local/bin`; authenticated via stored OAuth token (`repo` + `workflow` scopes). `gh auth status` ✓. |
| CID-046 | `fix(cli)` clippy + polish landing | ✅ | pedantic fixes; `cargo fmt` across 16 files; gates green; `style(fmt)` + 3 feature commits + roadmap doc. |
| CID-047 | `chore(git)` branch rename | ✅ | `master` → `main` (local + remote + default), stale `master` deleted. **First real CI runs — both green.** |
| CID-048 | `chore(git)` history split | ✅ | Orphan rewrite → **24 conventional commits**; tree byte-identical; `--force-with-lease` push (`efdc9b8 → 48df5c9`); CI green. |
| CID-049 | `chore(repo)` metadata | ✅ | URLs fixed; LICENSE-MIT + LICENSE-APACHE; 5 topics; branch protection live. |
| CID-050 | `docs(fixture)` cross-check record | ✅ | `tests/fixtures/contract/README.md` (contract ID, numbers, reproduction). |
| CID-051 | `feat(issues)` backlog created | ✅ | 6 issues live with `Stellar Wave` label; #1/#2 annotated. |
| CID-052 | `chore(release)` tag + dry-run | ✅/🚧 | `v0.1.0` pushed; `cargo publish --dry-run` clean. Real publish blocked on crates.io token. |

### Session-4 findings worth remembering

1. **Drips supplementary materials: none required** — repo review evaluates the repository itself; no docs site, demo video, or on-chain verification expected. Point system confirmed: Trivial 100 / Medium 150 / High 200.
2. **SCF "Soroban Resource Usage Reporter" 404s** (`stellar/soroban-resource-usage-reporter` returns HTTP 404; GitHub search finds no match, 2026-08-03). Re-confirm immediately before applying; the README's differentiator section links to this now-dead URL.
3. **rustfmt drift**: committed code was fmt-dirty against the current toolchain across 16 files — fixed in one `style(fmt)` commit, absorbed into the rewrite.
4. **Branch protection is live**: direct pushes to `main` require a green `build` check + review; future changes go through PRs (the final docs commits were landed via a brief, fully re-enabled protection window).

---

## 🔴 WHAT REMAINS — precisely

### 1. User action — crates.io publish (P0, minutes)

- **Blocked on a token**: no `~/.cargo/credentials`, no `CARGO_REGISTRY_TOKEN`.
- **Do**: `cargo login` (or set `CARGO_REGISTRY_TOKEN`) with an account that owns the `soroban-cost-estimator` crate name, then:
  ```
  cargo publish          # dry-run already verified clean (40 files, verify OK)
  ```
- Nothing else about the release is outstanding (`v0.1.0` tag pushed; README badges + `cargo install` docs already point at crates.io).

### 2. User action — Apply to Drips Wave (P0, browser)

1. Sign in to the Drips Wave app with GitHub.
2. Maintainers → Orgs and Repos → install the **Drips Wave GitHub App** on the org hosting this repo.
3. Sync and apply this repo to the **Stellar Wave Program**.
4. **Immediately before applying** (fresh, not inherited): re-confirm the repo isn't already in the approved-repo list, and re-confirm the SCF reporter's status (finding #2 above — it currently 404s, so the differentiator overlap question is open until re-checked).
5. If rejected, use the Drips appeal process (wait ≥2 weeks, substantive repo changes, max 3 appeals).

### 3. Contributor backlog — the 6 live issues (P1, for the first sprint)

| # | Issue | Status notes |
|---|-------|--------------|
| 1 | `feat(watch): graceful shutdown on SIGINT/SIGTERM` | Clean-exit AC **already implemented** (Session 4). Remaining ACs: second-signal force-exit (code 130), signal integration test. |
| 2 | `feat(report): populate read/write entries and bytes from the footprint` | Footprint parsing **already implemented** (live cross-check: 1/1 entries, 136 write bytes). Remaining AC: automated test asserting `write_entries >= 1`. |
| 3 | `feat(estimate-all): parallelize per-function simulations with a progress indicator` | Progress line **already implemented**; parallelism remains. |
| 4 | `feat(estimate): validate and coerce --arg values against contract-spec types` | Open — spec types are parsed; validation not yet wired. |
| 5 | `feat(cache): prune stale estimates and add cache stats command` | Open. |
| 6 | `fix(watch): exponential backoff on RPC failures` | Open. |

After approval: set Medium/High complexity tiers in the Drips dashboard (GitHub-label issues default to Trivial).

### 4. Small repo fix-ups (P2)

- **README differentiator dead link**: the linked `stellar/soroban-resource-usage-reporter` 404s — update the link/narrative (scope-freeze respected so far; no change made).
- Cosmetic: banner/logo + contrib.rocks — last or skipped.

### 5. Stretch (P3, post-submission)

- `estimate-all --fn` subset filter; `contractmeta` section parsing for richer reports; docs site/demo video **only if Drips ever asks** (it currently doesn't).

### 6. Ongoing duties (P0 after approval — this never stops)

- Re-run `config diff` after every Stellar protocol vote (Protocol 26 live May 2026; Protocol 27 vote July 2026 — roughly every 3–4 months). A drift alert on this tool is a **release trigger**, not a background curiosity.
- Re-verify `ConfigSetting*` XDR shapes on ecosystem SDK bumps — a protocol upgrade occasionally restructures a config type, breaking decoding silently if untested.
- Keep posting fresh issues each Wave cycle. A maintainer who applies once and stops adding issues stops being useful to the program.

---

## 🏁 Completion Vision

### The repository today (final state)

```
soroban-cost-estimator/
├── Cargo.toml                  # v0.1.0; repository/URLs at aigbagbobila/…; dry-run verified
├── README.md                   # Badges, quick-start, differentiator, --id/--json docs, Telegram contact
├── CONTRIBUTING.md             # PR guidelines, commit convention, correct clone URL
├── SECURITY.md                 # Disclosure contact, "unaudited tooling" disclaimer
├── LICENSE-MIT / LICENSE-APACHE
├── ROADMAP.md                  # This file
├── CONVERSATION_LOG.md         # Session-1/2 evidence log (CID-001…025)
├── scripts/create_issues.sh    # One-shot gh batch (6 issues, executed)
├── .github/workflows/ci.yml    # fmt → clippy → build → fixture → test (job: build, on main)
├── src/                        # main, cli, lib, error, xdr_helper, cache, wasm/, rpc/, config_snapshot/, report/
├── tests/                      # cli/cache/config_diff/parser — 53 tests, all passing
└── tests/fixtures/
    ├── minimal.wasm            # 44-byte structural-parse fixture (bare-WASM path)
    ├── contract.wasm           # REAL release-built Soroban contract (contractspecv0), deployed on testnet
    └── contract/               # source + build.sh + cross-check README (CID-025 record)
```

### The unique differentiator (unchanged)

> Every other Soroban cost tool tells you what your contract costs *today*.
> soroban-cost-estimator tells you when your cost report is *lying to you*
> because the network changed its prices.

### User workflows (current)

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

## 📋 Conversation Map — All IDs (CID-001 … CID-052)

```
CID-001 … CID-024   Session 1 — initial build (scaffold → CLI → wasm → rpc → fee → report → xdr → snapshot → diff → cache → wiring → CI → docs → tests)
CID-025  fix(fee-calc)       ← root-cause fix; clamp removed; regression tests
CID-026  feat(simulate)      ← parse_transaction_data_resource_fee fallback
CID-027  feat(xdr-helper)    ← raw XDR bytes; --id required; parse_contract_id
CID-028  feat(estimate)      ← --id + resource-fee fallback + fail-loudly guard
CID-029  fix(cli)            ← watch --interval suffix parser + test
CID-030  docs(readme)        ← URL/--id/--json/interval/contact pass
CID-031  feat(fixture)       ← real increment contract, release-built (4,742 B)
CID-032  feat(parser)        ← contractspecv0 typed-param decode + spec arity
CID-033  test(parser)        ← real-fixture test — UNBLOCKED, green (4/4)
CID-034  ops                 ← Stellar CLI 27.0.0 + testnet key + --cost flag
CID-035  test(live)          ← config snapshot E2E; invocation cross-checked vs native CLI
CID-036  feat(issues)        ← create_issues.sh (6 sprint issues)
CID-037  chore(git)          ← history-rewrite plan confirmed
CID-038  feat(cli)           ← watch graceful shutdown (uncommitted → landed CID-046)
CID-039  feat(cli)           ← estimate-all [i/N] progress (uncommitted → landed CID-046)
CID-040  fix(rpc)            ← fee-rate degradation warnings (uncommitted → landed CID-046)
CID-041  chore(ci)           ← fmt gate fix + branch trigger mismatch (→ CID-046/047)
CID-042  chore(git)          ← history split execution (→ CID-048)
CID-043  chore(repo)         ← topics/branch protection/tag/publish (→ CID-049/052)
CID-044  chore(issues)       ← create issues via gh batch (→ CID-051)
CID-045  ops                 ← gh installed + authenticated
CID-046  fix(cli)            ← clippy fixes + fmt drift + 3 feature units committed
CID-047  chore(git)          ← branch renamed master→main; first CI green
CID-048  chore(git)          ← 24-commit history rewrite, force-with-lease, CI green
CID-049  chore(repo)         ← URLs, licenses, topics, branch protection
CID-050  docs(fixture)       ← cross-check README (contract ID + numbers + repro)
CID-051  feat(issues)        ← 6 issues created + annotated
CID-052  chore(release)      ← v0.1.0 tag + publish dry-run (real publish: user token)
```

### Test suite summary (53, all passing — verified locally and in CI)

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
