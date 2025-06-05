/* tslint:disable */
/* eslint-disable */
export class SnakeGame {
  free(): void;
  constructor(canvas_id: string, grid_size: number);
  change_direction(dx: number, dy: number): void;
  /**
   * Advance the game by one tick (move, check collisions, eat food).
   */
  update(): void;
  /**
   * Draw the current state: clear canvas, draw food + snake, draw "Game Over" if ended.
   */
  render(): void;
  /**
   * Getter so JS can see if the game has ended.
   */
  readonly is_game_over: boolean;
  readonly score: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_snakegame_free: (a: number, b: number) => void;
  readonly snakegame_new: (a: number, b: number, c: number) => [number, number, number];
  readonly snakegame_change_direction: (a: number, b: number, c: number) => void;
  readonly snakegame_update: (a: number) => void;
  readonly snakegame_render: (a: number) => void;
  readonly snakegame_is_game_over: (a: number) => number;
  readonly snakegame_score: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
