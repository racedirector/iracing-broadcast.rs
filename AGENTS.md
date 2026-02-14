# AGENTS.md

## Mandatory quality gate before submission

Before submitting any code changes, opening a PR, or reporting task completion, agents **must** run and pass all of the following from the repository root:

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3. `cargo test --workspace --all-targets --all-features`

## Hard requirements

- Do **not** claim completion if any command above fails.
- Do **not** create a commit or PR until all required checks pass.
- If a check cannot run due to an environment limitation, clearly report the exact command, failure output, and limitation.

## Scope

These instructions apply to the entire repository.
