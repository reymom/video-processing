use crate::browser;
use crate::button;
use crate::constants::{image_source, run_simulation_button, run_simulation_id};
use crate::image;
use crate::plot_state::PlotMachine;
use crate::simulation_loop::Simulation;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use image::{Image, Renderer};

pub struct SimulationPlot {
    machine: Option<PlotMachine>,
}

impl SimulationPlot {
    pub fn new() -> Self {
        SimulationPlot { machine: None }
    }
}

#[async_trait(?Send)]
impl Simulation for SimulationPlot {
    async fn initialize(&self) -> Result<Box<dyn Simulation>> {
        match self.machine {
            None => {
                log!("none in initialize");
                let button = browser::draw_ui(run_simulation_button)
                    .and_then(|_unit| browser::find_html_element_by_id(run_simulation_id))
                    .map(button::add_click_handler)
                    .unwrap();

                //todo: this repetition is due to the fact that HtmlImageElement does not implement the Copy trait and cannot clone
                let machine = PlotMachine::new(
                    Image::new(
                        image::load_image(image_source).await?,
                        image::load_image(image_source).await?,
                    ),
                    button,
                );

                Ok(Box::new(SimulationPlot {
                    machine: Some(machine),
                }))
            }
            Some(_) => Err(anyhow!("Error: Plot is already initialized!")),
        }
    }

    fn update(&mut self) {
        if let Some(machine) = self.machine.take() {
            self.machine.replace(machine.update());
        }
        assert!(self.machine.is_some());
    }

    fn draw(&self, renderer: &Renderer) {
        if let Some(machine) = &self.machine {
            log!("drawing");
            machine.draw(renderer);
        };
    }
}
