# Template Rust WASM BitBurner Library

This repository serves as a template and a starting point for BitBurner players to use Rust in BitBurner instead of JavaScript by compiling Rust into WebAssembly and then imported into BitBurner's JavaScript.

## Prerequisites

- Install Rust

- Install wasm32-unknown-unknown

- Install `wasm-bindgen-cli`

## Building

Run the following command:

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web ./target/wasm32-unknown-unknown/release/bitoxide.wasm --out-dir ./wasm_output/
(echo -n 'export const wasm_b64 = "'; cat ./wasm_output/bitoxide_bg.wasm | base64 -w 0; echo '";') > ./wasm_output/wasm_source.js
```

Upon executing the commands above, a file `wasm_output/wasm_source.txt` will be created. This file can be copied into your BitBurner scripts.

## Running in BitBurner

Use this script below to execute the function exported from WASM:

```javascript
import { wasm_b64 } from "wasm_source.js";

/** @param {NS} ns */
export async function main(ns) {
    let wasm_binary = Uint8Array.from(atob(wasm_b64), c => c.charCodeAt(0));
    await WebAssembly.instantiate(wasm_binary, {})
        .then(module => {
            const { add } = module.instance.exports;

            ns.tprint(add(1, 2));
        });
}
```
