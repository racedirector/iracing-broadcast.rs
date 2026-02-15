import { describe, expect, it } from 'vitest';

import { validateNodeCrateScaffold } from '../src/validate-node-package.js';

describe('Node crate scaffold', () => {
  it('preserves the marker type and crate docs expected by the JS harness', async () => {
    const result = await validateNodeCrateScaffold();

    expect(result.hasMarkerType).toBe(true);
    expect(result.hasScaffoldDocs).toBe(true);
  });
});
