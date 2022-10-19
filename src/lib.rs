#[macro_use]
mod browser;
mod simulation;
mod button;
mod image;
mod plot;

use simulation::Simulation;
use browser::spawn_local;
use plot::ImagePlot;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    log!("testing log! macro :). Hello world again!");
    spawn_local(async move {
        Simulation::start(ImagePlot::new())
            .await
            .expect("could not start Plot");
    });
    Ok(())
}
