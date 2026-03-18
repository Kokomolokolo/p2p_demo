/* tslint:disable */
/* eslint-disable */

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly main: (a: number, b: number) => number;
    readonly wasm_bindgen__closure__destroy__h29829587de8b59d7: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h0afe0ebc0d70701d: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h05326e220b1e3cc9: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__he22bbe179a194a8a: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h308e9fda69dd1bde: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h0423ce12d7187fbb: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hfc1af89f8776d9c8: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__h62f8abf57227e687: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1110a88a7df53611: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1110a88a7df53611_3: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1110a88a7df53611_4: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1110a88a7df53611_5: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1110a88a7df53611_6: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1110a88a7df53611_7: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1110a88a7df53611_8: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h1110a88a7df53611_9: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h444719a0d03dc84b: (a: number, b: number, c: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h630a177c69648c62: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h630a177c69648c62_13: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h630a177c69648c62_14: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h630a177c69648c62_15: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h93d5cba5f753051a: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h8c5a97a8a58d8ad1: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__ha765a71af0b38c23: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__he7d4bcf60e423bf8: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
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
