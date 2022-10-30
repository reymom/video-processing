use crate::browser;
use crate::image::RawImage;

use anyhow::{anyhow, Result};
use futures::channel::oneshot::channel;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement, ImageData};

pub struct Renderer {
    pub context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn draw_image(&self, image: &HtmlImageElement, position: &Point) {
        self.context
            .draw_image_with_html_image_element(image, position.x.into(), position.y.into())
            .expect("Drawing is throwing exceptions! Unrecoverable error.");
    }

    pub fn put_image(&self, image_data: &ImageData, position: &Point) {
        self.context
            .put_image_data(image_data, position.x.into(), position.y.into())
            .expect("Put Image is throwing exceptions! Unrecoverable error.");
    }
}

pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub struct Image {
    element: HtmlImageElement,
    image: RawImage,
    position: Point,
}

impl Image {
    pub fn new(element: HtmlImageElement) -> Self {
        Self {
            element,
            image: RawImage::new(),
            position: Point { x: 0, y: 0 },
        }
    }

    pub fn load_image(mut self, renderer: &Renderer) -> Self {
        let imgdata = load_image_data(renderer).expect("cannot load raw image data!");
        self.image = imgdata.into();
        self
    }

    pub fn refresh(mut self) -> Self {
        self.image = RawImage::new();
        self
    }

    pub fn draw(&self, renderer: &Renderer) {
        renderer.draw_image(&self.element, &self.position);
    }

    pub fn put_image(&self, renderer: &Renderer) {
        let data = self
            .image
            .to_image_data()
            .expect("unrecoverable error: cannot get ImageData");

        renderer.put_image(&data, &self.position);
    }

    pub fn run_simulation_step(&mut self) {
        self.image.solarize();
        self.image.grayscale();
    }
}

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = browser::new_image()?;
    let (complete_tx, complete_rx) = channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);
    let success_callback = browser::closure_once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
            if let Err(err) = success_tx.send(Ok(())) {
                error!("Error sending ok result in success_callback {:#?}", err);
            };
        }
    });
    let error_callback: Closure<dyn FnMut(JsValue)> = browser::closure_once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
            if let Err(err) = error_tx.send(Err(anyhow!("Error Loading Image {:#?}", err))) {
                error!("Error sending ok result in error_callback {:#?}", err);
            };
        }
    });
    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(source);
    complete_rx.await??;
    Ok(image)
}

pub fn load_image_data(renderer: &Renderer) -> Result<ImageData> {
    let canvas = browser::canvas()?;
    renderer
        .context
        .get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64)
        .map_err(|err| anyhow!("Could not get ImageData {:#?}", err))
}
