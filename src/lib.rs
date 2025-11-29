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

#[cfg(not(windows))]
compile_error!(
    "iracing-broadcast currently only supports Windows targets because the iRacing \
     broadcast API is delivered via Windows messages. Please build with a Windows \
     target triple."
);

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
