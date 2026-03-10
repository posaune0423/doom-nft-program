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
