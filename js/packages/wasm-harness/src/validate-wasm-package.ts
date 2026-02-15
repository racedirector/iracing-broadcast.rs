import { readFile } from 'node:fs/promises';
import path from 'node:path';

const WASM_CRATE_LIB = path.resolve(process.cwd(), '..', 'crates', 'iracing-broadcast-wasm', 'src', 'lib.rs');

export type WasmScaffoldValidation = {
  hasMarkerType: boolean;
  hasScaffoldDocs: boolean;
};

export async function validateWasmCrateScaffold(): Promise<WasmScaffoldValidation> {
  const source = await readFile(WASM_CRATE_LIB, 'utf8');

  return {
    hasMarkerType: source.includes('pub struct WasmBindings;'),
    hasScaffoldDocs: source.includes('WASM bindings for `iracing-broadcast`.')
  };
}
