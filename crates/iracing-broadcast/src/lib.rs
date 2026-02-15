//! iRacing broadcast message services in Rust.
//!
//! This crate provides a thin, documented wrapper around the iRacing broadcast
//! window message protocol.
//!
//! The `Client` transport is only functional on Windows. On non-Windows targets
//! protocol/message types remain available for modeling and serialization.

mod client;
mod error;
mod message;
mod util;

pub use client::Client;
pub use error::*;
pub use message::{
    BroadcastEnvelope, BroadcastMessage, BroadcastMessageProvider, BroadcastMessageType,
    BroadcastPayload, CameraState, ChatCommandMode, PitCommandMode, ReplayPositionMode,
    ReplaySearchMode, TelemetryCommandMode, VideoCaptureMode,
};
