# AGENTS.md

## Mandatory quality gate before submission

Before submitting any code changes, opening a PR, or reporting task completion, agents **must** run and pass all of the following from the repository root:

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3. `cargo test --workspace --all-targets --all-features`
4. `pnpm --dir js lint`
5. `pnpm --dir js test`

## Hard requirements

- Do **not** claim completion if any command above fails.
- Do **not** create a commit or PR until all required checks pass.
- If a check cannot run due to an environment limitation, clearly report the exact command, failure output, and limitation.
- Changes touching `crates/iracing-broadcast-wasm` or `crates/iracing-broadcast-node` must include corresponding JS coverage in the harness under `js/packages/*-harness` and be validated with the JS checks above.

## Scope

These instructions apply to the entire repository.
