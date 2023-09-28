use wasm_bindgen::{prelude::wasm_bindgen, JsError as Error, JsValue};

#[fehler::throws]
#[wasm_bindgen]
pub fn parse_hex(hex: &str) -> JsValue {
    let parse_hex = cbor_diag::parse_hex(hex)?;
    let diag_result = parse_hex.to_diag_pretty();
    return JsValue::from(&diag_result);
}
