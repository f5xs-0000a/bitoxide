use js_sys::Array;
use wasm_bindgen::prelude::*;

// thank you github.com/paulcdejean
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn alert(msg: &str);

    pub type NS;

    #[wasm_bindgen(method)]
    fn tprint(
        this: &NS,
        print: &str,
    );

    #[wasm_bindgen(method)]
    fn scan(
        this: &NS,
        scan: Option<&str>,
    ) -> Vec<JsValue>;

    #[wasm_bindgen(catch, method)]
    fn nuke(
        this: &NS,
        host: &str,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method)]
    fn brutessh(
        this: &NS,
        hostname: &str,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method)]
    fn ftpcrack(
        this: &NS,
        hostname: &str,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method)]
    fn relaysmtp(
        this: &NS,
        hostname: &str,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method)]
    fn httpworm(
        this: &NS,
        hostname: &str,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method)]
    fn sqlinject(
        this: &NS,
        hostname: &str,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method)]
    fn getServer(
        this: &NS,
        host: Option<&str>,
    ) -> Server;

    pub type Server;
}

#[wasm_bindgen]
pub fn main_rs(
    ns: &NS,
    args: Array,
) {
    let mut buffer = "Hello, world! I said".to_owned();
    for arg in args.iter().map(|a| a.as_string().unwrap()) {
        buffer += &arg;
    }
    
    ns.tprint(&buffer);
}
