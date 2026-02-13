# WASM bindings implementation plan

This plan targets the `iracing-broadcast-wasm` crate and assumes it depends on
`iracing-broadcast` for shared protocol types and message modeling.

## Goals

1. Expose a stable, JavaScript-friendly API that mirrors the intent of the core crate.
2. Keep the wire-format/message semantics centralized in `iracing-broadcast`.
3. Support browser and non-browser WASM environments where practical.
4. Maintain clear error behavior and type conversions at the boundary.

## Constraints and architecture notes

- The current `iracing-broadcast` implementation is Windows-message based and has
  compile-time Windows gating in its public library code.
- Pure browser-based WASM cannot directly call Win32 APIs.
- Because of that, the WASM crate should start by exposing shared, platform-neutral
  protocol helpers and typed message construction, not direct simulator control.

## Phased roadmap

### Phase 1: Workspace and crate scaffolding (completed)

- [x] Convert repository into a Cargo workspace.
- [x] Add `iracing-broadcast-wasm` as a dedicated crate.
- [x] Add path dependency from WASM crate to `iracing-broadcast`.

### Phase 2: Extract platform-neutral core surface from `iracing-broadcast`

1. Identify types/modules that are protocol-only (enums, encoding helpers, validation).
2. Remove unnecessary coupling between those modules and the Windows client transport.
3. Ensure these protocol-only APIs compile for `wasm32-unknown-unknown`.
4. Keep Windows transport APIs behind `cfg(windows)` without blocking the full crate
   from being a dependency in WASM contexts.

Deliverable: `iracing-broadcast` builds for wasm targets with transport functionality
explicitly unavailable, while message modeling remains available.

### Phase 3: Introduce wasm-bindgen API layer

1. Add `wasm-bindgen` and optional `serde-wasm-bindgen` to `iracing-broadcast-wasm`.
2. Define a small JS-facing API surface:
   - constructors/builders for broadcast messages,
   - enum/string conversion helpers,
   - serializer output suitable for postMessage/network transport.
3. Map Rust errors to JS exceptions with clear, actionable messages.
4. Add crate-level docs with JS usage examples.

Deliverable: Generated WASM package can create and validate broadcast payloads from JS.

### Phase 4: Interop contracts and integration path

1. Decide on contract between WASM frontend and host runtime that can actually send
   iRacing messages (native daemon, websocket bridge, or Node addon).
2. Standardize payload schema (e.g., tagged JSON or compact binary) and version it.
3. Add round-trip tests to guarantee schema compatibility.

Deliverable: End-to-end documented flow from WASM-generated command to native sender.

### Phase 5: Quality gates and release process

1. Add CI checks for:
   - `cargo check --workspace`,
   - target-specific checks for `wasm32-unknown-unknown`,
   - linting/doc tests.
2. Add smoke test using `wasm-bindgen-test` where feasible.
3. Publish packaging instructions (`wasm-pack` and/or `cargo-component` if adopted).

Deliverable: repeatable, tested workflow for producing WASM bindings artifacts.

## Suggested initial task breakdown

1. Refactor `iracing-broadcast` to separate transport (`Client`) from protocol/model modules.
2. Replace crate-level non-Windows `compile_error!` with finer-grained `cfg` gating around
   Windows-only transport API.
3. In `iracing-broadcast-wasm`, expose one minimal message builder function via
   `#[wasm_bindgen]` to validate toolchain and packaging.
4. Add one browser-oriented example showing message construction and serialization.

## Open decisions to resolve

- Preferred JS API style: object-oriented vs functional builders.
- Serialization format for host bridge: JSON vs binary.
- Versioning strategy for payload contract shared with Node/native bridge.
- Whether to split protocol-only types into an additional crate in the future
  (if `iracing-broadcast` itself remains too Windows-centric).
