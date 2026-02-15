import { describe, expect, it } from 'vitest';

import { packBroadcastPayload } from '../index.js';

describe('packBroadcastPayload', () => {
  it('returns a JS-friendly payload shape', () => {
    expect(packBroadcastPayload(9, 2, 12, 0)).toEqual({
      messageType: 9,
      var1: 2,
      var2: 12,
      var3: 0
    });
  });
});
