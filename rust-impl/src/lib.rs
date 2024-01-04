use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const X_PEND_CONTENT: &'static str = r#"
export interface TsCallback {
    call(arg: string): number;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "TsCallback")]
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
