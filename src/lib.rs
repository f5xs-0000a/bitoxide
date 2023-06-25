use wasm_bindgen::{
    prelude::*,
    JsValue,
};

// thank you github.com/paulcdejean
#[wasm_bindgen]
extern "C" {
    // Continue adding more imported structs from Bitburner and their associated
    // methods in here.
    //
    // For object attributes, skip to after this `extern` block.

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

pub fn get_attribute<T>(
    object: &JsValue,
    field_name: &str,
    mapper: impl Fn(&JsValue) -> Option<T>,
) -> Result<Option<T>, JsValue> {
    js_sys::Reflect::get(object, &JsValue::from_str(field_name))
        .map(|x| mapper(&x))
}

#[wasm_bindgen]
pub fn main_rs(ns: &NS) {
    let mut buffer = "Hello, world! I said".to_owned();
    let args = get_attribute(ns, "args", |a| Some(js_sys::Array::from(a)))
        .unwrap()
        .unwrap();
    let args_iter = args.iter().map(|a| a.as_string().unwrap());

    for arg in args_iter {
        buffer += &arg;
    }

    ns.tprint(&buffer);
}
