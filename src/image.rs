use crate::browser;

use anyhow::{anyhow, Result};
use futures::channel::oneshot::channel;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub struct Image {
    element: HtmlImageElement,
    position: Point,
}

impl Image {
    pub fn new(element: HtmlImageElement) -> Self {
        Self {
            element,
            position: Point { x: 0, y: 0 },
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        renderer.draw_image(&self.element, &self.position);
    }

    pub fn update(self) -> Self {
        self
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

pub struct Renderer {
    pub context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn draw_image(&self, image: &HtmlImageElement, position: &Point) {
        self.context
            .draw_image_with_html_image_element(image, position.x.into(), position.y.into())
            .expect("Drawing is throwing exceptions! Unrecoverable error.");
    }
}
