# WASM JS integration example

This example demonstrates consuming the `iracing-broadcast-wasm` package from JavaScript and
acts as a lightweight integration test harness.

## 1) Build WASM bindings for Node.js

From the repository root:

```bash
wasm-pack build crates/iracing-broadcast-wasm --target nodejs --out-dir ../../examples/wasm-js/pkg
```

## 2) Run integration test

From the repository root:

```bash
npm test -w examples/wasm-js
```

The test validates that `build_pit_command` produces the expected packed protocol values.
