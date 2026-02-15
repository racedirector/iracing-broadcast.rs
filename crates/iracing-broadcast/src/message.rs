use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::util::pad_car_number;

/// Identifiers for broadcast messages recognized by the iRacing simulator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u32)]
#[serde(rename_all = "camelCase")]
pub enum BroadcastMessageType {
    /// Switch to a camera by position index.
    CameraSwitchPosition = 0,
    /// Switch to a camera by car number.
    CameraSwitchNumber,
    /// Update the camera state bitfield.
    CameraSetState,
    /// Change replay playback speed.
    ReplaySetPlaySpeed,
    /// Move to a specific replay position.
    ReplaySetPlayPosition,
    /// Perform a replay search.
    ReplaySearch,
    /// Toggle the replay state.
    ReplaySetState,
    /// Reload one or more textures.
    ReloadTextures,
    /// Issue a chat command.
    ChatCommand,
    /// Issue a pit command.
    PitCommand,
    /// Control telemetry capture.
    TelemetryCommand,
    /// Send a force-feedback command.
    FFBCommand,
    /// Search to a session-relative time.
    ReplaySearchSessionTime,
    /// Control screenshot or capture recording.
    VideoCapture,
}

impl From<BroadcastMessageType> for usize {
    fn from(value: BroadcastMessageType) -> Self {
        value as u32 as usize
    }
}

bitflags! {
    ///
    /// Bitfield of current camera state
    ///
    /// # Examples
    ///
    /// ```
    /// use iracing_broadcast::CameraState;
    ///
    /// let very_scenic = CameraState::UI_HIDDEN | CameraState::IS_SCENIC_ACTIVE;
    /// ```
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct CameraState: u32 {
        const IS_SESSION_SCREEN = 0x01;
        const IS_SCENIC_ACTIVE = 0x02;

        const CAM_TOOL_ACTIVE = 0x04;
        const UI_HIDDEN = 0x08;
        const USE_AUTO_SHOT_SELECTION = 0x10;
        const USE_TEMPORARY_EDITS = 0x20;
        const USE_KEY_ACCELERATION = 0x40;
        const USE_KEY_10X_ACCELERATION = 0x80;
        const USE_MOUSE_AIM_MODE = 0x100;
    }
}

/// Replay positioning behaviors when jumping within a session recording.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
#[serde(rename_all = "camelCase")]
pub enum ReplayPositionMode {
    /// Seek to the start of the session.
    Begin = 0,
    /// Seek relative to the current frame.
    Current,
    /// Seek to the end of the session.
    End,
}

impl From<ReplayPositionMode> for u16 {
    fn from(mode: ReplayPositionMode) -> Self {
        mode as u16
    }
}

/// High-level search controls for walking replay timelines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
#[serde(rename_all = "camelCase")]
pub enum ReplaySearchMode {
    /// Jump to the beginning of the session.
    ToStart = 0,
    /// Jump to the end of the session.
    ToEnd,
    /// Jump to the start of the previous session block.
    PreviousSession,
    /// Jump to the start of the next session block.
    NextSession,
    /// Jump back one lap.
    PreviousLap,
    /// Jump forward one lap.
    NextLap,
    /// Step one frame backward.
    PreviousFrame,
    /// Step one frame forward.
    NextFrame,
    /// Jump to the previous recorded incident.
    PreviousIncident,
    /// Jump to the next recorded incident.
    NextIncident,
}

impl From<ReplaySearchMode> for u16 {
    fn from(mode: ReplaySearchMode) -> Self {
        mode as u16
    }
}

/// Control commands for telemetry recording.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
#[serde(rename_all = "camelCase")]
pub enum TelemetryCommandMode {
    /// Stop capturing telemetry data.
    Stop = 0,
    /// Begin capturing telemetry data.
    Start,
    /// Restart telemetry capture from scratch.
    Restart,
}

impl From<TelemetryCommandMode> for u16 {
    fn from(mode: TelemetryCommandMode) -> Self {
        mode as u16
    }
}

/// Chat command options exposed by the broadcast protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
#[serde(rename_all = "camelCase")]
pub enum ChatCommandMode {
    /// Send a numbered chat macro.
    Macro = 0,
    /// Begin a chat session.
    Begin,
    /// Reply to an existing message.
    Reply,
    /// Cancel chat entry.
    Cancel,
}

impl From<ChatCommandMode> for u16 {
    fn from(mode: ChatCommandMode) -> Self {
        mode as u16
    }
}

/// Commands that adjust pit service behavior for the player's car.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PitCommandMode {
    /// Clear all pending pit service requests.
    Clear,
    /// Request a tearoff.
    Tearoff,
    /// Set fuel amount in gallons.
    Fuel(u8),
    /// Set left-front tire pressure in PSI.
    LF(u8),
    /// Set right-front tire pressure in PSI.
    RF(u8),
    /// Set left-rear tire pressure in PSI.
    LR(u8),
    /// Set right-rear tire pressure in PSI.
    RR(u8),
    /// Clear all tire change requests.
    ClearTires,
    /// Request a fast repair.
    FastRepair,
    /// Cancel tearoff request.
    ClearTearoff,
    /// Cancel fast repair request.
    ClearFastRepair,
    /// Cancel fuel request.
    ClearFuel,
}

impl PitCommandMode {
    /// Encode into (var1, var2) words as expected by the broadcast API.
    #[must_use]
    pub fn encode(self) -> (u16, u16) {
        match self {
            PitCommandMode::Clear => (0, 0),
            PitCommandMode::Tearoff => (1, 0),
            PitCommandMode::Fuel(level) => (2, u16::from(level)),
            PitCommandMode::LF(pressure) => (3, u16::from(pressure)),
            PitCommandMode::RF(pressure) => (4, u16::from(pressure)),
            PitCommandMode::LR(pressure) => (5, u16::from(pressure)),
            PitCommandMode::RR(pressure) => (6, u16::from(pressure)),
            PitCommandMode::ClearTires => (7, 0),
            PitCommandMode::FastRepair => (8, 0),
            PitCommandMode::ClearTearoff => (9, 0),
            PitCommandMode::ClearFastRepair => (10, 0),
            PitCommandMode::ClearFuel => (11, 0),
        }
    }
}

/// Control video capture and screenshot functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
#[serde(rename_all = "camelCase")]
pub enum VideoCaptureMode {
    /// Trigger a single screenshot.
    ScreenShot = 0,
    /// Begin capturing video.
    StartCapture,
    /// End capturing video.
    EndCapture,
    /// Toggle recording state.
    ToggleCapture,
    /// Show the capture timer UI.
    ShowTimer,
    /// Hide the capture timer UI.
    HideTimer,
}

impl From<VideoCaptureMode> for u16 {
    fn from(mode: VideoCaptureMode) -> Self {
        mode as u16
    }
}

pub trait BroadcastMessageProvider {
    fn to_payload(self) -> BroadcastPayload;

    fn to_message(self) -> (BroadcastMessageType, u16, u16, u16)
    where
        Self: Sized,
    {
        let payload = self.to_payload();
        (payload.message_type, payload.var1, payload.var2, payload.var3)
    }
}

/// A platform-neutral representation of one iRacing broadcast packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BroadcastPayload {
    pub message_type: BroadcastMessageType,
    pub var1: u16,
    pub var2: u16,
    pub var3: u16,
}

/// Versioned envelope used when exchanging payloads over JS bridges.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BroadcastEnvelope {
    pub version: u8,
    pub payload: BroadcastPayload,
}

impl BroadcastEnvelope {
    #[must_use]
    pub fn new(payload: BroadcastPayload) -> Self {
        Self { version: 1, payload }
    }
}

/// Messages that can be sent to the iRacing simulation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BroadcastMessage {
    CameraSwitchPosition(u8, u8, u8),
    CameraSwitchNumber(String, u8, u8),
    CameraSetState(u32),
    ReplaySetPlaySpeed(u8, bool),
    ReplaySetPlayPosition(ReplayPositionMode, u16),
    ReplaySearch(ReplaySearchMode),
    ReplaySetState,
    ReloadAllTextures,
    ReloadTextures(u8),
    ChatCommand(ChatCommandMode),
    ChatCommandMacro(u8),
    PitCommand(PitCommandMode),
    TelemetryCommand(TelemetryCommandMode),
    FFBCommand(u16),
    ReplaySearchSessionTime(u8, u16),
    VideoCapture(VideoCaptureMode),
}

impl BroadcastMessageProvider for BroadcastMessage {
    #[allow(clippy::too_many_lines)]
    fn to_payload(self) -> BroadcastPayload {
        match self {
            BroadcastMessage::CameraSwitchPosition(position, group, camera) => BroadcastPayload {
                message_type: BroadcastMessageType::CameraSwitchPosition,
                var1: position.into(),
                var2: group.into(),
                var3: camera.into(),
            },
            BroadcastMessage::CameraSwitchNumber(car_number, group, camera) => BroadcastPayload {
                message_type: BroadcastMessageType::CameraSwitchNumber,
                var1: pad_car_number(&car_number),
                var2: group.into(),
                var3: camera.into(),
            },
            BroadcastMessage::CameraSetState(camera_state_bits) => BroadcastPayload {
                message_type: BroadcastMessageType::CameraSetState,
                var1: camera_state_bits as u16,
                var2: 0,
                var3: 0,
            },
            BroadcastMessage::ReplaySetPlaySpeed(speed, slow_motion) => BroadcastPayload {
                message_type: BroadcastMessageType::ReplaySetPlaySpeed,
                var1: speed.into(),
                var2: slow_motion.into(),
                var3: 0,
            },
            BroadcastMessage::ReplaySetPlayPosition(mode, frame_number) => BroadcastPayload {
                message_type: BroadcastMessageType::ReplaySetPlayPosition,
                var1: mode.into(),
                var2: frame_number,
                var3: 0,
            },
            BroadcastMessage::ReplaySearch(mode) => BroadcastPayload {
                message_type: BroadcastMessageType::ReplaySearch,
                var1: mode.into(),
                var2: 0,
                var3: 0,
            },
            BroadcastMessage::ReplaySetState => BroadcastPayload {
                message_type: BroadcastMessageType::ReplaySetState,
                var1: 0,
                var2: 0,
                var3: 0,
            },
            BroadcastMessage::ReloadAllTextures => BroadcastPayload {
                message_type: BroadcastMessageType::ReloadTextures,
                var1: 0,
                var2: 0,
                var3: 0,
            },
            BroadcastMessage::ReloadTextures(car_index) => BroadcastPayload {
                message_type: BroadcastMessageType::ReloadTextures,
                var1: car_index.into(),
                var2: 0,
                var3: 0,
            },
            BroadcastMessage::ChatCommand(mode) => BroadcastPayload {
                message_type: BroadcastMessageType::ChatCommand,
                var1: mode.into(),
                var2: 0,
                var3: 0,
            },
            BroadcastMessage::ChatCommandMacro(macro_number) => BroadcastPayload {
                message_type: BroadcastMessageType::ChatCommand,
                var1: ChatCommandMode::Macro.into(),
                var2: macro_number.into(),
                var3: 0,
            },
            BroadcastMessage::PitCommand(pit_command_mode) => {
                let (var1, var2) = pit_command_mode.encode();
                BroadcastPayload {
                    message_type: BroadcastMessageType::PitCommand,
                    var1,
                    var2,
                    var3: 0,
                }
            }
            BroadcastMessage::TelemetryCommand(mode) => BroadcastPayload {
                message_type: BroadcastMessageType::TelemetryCommand,
                var1: mode.into(),
                var2: 0,
                var3: 0,
            },
            BroadcastMessage::FFBCommand(_value) => BroadcastPayload {
                message_type: BroadcastMessageType::FFBCommand,
                var1: 0,
                var2: 0,
                var3: 0,
            },
            BroadcastMessage::ReplaySearchSessionTime(session_number, session_time_ms) => {
                BroadcastPayload {
                    message_type: BroadcastMessageType::ReplaySearchSessionTime,
                    var1: session_number.into(),
                    var2: session_time_ms,
                    var3: 0,
                }
            }
            BroadcastMessage::VideoCapture(mode) => BroadcastPayload {
                message_type: BroadcastMessageType::VideoCapture,
                var1: mode.into(),
                var2: 0,
                var3: 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_switch_number_encodes_leading_zeros() {
        let payload = BroadcastMessage::CameraSwitchNumber("001".to_string(), 0, 0).to_payload();
        assert_eq!(payload.var1, 3001);
    }

    #[test]
    fn envelope_round_trip_json() {
        let envelope = BroadcastEnvelope::new(
            BroadcastMessage::PitCommand(PitCommandMode::Fuel(12)).to_payload(),
        );
        let encoded = serde_json::to_string(&envelope).expect("serialize envelope");
        let decoded: BroadcastEnvelope =
            serde_json::from_str(&encoded).expect("deserialize envelope");
        assert_eq!(decoded, envelope);
    }
}
