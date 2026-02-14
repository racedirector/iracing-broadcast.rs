//! WASM bindings for `iracing-broadcast`.
//!
//! This crate is intentionally a scaffold while the bindings design is being
//! finalized. See `docs/wasm-bindings-plan.md` at repository root for the
//! implementation plan.

#![allow(dead_code)]

use iracing_broadcast as _;

/// Marker type used while the WASM API surface is being designed.
pub struct WasmBindings;
