/* @ts-self-types="./wasm_argon2id.d.ts" */
import * as wasm from "./wasm_argon2id_bg.wasm";
import { __wbg_set_wasm } from "./wasm_argon2id_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    hash_password, start, verify_password
} from "./wasm_argon2id_bg.js";
