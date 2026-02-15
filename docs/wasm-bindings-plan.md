# WASM bindings implementation plan

This plan targets the `iracing-broadcast-wasm` crate and assumes it depends on
`iracing-broadcast` for shared protocol types and message modeling.

## Goals

1. Expose a stable, JavaScript-friendly API that mirrors the intent of the core crate.
2. Keep the wire-format/message semantics centralized in `iracing-broadcast`.
3. Support browser and non-browser WASM environments where practical.
4. Maintain clear error behavior and type conversions at the boundary.

## Constraints and architecture notes

- The `iracing-broadcast` transport is Windows-message based.
- Browser WASM cannot call Win32 APIs directly.
- The WASM crate therefore focuses on protocol helpers, typed message
  construction, and host-bridge payload serialization.

## Phased roadmap

### Phase 1: Workspace and crate scaffolding (completed)

- [x] Convert repository into a Cargo workspace.
- [x] Add `iracing-broadcast-wasm` as a dedicated crate.
- [x] Add path dependency from WASM crate to `iracing-broadcast`.

### Phase 2: Extract platform-neutral core surface from `iracing-broadcast` (completed)

- [x] Keep protocol-only enums and message modeling available cross-platform.
- [x] Keep Windows transport APIs isolated in `Client`.
- [x] Provide platform-neutral `BroadcastPayload`/`BroadcastEnvelope` types.
- [x] Ensure `iracing-broadcast` can be used for modeling on non-Windows targets.

### Phase 3: Introduce wasm-bindgen API layer (completed)

- [x] Add `wasm-bindgen` and `serde-wasm-bindgen` to `iracing-broadcast-wasm`.
- [x] Expose JS-facing builders for pit and chat commands.
- [x] Add JSON parse helper for host-bridge interop.
- [x] Map Rust/serde failures into explicit JS exceptions.
- [x] Add crate-level docs and usage examples.

### Phase 4: Interop contracts and integration path (completed)

- [x] Standardize on a versioned envelope contract:
  `{ version: 1, payload: { message_type, var1, var2, var3 } }`.
- [x] Add round-trip serialization tests in core and WASM crates.
- [x] Provide browser interop example using `postMessage`.

### Phase 5: Quality gates and release process (completed)

- [x] Keep repository-level fmt/clippy/test gates.
- [x] Add CI checks for `wasm32-unknown-unknown` target.
- [x] Ensure protocol crates pass lint/tests with all features.

## Implemented decisions

- JS API style: functional builders (`build_pit_command`, `build_chat_command`, etc.).
- Host bridge format: versioned tagged payload envelope.
- Payload versioning strategy: explicit top-level `version` field.
- Crate split: keep protocol and transport in `iracing-broadcast` for now;
  reconsider split if additional non-Windows host runtimes demand it.
