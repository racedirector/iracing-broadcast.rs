//! WASM bindings for `iracing-broadcast`.
//!
//! The first scaffolded interface focuses on creating protocol payloads in a
//! JavaScript-friendly shape. Sending messages to iRacing still requires a
//! native host runtime.

use iracing_broadcast::PitCommandMode;
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

        Self { message_type: 9, var1, var2, var3: 0 }
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

/// Build a packed pit-command payload.
///
/// `command` accepts one of:
/// `clear`, `tearoff`, `fuel`, `lf`, `rf`, `lr`, `rr`, `clear_tires`,
/// `fast_repair`, `clear_tearoff`, `clear_fast_repair`, or `clear_fuel`.
///
/// Commands `fuel`, `lf`, `rf`, `lr`, and `rr` require `value`.
#[wasm_bindgen]
pub fn build_pit_command(
    command: &str,
    value: Option<u8>,
) -> Result<EncodedBroadcastMessage, JsError> {
    let mode = parse_pit_command_mode(command, value).map_err(JsError::new)?;
    Ok(EncodedBroadcastMessage::pit_command(mode))
}

fn parse_pit_command_mode(
    command: &str,
    value: Option<u8>,
) -> Result<PitCommandMode, &'static str> {
    match command {
        "clear" => Ok(PitCommandMode::Clear),
        "tearoff" => Ok(PitCommandMode::Tearoff),
        "fuel" => {
            value.map(PitCommandMode::Fuel).ok_or("Pit command 'fuel' requires a numeric value.")
        }
        "lf" => value.map(PitCommandMode::LF).ok_or("Pit command 'lf' requires a numeric value."),
        "rf" => value.map(PitCommandMode::RF).ok_or("Pit command 'rf' requires a numeric value."),
        "lr" => value.map(PitCommandMode::LR).ok_or("Pit command 'lr' requires a numeric value."),
        "rr" => value.map(PitCommandMode::RR).ok_or("Pit command 'rr' requires a numeric value."),
        "clear_tires" => Ok(PitCommandMode::ClearTires),
        "fast_repair" => Ok(PitCommandMode::FastRepair),
        "clear_tearoff" => Ok(PitCommandMode::ClearTearoff),
        "clear_fast_repair" => Ok(PitCommandMode::ClearFastRepair),
        "clear_fuel" => Ok(PitCommandMode::ClearFuel),
        _ => Err("Unsupported pit command. See build_pit_command docs for supported values."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_pit_command_payload() {
        let payload = build_pit_command("fuel", Some(12)).expect("payload should encode");

        assert_eq!(payload.message_type(), 9);
        assert_eq!(payload.var1(), 2);
        assert_eq!(payload.var2(), 12);
        assert_eq!(payload.var3(), 0);
    }

    #[test]
    fn rejects_missing_command_value() {
        let Err(error) = parse_pit_command_mode("fuel", None) else {
            panic!("value is required for fuel");
        };

        assert!(
            error.contains("requires a numeric value"),
            "expected specific missing-value guidance"
        );
    }
}
