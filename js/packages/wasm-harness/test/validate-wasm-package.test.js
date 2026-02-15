import { describe, expect, it } from 'vitest';

import { validateWasmCrateScaffold } from '../src/validate-wasm-package.js';

describe('WASM crate scaffold', () => {
  it('preserves the marker type and crate docs expected by the JS harness', async () => {
    const result = await validateWasmCrateScaffold();

    expect(result.hasMarkerType).toBe(true);
    expect(result.hasScaffoldDocs).toBe(true);
  });
});
