use crate::browser;
use crate::button;
use crate::canvas::{load_image, load_image_data, Image, Renderer};
use crate::constants::{IMAGE_SOURCE, RUN_SIMULATION_BUTTON, RUN_SIMULATION_ID};
use crate::plot_machine::PlotMachine;
use crate::simulation_loop::Simulation;
use anyhow::{anyhow, Result};
use async_trait::async_trait;

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
                let button = browser::draw_ui(RUN_SIMULATION_BUTTON)
                    .and_then(|_unit| browser::find_html_element_by_id(RUN_SIMULATION_ID))
                    .map(button::add_click_handler)
                    .unwrap();

                //todo: this repetition is due to the fact that HtmlImageElement does not implement the Copy trait and cannot clone
                let machine = PlotMachine::new(
                    Image::new(
                        load_image(IMAGE_SOURCE).await?,
                        load_image(IMAGE_SOURCE).await?,
                        load_image_data(IMAGE_SOURCE).await?,
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
            machine.draw(renderer);
        };
    }
}
