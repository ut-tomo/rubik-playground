/* tslint:disable */
/* eslint-disable */
export function commutator(a_str: string, b_str: string): string;
export function conjugate(a_str: string, b_str: string): string;
export function invertAlgorithm(alg_str: string): string;
export class WasmCube {
  free(): void;
  [Symbol.dispose](): void;
  applyMove(move_str: string): void;
  getLegality(): any;
  applyAlgorithm(alg_str: string): void;
  getEdgeCycles(): any;
  getCornerCycles(): any;
  constructor();
  static identity(): WasmCube;
  getState(): any;
  isSolved(): boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmcube_free: (a: number, b: number) => void;
  readonly commutator: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly conjugate: (a: number, b: number, c: number, d: number) => [number, number, number, number];
  readonly invertAlgorithm: (a: number, b: number) => [number, number, number, number];
  readonly wasmcube_applyAlgorithm: (a: number, b: number, c: number) => [number, number];
  readonly wasmcube_applyMove: (a: number, b: number, c: number) => [number, number];
  readonly wasmcube_getCornerCycles: (a: number) => any;
  readonly wasmcube_getEdgeCycles: (a: number) => any;
  readonly wasmcube_getLegality: (a: number) => any;
  readonly wasmcube_getState: (a: number) => any;
  readonly wasmcube_identity: () => number;
  readonly wasmcube_isSolved: (a: number) => number;
  readonly wasmcube_new: () => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
