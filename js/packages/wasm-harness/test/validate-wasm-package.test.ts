import { describe, expect, it } from 'vitest';

import { validateWasmCrateScaffold } from '../src/validate-wasm-package.ts';

describe('WASM crate scaffold', () => {
  it('preserves the marker type and crate docs expected by the TS harness', async () => {
    const result = await validateWasmCrateScaffold();

    expect(result.hasMarkerType).toBe(true);
    expect(result.hasScaffoldDocs).toBe(true);
  });
});
