import { readFile } from 'node:fs/promises';
import path from 'node:path';

const NODE_CRATE_LIB = path.resolve(process.cwd(), '..', 'crates', 'iracing-broadcast-node', 'src', 'lib.rs');

export async function validateNodeCrateScaffold() {
  const source = await readFile(NODE_CRATE_LIB, 'utf8');

  return {
    hasMarkerType: source.includes('pub struct NodeBindings;'),
    hasScaffoldDocs: source.includes('Node.js bindings for `iracing-broadcast`.')
  };
}
