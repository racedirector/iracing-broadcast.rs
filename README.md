# iracing-broadcast.rs workspace

This repository is organized as a Cargo workspace with multiple crates:

- `crates/iracing-broadcast`: core Rust crate for iRacing broadcast transport and protocol modeling.
- `crates/iracing-broadcast-wasm`: wasm-bindgen bindings for constructing versioned protocol payloads.
- `crates/iracing-broadcast-node`: scaffold for future Node.js bindings over the core crate.

## WASM bindings plan

The phased implementation plan and progress notes are in:

- `docs/wasm-bindings-plan.md`
