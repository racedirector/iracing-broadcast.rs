use crate::{
    BroadcastError, BroadcastMessageType, CameraState, ChatCommandMode, PitCommandMode,
    ReplayPositionMode, ReplaySearchMode, Result, TelemetryCommandMode, VideoCaptureMode,
    util::pad_car_number,
};

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
    OsStr::new(s)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub trait BroadcastMessageProvider {
    fn to_message(self) -> (BroadcastMessageType, u16, u16, u16);
}

/// Messages that can be sent to the iRacing simulation.
///
/// Each variant maps to the documented window message contract in the iRacing
/// SDK. Primitive parameters are passed through as-is and packed into the
/// `WPARAM`/`LPARAM` pairs expected by the simulator.
///
/// # Examples
///
/// ```
/// use iracing_broadcast::BroadcastMessage;
///
/// let _ = BroadcastMessage::CameraSwitchPosition(0, 0, 0);
/// let _ = BroadcastMessage::CameraSwitchNumber("001", 0, 0);
/// ```
pub enum BroadcastMessage {
    /// Switch to a specific camera group and camera index for a position.
    CameraSwitchPosition(u8, u8, u8),
    /// Switch to a specific camera group and camera index for a car number.
    CameraSwitchNumber(&'static str, u8, u8),
    /// Apply a new [`CameraState`] bitfield.
    CameraSetState(CameraState),
    /// Set the replay play speed, with an optional slow-motion toggle.
    ReplaySetPlaySpeed(u8, bool),
    /// Jump to a replay position, with the frame number encoded in `var2`.
    ReplaySetPlayPosition(ReplayPositionMode, u16),
    /// Perform a replay search according to the provided mode.
    ReplaySearch(ReplaySearchMode),
    /// Toggle the replay state on or off.
    ReplaySetState,
    /// Reload all textures.
    ReloadAllTextures,
    /// Reload textures for a specific car index.
    ReloadTextures(u8),
    /// Send a chat command.
    ChatCommand(ChatCommandMode),
    /// Send a chat macro by number.
    ChatCommandMacro(u8),
    /// Issue a pit command.
    PitCommand(PitCommandMode),
    /// Control telemetry recording.
    TelemetryCommand(TelemetryCommandMode),
    /// Send a force-feedback command.
    FFBCommand(u16),
    /// Search a replay to a specific session time.
    ReplaySearchSessionTime(u8, u16),
    /// Control video capture.
    VideoCapture(VideoCaptureMode),
}

impl BroadcastMessageProvider for BroadcastMessage {
    fn to_message(self) -> (BroadcastMessageType, u16, u16, u16) {
        match self {
            BroadcastMessage::CameraSwitchPosition(position, group, camera) => (
                BroadcastMessageType::CameraSwitchPosition,
                position.into(),
                group.into(),
                camera.into(),
            ),
            BroadcastMessage::CameraSwitchNumber(car_number, group, camera) => (
                BroadcastMessageType::CameraSwitchNumber,
                pad_car_number(&car_number),
                group.into(),
                camera.into(),
            ),
            BroadcastMessage::CameraSetState(camera_state) => (
                BroadcastMessageType::CameraSetState,
                camera_state.bits() as u16,
                0,
                0,
            ),
            BroadcastMessage::ReplaySetPlaySpeed(speed, slow_motion) => (
                BroadcastMessageType::ReplaySetPlaySpeed,
                speed.into(),
                slow_motion.into(),
                0,
            ),
            BroadcastMessage::ReplaySetPlayPosition(mode, frame_number) => (
                BroadcastMessageType::ReplaySetPlayPosition,
                mode.into(),
                frame_number.into(),
                0,
            ),
            BroadcastMessage::ReplaySearch(mode) => {
                (BroadcastMessageType::ReplaySearch, mode.into(), 0, 0)
            }
            BroadcastMessage::ReplaySetState => (BroadcastMessageType::ReplaySetState, 0, 0, 0),
            BroadcastMessage::ReloadAllTextures => (BroadcastMessageType::ReloadTextures, 0, 0, 0),
            BroadcastMessage::ReloadTextures(car_index) => {
                (BroadcastMessageType::ReloadTextures, car_index.into(), 0, 0)
            }
            BroadcastMessage::ChatCommand(mode) => {
                (BroadcastMessageType::ChatCommand, mode.into(), 0, 0)
            }
            BroadcastMessage::ChatCommandMacro(macro_number) => (
                BroadcastMessageType::ChatCommand,
                ChatCommandMode::Macro.into(),
                macro_number.into(),
                0,
            ),
            BroadcastMessage::PitCommand(pit_command_mode) => {
                let (var1, var2) = pit_command_mode.encode();
                (BroadcastMessageType::PitCommand, var1, var2, 0)
            }
            BroadcastMessage::TelemetryCommand(mode) => {
                (BroadcastMessageType::TelemetryCommand, mode.into(), 0, 0)
            }
            BroadcastMessage::FFBCommand(_value) => (
                BroadcastMessageType::FFBCommand,
                0,
                0, // (value * 65536).into(),
                0,
            ),
            BroadcastMessage::ReplaySearchSessionTime(session_number, session_time_ms) => (
                BroadcastMessageType::ReplaySearchSessionTime,
                session_number.into(),
                session_time_ms,
                0,
            ),
            BroadcastMessage::VideoCapture(mode) => {
                (BroadcastMessageType::VideoCapture, mode.into(), 0, 0)
            }
        }
    }
}

#[cfg(windows)]
#[derive(Debug, Copy, Clone)]
/// Handle for sending broadcast messages to a running iRacing simulator.
///
/// The client registers the well-known broadcast window message and can then
/// dispatch typed messages via [`send_message`]. All methods are Windows-only
/// because the simulator relies on the Win32 messaging subsystem.
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
                "Failed to register broadcast window message '{}'",
                BROADCAST_MESSAGE_NAME
            )));
        }

        Ok(Client { message_id: id })
    }

    /// Send a broadcast message to the iRacing simulator.
    pub fn send_message<M: BroadcastMessageProvider>(&self, message: M) -> Result<()> {
        let (broadcast_type, var1, var2, var3) = message.to_message();
        // Pack the low/high words to match the Windows broadcast contract.
        let wparam_value = broadcast_type as usize | ((var1 as usize) << 16);
        let lparam_value = var2 as isize | ((var3 as isize) << 16);

        unsafe {
            // Safety: iRacing expects these messages to be delivered to
            // HWND_BROADCAST using the ID obtained from RegisterWindowMessageW.
            // All parameter packing matches the documented protocol, so the
            // Win32 API receives well-formed data.
            SendNotifyMessageW(
                HWND_BROADCAST,
                self.message_id,
                WPARAM(wparam_value),
                LPARAM(lparam_value),
            )
            .map_err(|e| BroadcastError::windows_api_error("SendNotifyMessageW", e))
        }
    }
}

// Non-windows stub
#[cfg(not(windows))]
pub struct Client {
    _private: (),
}

#[cfg(not(windows))]
impl Client {
    /// Attempt to create a broadcast-message connection on non-Windows platforms.
    ///
    /// This always returns an error as message events can only be sent on windows.
    pub fn new() -> Result<Self> {
        Err(BroadcastError::unsupported_platform(
            "Broadcast Client",
            "Windows",
        ))
    }

    pub fn send_message<M: BroadcastMessageProvider>(&self, _message: M) -> Result<()> {
        Err(BroadcastError::unsupported_platform(
            "Broadcast Client Send Message",
            "Windows",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn test_broadcast() {
        let broadcast = Client::new();
        assert!(broadcast.is_ok());
    }

    #[cfg(not(windows))]
    #[test]
    fn test_unsupported_platform() {
        let broadcast = Client::new();
        assert!(broadcast.is_err());
    }

    #[cfg(windows)]
    #[test]
    fn test_message() {
        let broadcast = Client::new().expect("Could not register broadcast client");
        let _ = broadcast.send_message(BroadcastMessage::PitCommand(PitCommandMode::Tearoff));
    }
}
