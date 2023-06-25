async function __wbg_init() {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    let wasm_binary = Uint8Array.from(atob(wasm_b64), c => c.charCodeAt(0));

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(wasm_binary, imports);

    return __wbg_finalize_init(instance, module);
}

export async function main(ns) {
    await __wbg_init();
    await main_rs(ns);
}
