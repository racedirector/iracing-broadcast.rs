//! iRacing broadcast message services in Rust.
//!
//! This crate provides a thin, documented wrapper around the iRacing broadcast
//! window message protocol. Typical usage involves constructing a [`Client`]
//! and sending typed [`BroadcastMessage`] values:
//!
//! ```no_run
//! use iracing_broadcast::{BroadcastMessage, Client, PitCommandMode};
//!
//! let client = Client::new()?;
//! client.send_message(BroadcastMessage::PitCommand(PitCommandMode::Tearoff))?;
//! # Ok::<(), iracing_broadcast::BroadcastError>(())
//! ```
//!
//! The API is intentionally minimal and mirrors the documented iRacing SDK
//! constants. Consult the type-level documentation for details on each message
//! and its parameters.
//!
//! The `Client` transport is only functional on Windows. On non-Windows targets
//! the protocol/message types remain available for modeling and serialization.

mod client;
mod error;
mod message;
mod util;

pub use client::{BroadcastMessage, Client};
pub use error::*;
pub use message::{
    BroadcastMessageType, CameraState, ChatCommandMode, PitCommandMode, ReplayPositionMode,
    ReplaySearchMode, TelemetryCommandMode, VideoCaptureMode,
};
