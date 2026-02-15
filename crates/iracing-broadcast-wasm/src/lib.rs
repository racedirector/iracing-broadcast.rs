//! WASM bindings for `iracing-broadcast`.
//!
//! The bindings expose protocol/message construction for browser and non-browser
//! runtimes. Transport to iRacing itself must happen in a native host process.

use iracing_broadcast::{
    BroadcastEnvelope, BroadcastMessage, BroadcastMessageProvider, ChatCommandMode, PitCommandMode,
};
use wasm_bindgen::prelude::*;

fn build_pit_command_envelope(kind: &str, value: Option<u8>) -> Result<BroadcastEnvelope, String> {
    let mode = match kind {
        "clear" => PitCommandMode::Clear,
        "tearoff" => PitCommandMode::Tearoff,
        "fuel" => PitCommandMode::Fuel(value.ok_or_else(|| "fuel requires value".to_string())?),
        "clearFuel" => PitCommandMode::ClearFuel,
        "fastRepair" => PitCommandMode::FastRepair,
        "clearFastRepair" => PitCommandMode::ClearFastRepair,
        other => {
            return Err(format!(
                "unsupported pit command '{other}' (supported: clear, tearoff, fuel, clearFuel, fastRepair, clearFastRepair)"
            ));
        }
    };

    Ok(BroadcastEnvelope::new(BroadcastMessage::PitCommand(mode).to_payload()))
}

fn parse_envelope_json_to_envelope(input: &str) -> Result<BroadcastEnvelope, String> {
    serde_json::from_str(input).map_err(|err| format!("invalid envelope JSON: {err}"))
}

fn build_chat_macro_envelope(macro_number: u8) -> BroadcastEnvelope {
    BroadcastEnvelope::new(BroadcastMessage::ChatCommandMacro(macro_number).to_payload())
}

fn build_chat_command_envelope(mode: &str) -> Result<BroadcastEnvelope, String> {
    let mapped = match mode {
        "begin" => ChatCommandMode::Begin,
        "reply" => ChatCommandMode::Reply,
        "cancel" => ChatCommandMode::Cancel,
        other => {
            return Err(format!(
                "unsupported chat mode '{other}' (supported: begin, reply, cancel)"
            ));
        }
    };

    Ok(BroadcastEnvelope::new(BroadcastMessage::ChatCommand(mapped).to_payload()))
}

#[wasm_bindgen]
pub fn build_pit_command(kind: &str, value: Option<u8>) -> Result<JsValue, JsError> {
    let envelope = build_pit_command_envelope(kind, value).map_err(|err| JsError::new(&err))?;
    serde_wasm_bindgen::to_value(&envelope)
        .map_err(|err| JsError::new(&format!("failed to serialize envelope: {err}")))
}

#[wasm_bindgen]
pub fn parse_envelope_json(input: &str) -> Result<JsValue, JsError> {
    let envelope = parse_envelope_json_to_envelope(input).map_err(|err| JsError::new(&err))?;
    serde_wasm_bindgen::to_value(&envelope)
        .map_err(|err| JsError::new(&format!("failed to serialize envelope: {err}")))
}

#[wasm_bindgen]
pub fn build_chat_macro(macro_number: u8) -> Result<JsValue, JsError> {
    let envelope = build_chat_macro_envelope(macro_number);
    serde_wasm_bindgen::to_value(&envelope)
        .map_err(|err| JsError::new(&format!("failed to serialize envelope: {err}")))
}

#[wasm_bindgen]
pub fn build_chat_command(mode: &str) -> Result<JsValue, JsError> {
    let envelope = build_chat_command_envelope(mode).map_err(|err| JsError::new(&err))?;
    serde_wasm_bindgen::to_value(&envelope)
        .map_err(|err| JsError::new(&format!("failed to serialize envelope: {err}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pit_fuel_builds_expected_payload() {
        let envelope = build_pit_command_envelope("fuel", Some(15)).expect("build payload");
        assert_eq!(envelope.version, 1);
        assert_eq!(envelope.payload.var1, 2);
        assert_eq!(envelope.payload.var2, 15);
    }

    #[test]
    fn parse_json_round_trip() {
        let input =
            r#"{"version":1,"payload":{"message_type":"pitCommand","var1":1,"var2":0,"var3":0}}"#;
        let envelope = parse_envelope_json_to_envelope(input).expect("parse json");
        assert_eq!(envelope.payload.var1, 1);
    }
}
