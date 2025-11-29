use bitflags::bitflags;

/// Identifiers for broadcast messages recognized by the iRacing simulator.
#[repr(u32)]
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
    #[derive(Default)]
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
#[repr(u16)]
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
#[repr(u16)]
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
#[repr(u16)]
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
#[repr(u16)]
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
    pub fn encode(self) -> (u16, u16) {
        match self {
            PitCommandMode::Clear => (0, 0),
            PitCommandMode::Tearoff => (1, 0),
            PitCommandMode::Fuel(level) => (2, level as u16),
            PitCommandMode::LF(pressure) => (3, pressure as u16),
            PitCommandMode::RF(pressure) => (4, pressure as u16),
            PitCommandMode::LR(pressure) => (5, pressure as u16),
            PitCommandMode::RR(pressure) => (6, pressure as u16),
            PitCommandMode::ClearTires => (7, 0),
            PitCommandMode::FastRepair => (8, 0),
            PitCommandMode::ClearTearoff => (9, 0),
            PitCommandMode::ClearFastRepair => (10, 0),
            PitCommandMode::ClearFuel => (11, 0),
        }
    }
}

/// Control video capture and screenshot functionality.
#[repr(u16)]
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
