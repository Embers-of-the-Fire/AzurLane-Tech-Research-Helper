/* tslint:disable */
/* eslint-disable */
/**
*/
export function greet(): void;
/**
* @returns {any}
*/
export function predef_restriction(): any;
/**
* @returns {any}
*/
export function predef_reference_value(): any;
/**
* @param {number} doubloon_ratio
* @param {number} cube_ratio
* @param {number} cong_chips_ratio
* @param {number} time_ratio
* @param {number} ultra_blp_ratio
* @param {number} ultra_equip_ratio
* @param {number} fni_5_super_r
* @param {number} fni_5_ultra_r
* @param {number} fni_f
* @param {boolean} do_data_collection
* @param {boolean} do_research_assignment
* @returns {any}
*/
export function build_restriction(doubloon_ratio: number, cube_ratio: number, cong_chips_ratio: number, time_ratio: number, ultra_blp_ratio: number, ultra_equip_ratio: number, fni_5_super_r: number, fni_5_ultra_r: number, fni_f: number, do_data_collection: boolean, do_research_assignment: boolean): any;
/**
* @param {number} doubloon
* @param {number} cube
* @param {number} time_of_an_hour
* @param {number} super_rare
* @param {number} ultra_rare
* @param {number} ultra_equip
* @param {number} cong_chips
* @param {number} time_ratio
* @returns {any}
*/
export function build_reference_value(doubloon: number, cube: number, time_of_an_hour: number, super_rare: number, ultra_rare: number, ultra_equip: number, cong_chips: number, time_ratio: number): any;
/**
* @param {any} rest
* @param {any} raw_ref
* @param {number} refer_v
* @param {number} limit
* @returns {any}
*/
export function calc(rest: any, raw_ref: any, refer_v: number, limit: number): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly greet: () => void;
  readonly predef_restriction: () => number;
  readonly predef_reference_value: () => number;
  readonly build_restriction: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number) => number;
  readonly build_reference_value: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly calc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
