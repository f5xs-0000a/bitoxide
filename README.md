# Template Rust WASM Bitburner Library

This repository serves as a template and a starting point for Bitburner players to use Rust in Bitburner instead of JavaScript by compiling Rust into WebAssembly and then imported into Bitburner's JavaScript.

## Prerequisites

#### Rust

Install [Rust](https://rustup.rs/) on your computer. This will allow you to compile the code.

#### WebAssembly and `cargo-post`

Open your terminal and run these commands:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install --git https://github.com/phil-opp/cargo-post.git
```

These commands will allow you to compile to WebAssembly and do post-build processes. Both will be used.

## Building

To build the program, run the following command:

```bash
cargo post build --release --target wasm32-unknown-unknown
```

The build should be listening for a WebSocket connection at port 7953. Open your Bitburner, go to Options, go to Remote API, then set the port number to 7953 then press Connect. This will upload the JavaScript file to Bitburner.

After that, run your script in Bitburner.

```
run bitoxide.js
```

### Copy from `stdout` instead

If opening the Websocket Remote API every build is too much steps for you, one can consider to copy directly from `stdout` and paste the code into the Bitburner editor. This can be shortened by the use of tools like `xclip` or Windows' `clip` by piping the output of the build command into `xclip` or `clip`.

Use this environment variable to enable writing to `stdout`:

```bash
OUTPUT_MODE=stdout
```

### Enable debug symbols

To enable debug symbols and add tracing into panics, use the environment variable below:

```bash
DEBUG=true
```

Note that this can increase the output size by over ten times the original size.
