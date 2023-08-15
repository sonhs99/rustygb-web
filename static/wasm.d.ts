/* tslint:disable */
/* eslint-disable */
/**
* @returns {number}
*/
export function get_frame_buffer_ptr(): number;
/**
* @param {number} key
*/
export function press_button(key: number): void;
/**
* @param {number} key
*/
export function release_button(key: number): void;
/**
*/
export class Emulator {
  free(): void;
/**
* @param {Uint8Array} rom
* @param {Uint8Array} frame_buffer
* @returns {Emulator}
*/
  static new(rom: Uint8Array, frame_buffer: Uint8Array): Emulator;
/**
* @param {Uint8Array} frame_buffer
* @returns {boolean}
*/
  step(frame_buffer: Uint8Array): boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_emulator_free: (a: number) => void;
  readonly emulator_new: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly emulator_step: (a: number, b: number, c: number, d: number) => number;
  readonly get_frame_buffer_ptr: () => number;
  readonly press_button: (a: number) => void;
  readonly release_button: (a: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
