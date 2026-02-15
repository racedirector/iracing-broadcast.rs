//! WASM bindings for `iracing-broadcast`.
//!
//! The first scaffolded interface focuses on creating protocol payloads in a
//! JavaScript-friendly shape. Sending messages to iRacing still requires a
//! native host runtime.

use iracing_broadcast::{BroadcastMessageType, PitCommandMode};
use wasm_bindgen::prelude::*;

/// JS-facing representation of a packed iRacing broadcast message.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub struct EncodedBroadcastMessage {
    message_type: u32,
    var1: u16,
    var2: u16,
    var3: u16,
}

impl EncodedBroadcastMessage {
    fn pit_command(mode: PitCommandMode) -> Self {
        let (var1, var2) = mode.encode();

        Self { message_type: BroadcastMessageType::PitCommand as u32, var1, var2, var3: 0 }
    }
}

#[wasm_bindgen]
impl EncodedBroadcastMessage {
    /// Numeric message identifier understood by iRacing.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn message_type(&self) -> u32 {
        self.message_type
    }

    /// First 16-bit payload word.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn var1(&self) -> u16 {
        self.var1
    }

    /// Second 16-bit payload word.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn var2(&self) -> u16 {
        self.var2
    }

    /// Third 16-bit payload word.
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn var3(&self) -> u16 {
        self.var3
    }

    /// Serialize this payload into a JS value when the `serde` feature is enabled.
    #[cfg(feature = "serde")]
    pub fn to_js_value(&self) -> Result<JsValue, JsError> {
        serde_wasm_bindgen::to_value(self)
            .map_err(|error| JsError::new(&format!("Failed to serialize payload: {error}")))
    }
}

/// Supported pit commands for the WASM interface.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WasmPitCommand {
    Clear,
    Tearoff,
    Fuel,
    LF,
    RF,
    LR,
    RR,
    ClearTires,
    FastRepair,
    ClearTearoff,
    ClearFastRepair,
    ClearFuel,
}

/// Build a packed pit-command payload.
///
/// `value` is required for [`WasmPitCommand::Fuel`], [`WasmPitCommand::LF`],
/// [`WasmPitCommand::RF`], [`WasmPitCommand::LR`], and [`WasmPitCommand::RR`].
#[wasm_bindgen]
pub fn build_pit_command(
    command: WasmPitCommand,
    value: Option<u8>,
) -> Result<EncodedBroadcastMessage, JsError> {
    let mode = parse_pit_command_mode(command, value).map_err(JsError::new)?;
    Ok(EncodedBroadcastMessage::pit_command(mode))
}

fn parse_pit_command_mode(
    command: WasmPitCommand,
    value: Option<u8>,
) -> Result<PitCommandMode, &'static str> {
    match command {
        WasmPitCommand::Clear => Ok(PitCommandMode::Clear),
        WasmPitCommand::Tearoff => Ok(PitCommandMode::Tearoff),
        WasmPitCommand::Fuel => {
            value.map(PitCommandMode::Fuel).ok_or("Pit command 'Fuel' requires a numeric value.")
        }
        WasmPitCommand::LF => {
            value.map(PitCommandMode::LF).ok_or("Pit command 'LF' requires a numeric value.")
        }
        WasmPitCommand::RF => {
            value.map(PitCommandMode::RF).ok_or("Pit command 'RF' requires a numeric value.")
        }
        WasmPitCommand::LR => {
            value.map(PitCommandMode::LR).ok_or("Pit command 'LR' requires a numeric value.")
        }
        WasmPitCommand::RR => {
            value.map(PitCommandMode::RR).ok_or("Pit command 'RR' requires a numeric value.")
        }
        WasmPitCommand::ClearTires => Ok(PitCommandMode::ClearTires),
        WasmPitCommand::FastRepair => Ok(PitCommandMode::FastRepair),
        WasmPitCommand::ClearTearoff => Ok(PitCommandMode::ClearTearoff),
        WasmPitCommand::ClearFastRepair => Ok(PitCommandMode::ClearFastRepair),
        WasmPitCommand::ClearFuel => Ok(PitCommandMode::ClearFuel),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_pit_command_payload() {
        let payload =
            build_pit_command(WasmPitCommand::Fuel, Some(12)).expect("payload should encode");

        assert_eq!(payload.message_type(), BroadcastMessageType::PitCommand as u32);
        assert_eq!(payload.var1(), 2);
        assert_eq!(payload.var2(), 12);
        assert_eq!(payload.var3(), 0);
    }

    #[test]
    fn rejects_missing_command_value() {
        let Err(error) = parse_pit_command_mode(WasmPitCommand::Fuel, None) else {
            panic!("value is required for fuel");
        };

        assert!(
            error.contains("requires a numeric value"),
            "expected specific missing-value guidance"
        );
    }
}
