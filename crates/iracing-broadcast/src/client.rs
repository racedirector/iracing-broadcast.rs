use crate::{BroadcastError, BroadcastMessageProvider, Result};

#[cfg(windows)]
use {
    windows::Win32::{
        Foundation::{LPARAM, WPARAM},
        UI::WindowsAndMessaging::{HWND_BROADCAST, RegisterWindowMessageW, SendNotifyMessageW},
    },
    windows::core::PCWSTR,
};

#[cfg(windows)]
const BROADCAST_MESSAGE_NAME: &str = r"IRSDK_BROADCASTMSG";

#[cfg(windows)]
fn wide_string(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

#[cfg(windows)]
#[derive(Debug, Copy, Clone)]
/// Handle for sending broadcast messages to a running iRacing simulator.
pub struct Client {
    message_id: u32,
}

#[cfg(windows)]
impl Client {
    /// Register the broadcast window message and create a sender handle.
    pub fn new() -> Result<Self> {
        let message: Vec<u16> = wide_string(BROADCAST_MESSAGE_NAME);

        let id = unsafe { RegisterWindowMessageW(PCWSTR::from_raw(message.as_ptr())) };

        if id == 0 {
            return Err(BroadcastError::connection_failed(format!(
                "Failed to register broadcast window message '{BROADCAST_MESSAGE_NAME}'"
            )));
        }

        Ok(Client { message_id: id })
    }

    /// Send a broadcast message to the iRacing simulator.
    pub fn send_message<M: BroadcastMessageProvider>(&self, message: M) -> Result<()> {
        let (broadcast_type, var1, var2, var3) = message.to_message();
        let wparam_value = broadcast_type as usize | ((var1 as usize) << 16);
        let lparam_value = i32::from(var2) | (i32::from(var3) << 16);

        unsafe {
            SendNotifyMessageW(
                HWND_BROADCAST,
                self.message_id,
                WPARAM(wparam_value),
                LPARAM(lparam_value as isize),
            )
            .map_err(|e| BroadcastError::windows_api_error("SendNotifyMessageW", e))
        }
    }
}

#[cfg(not(windows))]
pub struct Client {
    _private: (),
}

#[cfg(not(windows))]
impl Client {
    pub fn new() -> Result<Self> {
        Err(BroadcastError::unsupported_platform("Broadcast Client", "Windows"))
    }

    pub fn send_message<M: BroadcastMessageProvider>(&self, _message: M) -> Result<()> {
        Err(BroadcastError::unsupported_platform("Broadcast Client Send Message", "Windows"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(windows)]
    use crate::{BroadcastMessage, PitCommandMode};

    #[cfg(not(windows))]
    #[test]
    fn test_unsupported_platform() {
        let broadcast = Client::new();
        assert!(broadcast.is_err());
    }

    #[cfg(windows)]
    #[test]
    fn test_broadcast() {
        let broadcast = Client::new();
        assert!(broadcast.is_ok());
    }

    #[cfg(windows)]
    #[test]
    fn test_message() {
        let broadcast = Client::new().expect("Could not register broadcast client");
        let _ = broadcast.send_message(BroadcastMessage::PitCommand(PitCommandMode::Tearoff));
    }
}
