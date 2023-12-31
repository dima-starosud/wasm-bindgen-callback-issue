use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "(arg: string) => number")]
    #[derive(Debug)]
    pub type TsCallback;

    #[wasm_bindgen(method, js_name = "call")]
    pub fn call(this: &TsCallback, arg: &str) -> i32;
}

#[wasm_bindgen]
pub fn rust_function(ts_callback: &TsCallback) -> i32 {
    let arg = format!("Hello from Rust! Here is what I've got {ts_callback:?}");
    ts_callback.call(&arg)
}
