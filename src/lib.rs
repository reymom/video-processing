#[macro_use]
mod browser;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    log!("testing log! macro :). Hello world again!");

    Ok(())
}
