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

## Usage Notes

Using Rust in Bitburner (or any web-based WASM environment for that matter) has a lot of limitations. Most of these stem from the fact that you're running outside the context of an operating system, but rather inside the context of a WASM environment. Particularly, these limitations come to mind:

- Since the Rust WASM binary is run on an environment that isn't an operating system, the binary has a few limitations. While it has access to the heap memory (`std::collections`) and atomics (`std::sync`), that's about it. Below are parts of `std` that cannot be used (non-exhaustive):
    - `std::env`
    - `std::fs`
    - `std::process` (I don't think you can multithread)
    - `std::time`
        - You have to rely on `js_sys::Date` for time.
    - `std::thread`
        - This also means you cannot use `std::thread::sleep`. Instead, you have to rely on `ns.sleep` or `ns.asleep`.
- Because the async executor is not in Rust side, using any async library's `select` (e.g. `tokio::select!`) over futures (or probably just JS promises) that doesn't immediately resolve (like `ns.sleep`) will result in a hanging runtime.
- Calls from Rust to JS via the FFI is expensive. Try to minimize the number of calls made. Instead of using several methods to obtain related values, try to use methods that return a summary of values. An example of this is using `ns.getServer` over each single call of `ns.getServerMaxRam`, `ns.getServerMaxMoney`, etc.
    - Passing strings from Javascript to Rust and vice-versa is expensive because the strings have to be encoded from Javascript's UTF-16 to Rust's UTF-8 and vice versa.

Here are my personal recommendations in effectively creating your own solutions in Rust using this template:

- Read the [`wasm-bindgen` guide](https://rustwasm.github.io/wasm-bindgen/) and check the [`serde_wasm_bindgen` crate](https://docs.rs/serde-wasm-bindgen)
- Use `futures::future::CatchUnwind` to catch panics then print them using `ns.alert`
- Create a singular multitool script for all your needs in Bitburner. `clap` comes to mind in making the multitool very usable.
