# iracing-broadcast-wasm

WASM bindings for building iRacing broadcast payloads in JavaScript runtimes.

## Example

```js
import init, { build_pit_command } from "iracing-broadcast-wasm";

await init();
const envelope = build_pit_command("fuel", 12);
// { version: 1, payload: { messageType: "pitCommand", var1: 2, var2: 12, var3: 0 } }
```

The generated envelope is intended to be sent to a native host bridge
(Node addon, websocket daemon, or other process) that can deliver the message
through Win32 to iRacing.
