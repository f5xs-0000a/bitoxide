# Template Rust WASM BitBurner Library

This repository serves as a template and a starting point for BitBurner players to use Rust in BitBurner instead of JavaScript by compiling Rust into WebAssembly and then imported into BitBurner's JavaScript.

## Prerequisites

#### Rust

Install [Rust](https://rustup.rs/) on your computer. This will allow you to compile the code.

#### `wasm32-unknown-unknown` toolchain

Open your terminal and run this command:

```bash
rustup target add wasm32-unknown-unknown
```

This will allow you to compile to WebAssembly.

#### `wasm-bindgen-cli`

On the terminal again, run this command:

```bash
cargo install wasm-bindgen-cli
```

## Building

In order to build the program into WASM, run the following:

```bash
cargo build --release --target wasm32-unknown-unknown
```

This will create a file `./target/wasm32-unknown-unknown/release/bitoxide.wasm`. You will then strip down the binary using the following command:

```bash
wasm-bindgen --target web ./target/wasm32-unknown-unknown/release/bitoxide.wasm --out-dir ./wasm_output/
```

After that, a `./wasm_output/bitoxide_bg.wasm` file will be created. You will then convert it into base64 using the following command:

```bash
(echo -n 'export const wasm_b64 = "'; cat ./wasm_output/bitoxide_bg.wasm | base64 -w 0; echo '";') > ./wasm_source.js
```

## Running in BitBurner

After following the Building steps above, you will now have two files that you'll need: `wasm_source.js` and `bitoxide.js`. Copy both of these files into BitBurner while retaining their filenames.

And all you have to do is to run `bitoxide.js` in BitBurner:

```bash
run bitoxide.js
```
