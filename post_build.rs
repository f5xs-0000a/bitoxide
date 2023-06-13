use std::{
    env::var,
    fs::OpenOptions,
    io::{
        BufRead as _,
        BufReader,
        Read as _,
        Result as IoResult,
        Write,
    },
    path::{
        Path,
        PathBuf,
    },
    process::Command,
};

use base64::write::EncoderWriter;
use toml::Value;
use tungstenite::{
    handshake::server::{
        ErrorResponse,
        Request,
        Response,
    },
    protocol::Message,
};

fn get_crate_name() -> String {
    let crate_manifest_path = var("CRATE_MANIFEST_PATH")
        .expect("Cannot read CRATE_MANIFEST_PATH environment variable");

    // read Cargo.toml to read the crate name
    let mut cargo_toml_file = OpenOptions::new()
        .read(true)
        .open(&format!("{}", crate_manifest_path))
        .expect("Cannot open Cargo.toml");

    let mut buffer = String::new();
    cargo_toml_file
        .read_to_string(&mut buffer)
        .expect("Failed to read Cargo.toml");

    // Parse the Cargo.toml file content
    let cargo_toml: Value =
        buffer.parse::<Value>().expect("Failed to parse Cargo.toml");

    // Extract the package.name portion of the TOML file
    cargo_toml["package"]["name"].as_str().unwrap().to_owned()
}

fn get_wasm_path(
    crate_name: &str,
    crate_target_dir: &Path,
    profile: &str,
    target_triple: &str,
) -> PathBuf {
    let mut path = crate_target_dir.join(target_triple);
    path.push(profile);
    path.push(crate_name);
    path.set_extension("wasm");

    path
}

struct Writable {
    contents: Vec<u8>,
    line_len: usize,
}

impl Writable {
    fn new() -> Writable {
        let contents = "const wasm_b64 = \"".to_owned().into_bytes();
        let line_len = contents.len();

        Writable {
            contents,
            line_len,
        }
    }

    fn finish(mut self) -> String {
        self.contents.push(b'\"');
        self.contents.push(b';');
        return String::from_utf8(self.contents).unwrap();
    }
}

impl Write for Writable {
    fn write(
        &mut self,
        buf: &[u8],
    ) -> IoResult<usize> {
        let mut idx = 0;

        while idx < buf.len() {
            let chars_left_on_line = 79 - self.line_len;

            if chars_left_on_line == 0 {
                self.contents.push(b'\\');
                self.contents.push(b'\n');
                self.line_len = 0;
                continue;
            }

            let chars_left_on_buffer = buf.len() - idx;

            let chars_to_write = chars_left_on_line.min(chars_left_on_buffer);

            self.contents.write(&buf[idx .. idx + chars_to_write]);
            idx += chars_to_write;
            self.line_len += chars_to_write;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}

fn main() {
    let crate_target_dir = var("CRATE_TARGET_DIR")
        .map(|var| PathBuf::from(var))
        .expect("Cannot read CRATE_TARGET_DIR environment variable");
    let profile = var("CRATE_PROFILE")
        .expect("Cannot read CRATE_PROFILE environment variable");
    let target_triple = var("CRATE_TARGET_TRIPLE")
        .expect("Cannot read CRATE_TARGET_TRIPLE environment variable");

    let crate_name = get_crate_name();
    let wasm_path =
        get_wasm_path(&crate_name, &crate_target_dir, &profile, &target_triple);

    let wasm_output = crate_target_dir.join("wasm_output");

    // run wasm-bindgen
    Command::new("wasm-bindgen")
        .arg("--target")
        .arg("web")
        .arg(&wasm_path)
        .arg("--out-dir")
        .arg(&wasm_output)
        .output()
        .expect("Cannot run wasm-bindgen");

    let mut wasm_file = OpenOptions::new()
        .read(true)
        .open(wasm_output.join(format!("{}_bg.wasm", crate_name)))
        .map(|file| BufReader::new(file))
        .expect("Cannot read the wasm file.");

    // encode
    let mut writable = Writable::new();
    {
        let mut encoder = EncoderWriter::new(
            &mut writable,
            &base64::engine::general_purpose::STANDARD,
        );

        for byte in wasm_file.bytes() {
            encoder.write_all(&[byte.unwrap()]);
        }
        encoder.flush();
        encoder.finish().unwrap();
    }

    let mut resulting_javascript = writable.finish();
    resulting_javascript += "\n\n";

    // read the contents of the javascript file

    let mut wasm_file = OpenOptions::new()
        .read(true)
        .open(wasm_output.join(format!("{}.js", crate_name)))
        .map(|file| BufReader::new(file))
        .expect("Cannot open the bundler js file");

    let mut buffer = String::new();
    loop {
        buffer.clear();
        match wasm_file
            .read_line(&mut buffer)
            .expect("Cannot read the js file.")
        {
            0 => break,
            _ => {},
        }

        // stop reading from here. we'll have our own initializer.
        if buffer.contains("function initSync(module) {") {
            break;
        }

        resulting_javascript += &buffer;
    }

    resulting_javascript += include_str!("./addendum.js");

    // start the websocket server
    let server = std::net::TcpListener::bind("127.0.0.1:7953").unwrap();

    eprintln!("Listening on port 7953...");

    let stream = server.incoming().next().unwrap();
    let mut websocket =
        tungstenite::accept(stream.unwrap()).unwrap();

    eprintln!("Listener found. Uploading...");
    let message = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "pushFile",
        "params": {
            "filename": format!("{}.js", crate_name),
            "content": resulting_javascript,
            "server": "home",
        }
    })
    .to_string();

    websocket.write_message(Message::Text(message)).unwrap();

    websocket.close(None);
}

fn callback(
    req: &Request,
    mut response: Response,
) -> Result<Response, ErrorResponse> {
    // Let's add an additional header to our response to the client.
    let headers = response.headers_mut();

    Ok(response)
}
