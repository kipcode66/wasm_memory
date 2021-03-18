extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start, catch)]
pub async fn main() -> Result<(), JsValue> {
    // Something...
    Ok(())
}