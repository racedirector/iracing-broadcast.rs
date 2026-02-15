# iracing-broadcast.rs workspace

This repository is now organized as a Cargo workspace with multiple crates:

- `crates/iracing-broadcast`: core Rust crate for iRacing broadcast window messages.
- `crates/iracing-broadcast-wasm`: scaffold for browser/WASM bindings over the core crate.
- `crates/iracing-broadcast-node`: scaffold for Node.js bindings over the core crate.
- `examples/wasm-js`: JavaScript integration example for consuming generated WASM bindings.

## Next steps

The phased implementation plan for WASM bindings is documented in:

- `docs/wasm-bindings-plan.md`
