use crate::image;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use image::{Image, Renderer};

#[async_trait(?Send)]
pub trait Plot {
    async fn initialize(&self) -> Result<Box<dyn Plot>>;
    fn draw(&self, render: &Renderer);
}

pub struct ImagePlot {
    image: Option<Image>,
}

impl ImagePlot {
    pub fn new() -> Self {
        ImagePlot { image: None }
    }
}

#[async_trait(?Send)]
impl Plot for ImagePlot {
    async fn initialize(&self) -> Result<Box<dyn Plot>> {
        match self.image {
            None => {
                log!("none in initialize");
                Ok(Box::new(ImagePlot {
                    image: Some(Image::new(image::load_image("me.jpg").await?)),
                }))
            }
            Some(_) => Err(anyhow!("Error: Plot is already initialized!")),
        }
    }

    fn draw(&self, renderer: &Renderer) {
        if let Some(image) = &self.image {
            log!("drawing");
            image.draw(renderer);
        };
    }
}
