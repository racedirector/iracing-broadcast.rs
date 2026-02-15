import assert from 'node:assert/strict';
import test from 'node:test';

import { WasmPitCommand, build_pit_command } from '../pkg/iracing_broadcast_wasm.js';

test('build_pit_command encodes fuel command payload', () => {
  const payload = build_pit_command(WasmPitCommand.Fuel, 12);

  assert.equal(payload.message_type, 9);
  assert.equal(payload.var1, 2);
  assert.equal(payload.var2, 12);
  assert.equal(payload.var3, 0);
});
