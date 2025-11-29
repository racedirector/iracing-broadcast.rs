//! Error types for broadcast messages.
//!
//! This module provides comprehensive error handling for the broadcast message
//! library. All errors implement the `std::error::Error` trait and include
//! structured context for debugging and recovery guidance.
//!
//! ## Error Categories
//!
//! - **Connection Errors**: Issues connecting to iRacing.
//! - **Windows API Errors**: Platform-specific Windows operation failures
//!
//! ## Recovery and Retry
//!
//! Errors provide methods to determine if they are recoverable
//!
//! ## Helper Constructors
//!
//! Use helper methods for common error scenarios:
//!
//! ```rust
//! use iracing_broadcast::BroadcastError;
//!
//! // Connection failures
//! let conn_error = BroadcastError::connection_failed("iRacing not detected");
//!
//! // Unsupported platform
//! let unsupported_error = BroadcastError::unsupported_platform("Message registration", "Windows");
//! ```

use thiserror::Error;

#[cfg(windows)]
use windows_core as core;

pub type Result<T, E = BroadcastError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum BroadcastError {
    #[error("Failed to connect to iRacing: {reason}")]
    Connection { reason: String },

    #[error("{feature} is only available on {required_platform}")]
    UnsupportedPlatform {
        feature: String,
        required_platform: String,
    },

    #[error("Windows API error: {operation}")]
    #[cfg(windows)]
    WindowsApi {
        operation: String,
        #[source]
        source: core::Error,
    },
}

impl BroadcastError {
    pub fn is_retryable(&self) -> bool {
        match self {
            BroadcastError::Connection { .. } => true,
            BroadcastError::UnsupportedPlatform { .. } => false,
            #[cfg(windows)]
            BroadcastError::WindowsApi { .. } => true,
        }
    }

    pub fn recovery_suggestions(&self) -> Vec<&'static str> {
        match self {
            BroadcastError::Connection { .. } => vec![
                "Ensure iRacing is running",
                "Check Windows permissions for shared memory access",
                "Verify iRacing SDK version compatibility",
                "Try restarting iRacing",
            ],
            BroadcastError::UnsupportedPlatform { .. } => vec![
                "Use platform-appropriate features",
                "Check documentation for platform requirements",
            ],
            #[cfg(windows)]
            BroadcastError::WindowsApi { .. } => vec![
                "Check Windows API permissions",
                "Verify system resources availability",
                "Check Windows version compatibility",
            ],
        }
    }

    /// Helper constructor for connection errors.
    pub fn connection_failed(reason: impl Into<String>) -> Self {
        BroadcastError::Connection {
            reason: reason.into(),
        }
    }

    /// Helper constructor for Windows API errors.
    #[cfg(windows)]
    pub fn windows_api_error(operation: impl Into<String>, source: core::Error) -> Self {
        BroadcastError::WindowsApi {
            operation: operation.into(),
            source,
        }
    }

    /// Helper constructor for unsupported platform errors.
    pub fn unsupported_platform(
        feature: impl Into<String>,
        required_platform: impl Into<String>,
    ) -> Self {
        BroadcastError::UnsupportedPlatform {
            feature: feature.into(),
            required_platform: required_platform.into(),
        }
    }
}

#[cfg(windows)]
impl From<core::Error> for BroadcastError {
    fn from(err: core::Error) -> Self {
        BroadcastError::WindowsApi {
            operation: "Unknown Windows operation".to_string(),
            source: err,
        }
    }
}
