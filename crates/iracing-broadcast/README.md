# iracing-broadcast

A lightweight Rust crate for sending iRacing broadcast window messages. It
encodes the simulator's documented broadcast protocol into typed enums so you
can trigger camera switches, replay searches, telemetry capture, and more from
your Rust application.

## Usage

```rust,no_run
use iracing_broadcast::{BroadcastMessage, Client, PitCommandMode};

fn main() -> Result<(), iracing_broadcast::BroadcastError> {
    let client = Client::new()?;

    // Request a tearoff from the pit crew.
    client.send_message(BroadcastMessage::PitCommand(PitCommandMode::Tearoff))?;
    Ok(())
}
```

The broadcast client is only available when targeting Windows because the
simulator communicates through Win32 window messages.
