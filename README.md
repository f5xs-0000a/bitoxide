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

## Features

### `dynamic_ram_bypass`

Using this feature allows you to exploit the dynamic RAM in Bitburner without actually using it at the expense of using `eval`.

As far as the author of this crate is concerned, there are multiple levels of RAM usage in Bitburner:

1. The initial RAM check performed between invoking thet script and running it. This RAM check is the same as the one that displays how much RAM is used by the script in the in-game editor (located at the bottom of the game window).
2. The first dynamic RAM check when the script is invoked using `ns.run()`
3. The second dynamic RAM check which counts how much RAM is being used by the script during runtime.

This feature bypasses the first RAM check because the entire script will be wrapped inside an `eval`. The second RAM check isn't even run because there is no invocation of `ns.run()` nor should you need to invoke it. The third RAM will still keep track of a script's unique calls and add their associated RAM cost into the total RAM usage.

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
