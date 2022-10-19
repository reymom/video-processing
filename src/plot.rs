use crate::browser;
use crate::button;
use crate::image;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use image::{Image, Renderer};

#[async_trait(?Send)]
pub trait Plot {
    async fn initialize(&self) -> Result<Box<dyn Plot>>;
    fn update(&mut self);
    fn draw(&self, render: &Renderer);
}

pub struct ImagePlot {
    image: Option<Image>,
    simulation_handler: UnboundedReceiver<()>,
}

impl ImagePlot {
    pub fn new() -> Self {
        let (_, receiver) = unbounded();
        ImagePlot {
            image: None,
            simulation_handler: receiver,
        }
    }

    fn run_simulation_pressed(&mut self) -> bool {
        matches!(self.simulation_handler.try_next(), Ok(Some(())))
    }
}

#[async_trait(?Send)]
impl Plot for ImagePlot {
    async fn initialize(&self) -> Result<Box<dyn Plot>> {
        match self.image {
            None => {
                log!("none in initialize");
                let button =
                    browser::draw_ui("<button id='run_simulation'>Run simulation</button>")
                        .and_then(|_unit| browser::find_html_element_by_id("run_simulation"))
                        .map(button::add_click_handler)
                        .unwrap();
                Ok(Box::new(ImagePlot {
                    image: Some(Image::new(image::load_image("me.jpg").await?)),
                    simulation_handler: button,
                }))
            }
            Some(_) => Err(anyhow!("Error: Plot is already initialized!")),
        }
    }

    fn update(&mut self) {
        if let Some(image) = self.image.take() {
            self.image.replace(image.update());
        }
        assert!(self.image.is_some());
    }

    fn draw(&self, renderer: &Renderer) {
        if let Some(image) = &self.image {
            log!("drawing");
            image.draw(renderer);
        };
    }
}
