#[macro_use]
mod browser;
mod button;
mod canvas;
mod constants;
mod plot;
mod plot_machine;
mod plot_states;
mod simulation_loop;

use browser::spawn_local;
use plot::SimulationPlot;
use simulation_loop::SimulationLoop;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    spawn_local(async move {
        SimulationLoop::start(SimulationPlot::new())
            .await
            .expect("could not start SimulationLoop");
    });
    Ok(())
}
