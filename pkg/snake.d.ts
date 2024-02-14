/* tslint:disable */
/* eslint-disable */
/**
*/
export enum Direction {
  Up = 0,
  Right = 1,
  Down = 2,
  Left = 3,
}
/**
*/
export enum GameState {
  Won = 0,
  Lost = 1,
  Playing = 2,
}
/**
*/
export class World {
  free(): void;
/**
* @param {number} width
* @param {number} snake_idx
* @param {Direction} snake_dir
* @returns {World}
*/
  static new(width: number, snake_idx: number, snake_dir: Direction): World;
/**
*/
  step(): void;
/**
* @returns {number}
*/
  width(): number;
/**
* @returns {number}
*/
  size(): number;
/**
* @returns {number}
*/
  points(): number;
/**
* @returns {number | undefined}
*/
  get_reward_cell(): number | undefined;
/**
* @returns {number}
*/
  snake_head_idx(): number;
/**
*/
  start_game(): void;
/**
* @returns {GameState | undefined}
*/
  game_state(): GameState | undefined;
/**
* @returns {string}
*/
  game_state_text(): string;
/**
* @param {Direction} direction
*/
  change_snake_dir(direction: Direction): void;
/**
* @returns {number}
*/
  snake_length(): number;
/**
* @returns {number}
*/
  snake_cells(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_world_free: (a: number) => void;
  readonly world_new: (a: number, b: number, c: number) => number;
  readonly world_step: (a: number) => void;
  readonly world_width: (a: number) => number;
  readonly world_size: (a: number) => number;
  readonly world_points: (a: number) => number;
  readonly world_get_reward_cell: (a: number, b: number) => void;
  readonly world_snake_head_idx: (a: number) => number;
  readonly world_start_game: (a: number) => void;
  readonly world_game_state: (a: number) => number;
  readonly world_game_state_text: (a: number, b: number) => void;
  readonly world_change_snake_dir: (a: number, b: number) => void;
  readonly world_snake_length: (a: number) => number;
  readonly world_snake_cells: (a: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
