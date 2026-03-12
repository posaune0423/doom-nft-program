# Task Plan: Documentation English Translation (2026-03-12)

- [x] Inspect public markdown files and identify Japanese text that should be translated.
- [x] Translate `README.md` and `docs/*.md` to English while preserving technical meaning.
- [x] Verify that no Japanese text remains in the public documentation set.
- [x] Record the translation scope and verification results in the review section.

# Review: Documentation English Translation (2026-03-12)

- Rewrote `README.md` in English and aligned it with the current repository structure, Metaplex Core mint flow, and active Bun-based commands.
- Translated `docs/PRODUCT.md` to English while preserving the existing product context and relationship to `doom-index`.
- Translated `docs/DOOM_INDEX_NFT_MINT_REQUIREMENTS.md` to English while keeping the document structure, technical requirements, and JSON example intact.
- Verified with `rg -n "[ぁ-んァ-ン一-龯]" README.md docs --glob '*.md'`, which returned no matches.

# Task Plan: Test Module Reorganization (2026-03-12)

- [x] Inspect the current contract test coverage and map tests to the corresponding source modules.
- [x] Refactor `tests/src` so contract tests are organized by source-aligned modules instead of a single `lib.rs`.
- [x] Run the contract-focused verification commands after the reorganization.
- [x] Record the final structure and verification outcomes in the review section.

# Review: Test Module Reorganization (2026-03-12)

- Replaced the monolithic `tests/src/lib.rs` with a thin module entry plus shared helpers in `tests/src/test_context.rs`.
- Organized contract tests under `tests/src/instructions/` so each file now mirrors a program instruction module such as `initialize_collection`, `mint_doom_index_nft`, `transfer_admin`, and `update_base_metadata_url`.
- Split the previous combined admin-control coverage into source-aligned tests for `transfer_admin` and `update_base_metadata_url`, bringing the contract suite to 9 tests.
- Verified with `cargo fmt --all --check`, `cargo test -p tests --lib -- --list`, and `bun run test:contract`.

# Task Plan: CI Rust + Contract Tests (2026-03-12)

- [x] Inspect the existing CI workflow and local test scripts to confirm the minimum GitHub Actions changes required.
- [x] Update `.github/workflows/ci.yml` so Rust checks are explicit and contract tests run in CI.
- [x] Run the relevant local verification commands for the updated workflow.
- [x] Record the implementation result and verification outcomes in the review section.

# Review: CI Rust + Contract Tests (2026-03-12)

- Kept the existing CI setup intact and made the Rust-side test coverage explicit in `.github/workflows/ci.yml`.
- Replaced the generic `Run tests` step with separate `Run Rust tests` and `Run contract tests` steps so the workflow clearly shows both execution paths.
- Verified locally with `bun run format:ts:check`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --exclude tests`, and `bun run test:contract`.

# Task Plan

- [x] Reproduce the reported diagnostics with a deterministic command.
- [x] Fix Anchor feature wiring for `idl-build` so macro expansion works under `clippy --all-features`.
- [x] Remove the unused `initialize` instruction/context that triggers the lifetime diagnostic in macro expansion.
- [x] Run `cargo fmt`.
- [x] Run `cargo check -p doom-nft-program`.
- [x] Run `cargo clippy -p doom-nft-program --all-features --all-targets`.
- [x] Run `cargo test`.

# Notes

- `docs/` was not present in this repository, so implementation context came from source code and existing workspace config.
- The reported errors reproduce under `cargo clippy -p doom-nft-program --all-features --all-targets`, which matches the workspace rust-analyzer setting using `clippy`.

# Review

- Added `anchor-spl/idl-build` to the crate `idl-build` feature so Anchor's IDL-related macro expansion works for SPL account types under `clippy --all-features`.
- Added a direct `solana-program` dependency so macro-generated references to `solana_program` resolve under rust-analyzer/clippy.
- Removed the unused `initialize` instruction and empty `Initialize` accounts struct, which was the source of the bogus lifetime diagnostic around `Context<Initialize>`.
- Verification passed with `cargo fmt`, `cargo check -p doom-nft-program`, `cargo clippy -p doom-nft-program --all-features --all-targets`, and `cargo test`.

# Task Plan: Anchor Docs Summary (2026-03-11)

- [x] Read project context and current Anchor workspace files
- [x] Check latest official Anchor documentation for local deployment and client calls
- [x] Summarize concrete local deploy/call workflows for this repo

# Review: Anchor Docs Summary (2026-03-11)

- Confirmed against official Anchor docs that `anchor test` deploys workspace programs before running tests.
- Confirmed that `anchor test` auto-starts a local validator when the configured cluster is `localnet`.
- Confirmed that `anchor test --skip-local-validator` is the path for reusing an already running validator.
- Confirmed that `anchor shell` starts a Node.js shell with an Anchor client configured from local workspace config.
- Confirmed that `anchor build` writes artifacts under `target/deploy`, `target/idl`, and `target/types`.
- Repo-specific caveat: local `anchor-cli` is `0.32.1`, while this workspace uses `anchor-lang = 0.29.0` and `@coral-xyz/anchor = ^0.31.1`, so version mismatch should be called out.

# Task Plan: Tooling Setup (2026-03-11)

- [x] Inspect current workspace tooling, lockfiles, and existing CI/hook files
- [x] Add Rust formatter/linter configuration and package scripts
- [x] Add lefthook configuration and install wiring
- [x] Add GitHub Actions CI for format, lint, and tests
- [x] Run verification commands and record outcomes

# Review: Tooling Setup (2026-03-11)

- Added `rust-toolchain.toml` to require the stable toolchain with `rustfmt` and `clippy`.
- Switched Anchor's JS package manager setting from `yarn` to `bun` to match the existing lockfile and local workflow.
- Upgraded `prettier` to `3.8.1`, added `lefthook` `2.1.3`, and rewrote `package.json` scripts around `format`, `lint`, `test`, and `check`.
- Added `lefthook.yml` with pre-commit checks for Rust fmt, Clippy, and Prettier, plus a pre-push test gate.
- Added `.github/workflows/ci.yml` to run Bun install, Prettier check, Rust fmt, Clippy, and workspace tests on push/PR.
- Added `Makefile` targets for `install`, `build`, `test`, `lint`, `lint:fix`, `format`, and `format:fix`, using a catch-all alias rule for the colon forms.
- Verified locally with `bun run prepare`, `bun run format:ts:check`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace`, and `bun run check`.
- Verified locally with `make install`, `make format`, `make lint`, `make test`, `make build`, `make -n lint:fix`, and `make -n format:fix`.
- Remaining note: Rust commands emit a future-incompatibility warning from `solana-client v1.18.26`, but the checks pass.

# Task Plan: Create PR (2026-03-11)

- [x] Load project memory and inspect repository state
- [ ] Review the pending diff for tests, silent failures, comments, types, and general risks
- [ ] Create a feature branch from `main` for the PR
- [ ] Commit the current repository changes with a concise message
- [ ] Push the branch to `origin`
- [ ] Review the full PR diff against `origin/main`
- [ ] Create or update the GitHub PR with a concise body
- [ ] Poll CI and triage actionable feedback
- [ ] Check mergeability against `origin/main`
- [ ] Record PR outcome and any follow-up needed

# Review: Create PR (2026-03-11)

- In progress.

# Task Plan: NFT Mint Requirements Update (2026-03-12)

- [x] Inspect current repo docs and determine where to store the requirements/design document
- [x] Verify current official Metaplex Core guidance for metadata JSON and devnet validation expectations
- [x] Draft a repository-local requirements/design document for DOOM INDEX NFT mint v0.2
- [x] Incorporate deterministic URI minting, standard metadata, and 3D model support
- [x] Add explicit devnet verification requirements and acceptance criteria

# Review: NFT Mint Requirements Update (2026-03-12)

- Added `docs/DOOM_INDEX_NFT_MINT_REQUIREMENTS.md` as the primary requirements/design draft for the new Metaplex Core-based mint flow.
- Fixed the mint architecture around deterministic per-token metadata URIs and removed the earlier backend approval dependency from the spec.
- Standardized metadata around Metaplex Core JSON fields with `image`, `animation_url`, and `properties.files`, including a GLB example for 3D model support.
- Added devnet-specific functional requirements and an end-to-end validation checklist so the implementation can be verified outside localnet.

# Task Plan: Contract V1 Implementation (2026-03-12)

- [x] Upgrade the workspace toolchain and dependencies to Anchor 0.31.x / Solana 2.2.x / `mpl-core` 0.11.x.
- [x] Replace the existing SPL-token NFT program with Contract V1 state, errors, events, and instruction handlers.
- [x] Add Rust integration tests for config initialization, collection initialization, reservation, minting, and admin controls.
- [x] Refactor the program into a module structure modeled after official `mpl-core-anchor-examples` (`constants`, `error`, `events`, `instructions`, `state`, `utils`) instead of a monolithic `lib.rs`.
- [x] Align the implementation with the current requirements doc, including the separated contract-level `upgrade_authority` field and instruction if it remains part of the spec.
- [x] Re-run format and Rust tests after the refactor.
- [x] Investigate and document the current local SBF build blocker caused by Solana platform-tools cargo failing on `edition2024` transitive manifests.
- [x] Add devnet smoke scripts for initialize, reserve, and mint verification.
- [x] Run the full verification set that is feasible locally and record exact blockers for anything that cannot be completed.

# Notes: Contract V1 Implementation (2026-03-12)

- Reference project for structure and CPI style: `metaplex-foundation/mpl-core-anchor-examples`
- The current local BPF/SBF build path is blocked by the bundled Solana platform-tools cargo failing on a transitive `edition2024` dependency manifest (`rmp 0.8.15`) before tests can run under `cargo-build-sbf`.

# Review: Contract V1 Implementation (2026-03-12)

- Refactored the program from a monolithic `lib.rs` into `constants.rs`, `error.rs`, `events.rs`, `instructions/`, `state/`, and `utils.rs`, following the official `metaplex-foundation/mpl-core-anchor-examples` layout.
- Added the contract-level `upgrade_authority` field and `set_upgrade_authority` instruction so the on-chain config now matches the current requirements document.
- Kept the Core mint path compatible with Metaplex Core `CreateCollectionV2` / `CreateV2`, including the collection-specific rule that assets in a collection must not also set an explicit `update_authority`.
- Reworked the Rust integration tests to run the DOOM program as a host builtin while loading only the official `mpl_core` binary, which avoids the local `cargo-build-sbf` blocker and still exercises the real Core CPI path.
- Added devnet helper scripts under `scripts/devnet/` for `init`, `reserve`, and `mint`, with shared PDA/account encoding logic in `scripts/devnet/common.ts`.
- Verified with `cargo fmt --all`, `cargo clippy -p doom-nft-program --all-features --all-targets -- -D warnings`, `./scripts/test-contract-v1.sh`, and `bun x tsc --noEmit`.
- Remaining limitation: the local Solana platform-tools cargo still cannot build this dependency graph to SBF because it chokes on a transitive `edition2024` manifest. The contract test workflow now avoids that path by using the host processor for this program.

# Task Plan: Devnet Deploy Smoke Test (2026-03-12)

- [ ] Inspect the current Anchor/devnet configuration, wallet state, and existing smoke scripts.
- [ ] Confirm whether a fresh SBF build is possible from the current source tree or whether deployment must use the checked-in artifact.
- [ ] Deploy the DOOM NFT program to Solana devnet with Anchor-compatible tooling and record the final program id.
- [ ] Run the devnet smoke flow for config initialization, collection initialization, token reservation, and NFT minting.
- [ ] Inspect the on-chain/accounts outputs and metadata fetches to confirm the expected state transitions.
- [ ] Record exact commands, signatures, addresses, and blockers in the review section.

# Review: Devnet Deploy Smoke Test (2026-03-12)

- Aligned the repository to the actual deploy keypair returned by `anchor keys list` by switching the program id from `AavECgzCbVhHeBGAfcUgT1tYEC4N4B96E8XtF9H1fMGt` to `u929SRVcCFcGM2iyYkMykDRq7xW4N9ozEMU3Vo1hgfP` in `Anchor.toml`, `programs/doom-nft-program/src/lib.rs`, and `scripts/devnet/common.ts`.
- Added `anchor_version = "0.31.1"` and a `[programs.devnet]` entry to `Anchor.toml` so the workspace config matches the installed deploy key and Anchor crate version.
- Reproduced the original `anchor build` blocker under bundled `platform-tools v1.51` / `cargo 1.84.1`, where `rmp 0.8.15` failed due to the unstabilized `edition2024` manifest requirement.
- Installed newer Solana platform-tools and confirmed `cargo-build-sbf --tools-version v1.53 --manifest-path programs/doom-nft-program/Cargo.toml --sbf-out-dir target/deploy` succeeds, producing an up-to-date SBF artifact for the current source tree.
- Generated a dedicated devnet payer at `target/devnet/deployer.json` (`HmFV8YND3fAqhu1eP2Tii45sCUQu2FMUcaZdgmf1hmd9`) to avoid mutating the user's default Solana wallet.
- Deployment is currently blocked because every attempted faucet path left the payer at `0 SOL`: `solana airdrop` against `api.devnet.solana.com` failed with rate limiting for 5 / 2 / 1 / 0.5 / 0.1 SOL, and alternative public RPC paths were either paid-tier only, API-key-gated, or rate-limited as well.
- Until a devnet payer is funded, the remaining `anchor deploy` and on-chain smoke steps (`init`, `reserve`, `mint`, plus any admin instruction checks) cannot be executed.

# Task Plan: Local Fee Measurement (2026-03-12)

- [ ] Check the standard fee-reporting capabilities available in Anchor CLI and Solana CLI/RPC.
- [ ] Start a fresh local validator and fund the local deploy/test wallet.
- [ ] Deploy the program to localnet and record the deployment spend from balance deltas and transaction metadata.
- [ ] Execute local `initialize`, `reserve`, and `mint` flows with reachable metadata fixtures.
- [ ] Measure each instruction's network fee and rent-bearing account creations from balances and transaction details.
- [ ] Record the findings and the recommended standard tooling in the review section.

# Review: Local Fee Measurement (2026-03-12)

- Anchor CLI 0.32.1 の `deploy` / `test` help と公式 CLI docs を確認したが、Anchor 自体に標準の fee estimator / fee reporter はなかった。実測と見積りは Solana 側の `solana rent`, `getFeeForMessage`, `simulateTransaction`, `getTransaction.meta`, `solana confirm -v` を使うのが標準経路。
- `solana-test-validator` を `target/localnet-ledger` で起動し、`mpl-core` を `--upgradeable-program CoREEN... target/test-sbf/mpl_core_program.so none` で genesis にロードした。
- `python3 -m http.server 8123 --directory target/local-fixtures` で local metadata fixture を配信し、`BASE_METADATA_URL=http://127.0.0.1:8123` のまま `init -> reserve -> mint` を localnet で実行できた。
- `anchor deploy --provider.cluster http://127.0.0.1:8899 --provider.wallet target/devnet/deployer.json` は成功し、program id `u929...` / data length `331336 bytes` / signature `4K2N3oif...` を確認した。
- local deploy の総支出は `2.31019408 SOL` (`2310194080` lamports) で、内訳は program-data rent `2.30730264 SOL` + executable program account rent `0.00114144 SOL` + deploy transaction fees `0.00175 SOL` だった。
- `initialize_global_config` (`4uMhwnyi...`) は `3721640` lamports (`0.00372164 SOL`) で、内訳は fee `5000` + `GlobalConfig` rent `3716640`。`computeUnitsConsumed = 10633`。
- `initialize_collection` (`qVj2Rgi...`) は `1569040` lamports (`0.00156904 SOL`) で、内訳は fee `10000` + collection asset rent `1559040`。`computeUnitsConsumed = 15302`。
- `reserve_token_id` (`5rJadHXM...`) は `1243880` lamports (`0.00124388 SOL`) で、内訳は fee `5000` + reservation rent `1238880`。`computeUnitsConsumed = 10190`。
- `mint_doom_index_nft` (`2cevXVJF...`) は `3208240` lamports (`0.00320824 SOL`) で、内訳は fee `10000` + Core asset rent `3198240`。`computeUnitsConsumed = 25713`。
- localnet で作られた account sizes は `GlobalConfig = 406 bytes`, collection asset `96 bytes`, reservation `50 bytes`, minted Core asset `116 bytes`。対応する rent は `solana rent` の実測と一致した。

# Task Plan: Code Review Against main (2026-03-12)

- [ ] Inspect the diff against merge base `c1efa75aca4c3898e5ab0f6deeed665d7f9989df`.
- [ ] Validate changed code paths for behavioral regressions and test coverage gaps.
- [ ] Summarize prioritized, actionable findings with an overall correctness verdict.

# Task Plan: Review Fixes For Test Fixture + Wallet Defaults (2026-03-12)

- [x] Inspect the reported regressions in `scripts/build-test-sbf.sh`, `scripts/devnet/common.ts`, and `Anchor.toml`.
- [x] Replace the live Core program dump path with a version-pinned test fixture source.
- [x] Add or update coverage for devnet wallet resolution so the default path is exercised.
- [x] Align Anchor and devnet helper wallet defaults without depending on an untracked `target/` file.
- [x] Update docs that describe the contract-test fixture workflow if they no longer match behavior.
- [x] Run the relevant verification commands and record the exact outcomes.

# Review: Review Fixes For Test Fixture + Wallet Defaults (2026-03-12)

- Reverted `Anchor.toml` to the stable default wallet `~/.config/solana/id.json` so direct `anchor` commands no longer depend on the untracked `target/devnet/deployer.json`.
- Updated `scripts/devnet/common.ts` so wallet resolution now uses `ANCHOR_WALLET` first, then `provider.wallet` from `Anchor.toml`, then the stable default. This keeps the devnet helper scripts aligned with Anchor's configured wallet.
- Added `scripts/devnet/common.test.ts` to cover the wallet-resolution precedence and fallback behavior.
- Replaced the live `solana program dump` path in `scripts/build-test-sbf.sh` with a copy of a checked-in fixture.
- Added `tests/fixtures/mpl_core_program.so` and `tests/fixtures/mpl_core_program.so.sha256`, pinned to the official Metaplex Core release asset `release/core@0.9.10` with SHA-256 `604d401ea0c6c7b530c42274deeb903c953c1ef930bc5468497f60f3128493cc`.
- Updated `README.md` and `tests/fixtures/README.md` so the contract-test fixture workflow and provenance match the implementation.
- Verified with `bun test scripts/devnet/common.test.ts`, `bun x tsc --noEmit`, `bun run format:check`, `bun run lint`, and `bun run test:contract`.

# Task Plan: Verify And Fix Review Findings (2026-03-12)

- [x] Verify each reported finding against the current codebase and skip any already-fixed items.
- [x] Add or extend tests first for the confirmed behavior gaps in Rust and TypeScript helpers.
- [x] Implement the minimum code changes needed in the program, scripts, and package tooling.
- [x] Run focused and full verification commands.
- [x] Record which findings were fixed versus already resolved in the review section.

# Review: Verify And Fix Review Findings (2026-03-12)

- Added an explicit `typecheck` script and wired it into `check`, with `tsconfig.json` scoped to `scripts/**/*.ts` so the devnet scripts are covered by `tsc --noEmit`.
- Tightened program-side authority handling by requiring `transfer_admin` to be co-signed by the current `admin`, the current `upgrade_authority`, and the `new_admin` signer, which gives `GlobalConfig.upgrade_authority` a real privilege gate instead of leaving it unused.
- Removed the crate-wide deprecated lint suppression and replaced it with a wrapper module that localizes the Anchor 0.31 `#[program]` macro deprecation allowance while keeping `clippy -D warnings` green.
- Hardened `validate_base_metadata_url` to reject whitespace, non-HTTPS URLs, trailing slashes, and values over 256 bytes; aligned `scripts/devnet/init.ts` with the same constraints.
- Replaced the hard-coded asset name literal with `COLLECTION_NAME` and introduced `CollectionMismatch` so collection-address failures are reported accurately.
- Made `loadOrCreateKeypair` atomic with exclusive create mode `0o600`, added reserve retry logic on reservation contention, and fixed `scripts/test-contract-v1.sh` to run from `ROOT_DIR`.
- Updated `scripts/devnet/mint.ts` to decode the URI from the minted on-chain Metaplex Core asset account and to fall back from `HEAD` to `GET` when validating `image` and `animation_url`.
- Extended Rust and TypeScript coverage for reservation misuse, paused minting, transfer-admin handshakes, keypair permissions, reserve retries, on-chain URI decoding, and HEAD-to-GET asset validation.
- Verified and intentionally skipped two stale comments because the current code already addressed them: `scripts/build-test-sbf.sh` no longer reuses a dumped `mpl_core_program.so`, and the redundant `let config = global_config;` alias in `tests/src/lib.rs` was already gone after the earlier test-module split.
- Verified with `bun test scripts/devnet/common.test.ts scripts/devnet/mint.test.ts`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `bun run check`, and the `bun run check` path’s `./scripts/test-contract-v1.sh` contract suite.
