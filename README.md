# argon2id-wasm

Argon2id untuk browser yang dikompilasi ke WebAssembly. Repositori ini sudah menyertakan source Rust, binding JavaScript, dan file WASM hasil build.

Parameter hashing saat ini adalah Argon2id versi 1.3 dengan memori `19456 KiB`, waktu `2`, dan paralelisme `1`.

## Menggunakan di browser

Sajikan folder proyek melalui server HTTP, lalu impor binding JavaScript:

```js
import init, {
  hash_password,
  verify_password,
} from "./src/wasm/wasm_argon2id.js";

await init();

const hash = hash_password("kata-sandi-rahasia");
const valid = verify_password("kata-sandi-rahasia", hash);
```

`hash_password` menghasilkan hash PHC yang dapat disimpan, misalnya `$argon2id$v=19$...`. `verify_password` menghasilkan `true` atau `false`.

Contoh interaktif tersedia di `examples/browser.html`. Dari folder proyek, jalankan server HTTP, misalnya:

```bash
npx serve .
```

Lalu buka `/examples/browser.html`.

## Menggunakan di Node.js

Binding yang tersedia ditargetkan untuk browser. Untuk menjalankannya di Node.js, muat file WASM secara sinkron:

```js
import { readFileSync } from "node:fs";
import {
  initSync,
  hash_password,
  verify_password,
} from "./src/wasm/wasm_argon2id.js";

const wasm = readFileSync("./src/wasm/wasm_argon2id_bg.wasm");
initSync({ module: wasm });

const hash = hash_password("kata-sandi-rahasia");
console.log(verify_password("kata-sandi-rahasia", hash)); // true
```

## Build ulang WASM

Prasyarat: Rust dengan target `wasm32-unknown-unknown` dan `wasm-bindgen-cli` tersedia pada PATH.

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
npm run build:wasm
```

Hasil build ditulis ke `src/wasm/`, termasuk `wasm_argon2id.js` dan `wasm_argon2id_bg.wasm`.

### Build untuk Cloudflare Workers dan bundler

Cloudflare Workers tidak mengizinkan kompilasi WebAssembly dinamis saat runtime. Untuk Vite, SvelteKit, dan bundler Worker, gunakan target `bundler`:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.123
npm run build:wasm:cloudflare
```

Target ini menghasilkan binding yang mengimpor modul WASM secara statis sehingga bundler dapat memasukkannya ke Worker. Gunakan artifact dari `src/wasm/` untuk deployment Cloudflare; jangan gunakan output target `web` untuk runtime Worker.

## Pengujian source Rust

```bash
npm run test:rust
```
