use iracing_broadcast::{
    BroadcastMessage, CameraState, ChatCommandMode, Client, PitCommandMode, ReplayPositionMode,
    ReplaySearchMode, TelemetryCommandMode, VideoCaptureMode,
};

pub fn main() {
    let broadcast = Client::new().expect("Could not create broadcast client");

    demo_camera_messages(&broadcast);
    demo_replay_messages(&broadcast);
    demo_chat_messages(&broadcast);
    demo_pit_messages(&broadcast);
    demo_telemetry_and_ffb(&broadcast);
    demo_video_capture(&broadcast);
}

fn demo_camera_messages(broadcast: &Client) {
    let _ = broadcast.send_message(BroadcastMessage::CameraSwitchPosition(0, 0, 0));
    let _ = broadcast.send_message(BroadcastMessage::CameraSwitchNumber("064", 1, 1));
    let scenic_camera = CameraState::IS_SCENIC_ACTIVE | CameraState::UI_HIDDEN;
    let _ = broadcast.send_message(BroadcastMessage::CameraSetState(scenic_camera));
}

fn demo_replay_messages(broadcast: &Client) {
    let _ = broadcast.send_message(BroadcastMessage::ReplaySetPlaySpeed(1, false));
    let _ = broadcast.send_message(BroadcastMessage::ReplaySetPlaySpeed(4, true));
    let _ = broadcast.send_message(BroadcastMessage::ReplaySetPlayPosition(
        ReplayPositionMode::Begin,
        0,
    ));
    let _ = broadcast.send_message(BroadcastMessage::ReplaySearch(
        ReplaySearchMode::NextIncident,
    ));
    let _ = broadcast.send_message(BroadcastMessage::ReplaySetState);
    let _ = broadcast.send_message(BroadcastMessage::ReplaySearchSessionTime(0, 15_000));
    let _ = broadcast.send_message(BroadcastMessage::ReloadAllTextures);
    let _ = broadcast.send_message(BroadcastMessage::ReloadTextures(12));
}

fn demo_chat_messages(broadcast: &Client) {
    for mode in [
        ChatCommandMode::Macro,
        ChatCommandMode::Begin,
        ChatCommandMode::Reply,
        ChatCommandMode::Cancel,
    ] {
        let _ = broadcast.send_message(BroadcastMessage::ChatCommand(mode));
    }
    let _ = broadcast.send_message(BroadcastMessage::ChatCommandMacro(3));
}

fn demo_pit_messages(broadcast: &Client) {
    let pit_modes = [
        PitCommandMode::Clear,
        PitCommandMode::Tearoff,
        PitCommandMode::Fuel(65),
        PitCommandMode::LF(26),
        PitCommandMode::RF(26),
        PitCommandMode::LR(26),
        PitCommandMode::RR(26),
        PitCommandMode::ClearTires,
        PitCommandMode::FastRepair,
        PitCommandMode::ClearTearoff,
        PitCommandMode::ClearFastRepair,
        PitCommandMode::ClearFuel,
    ];

    for mode in pit_modes {
        let _ = broadcast.send_message(BroadcastMessage::PitCommand(mode));
    }
}

fn demo_telemetry_and_ffb(broadcast: &Client) {
    let _ = broadcast.send_message(BroadcastMessage::TelemetryCommand(
        TelemetryCommandMode::Restart,
    ));
    let _ = broadcast.send_message(BroadcastMessage::FFBCommand(32_768));
}

fn demo_video_capture(broadcast: &Client) {
    // Start capture
    let _ = broadcast.send_message(BroadcastMessage::VideoCapture(
        VideoCaptureMode::StartCapture,
    ));

    // Stop capture
    let _ = broadcast.send_message(BroadcastMessage::VideoCapture(VideoCaptureMode::EndCapture));
}
