use crate::browser;

use anyhow::{anyhow, Result};
use futures::channel::oneshot::channel;
use image::io::Reader as ImageReader;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{Clamped, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement, ImageData};

pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub struct Image {
    element: HtmlImageElement,
    initial_element: HtmlImageElement,
    data: ImageData,
    position: Point,
}

impl Image {
    pub fn new(
        element: HtmlImageElement,
        initial_element: HtmlImageElement,
        data: ImageData,
    ) -> Self {
        log!("Image::new!");
        Self {
            element,
            initial_element,
            data,
            position: Point { x: 0, y: 0 },
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        renderer.draw_image(&self.element, &self.position);
        //renderer.put_image(&self.data, &self.position);
    }

    pub fn refresh(&mut self) {
        self.element = self.initial_element.clone();
    }

    pub fn run_simulation_step(&mut self) {
        // todo: change pixels
        // self.element =
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
    log!("image complete");
    Ok(image)
}

pub async fn load_image_data(source: &str) -> Result<ImageData> {
    log!("load_image_data");
    //let img = image::open(source)?;
    //let img = image::load(BufReader::new(File::open(source)?), ImageFormat::Jpeg)?;
    //let img = Reader::open(source)?.decode()?;
    //log!("image::open done");
    //let clamped_buf: Clamped<&[u8]> = Clamped(img.as_bytes());
    //log!("load_image_data clamped done");

    //log!("image_data before");
    //ImageData::new_with_u8_clamped_array_and_sh(
    //    Clamped(&mut vec![255; (400 * 300 * 4) as usize]),
    //    400,
    //    300,
    //)
    //.map_err(|err| anyhow!("Could not cast into ImageData {:#?}", err))
    //let clamped_array = Uint8ClampedArray::new_with_length(5000);
    //let clamped_buf = Clamped(&clamped_array);
    //ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, img.width(), img.height())
    //    .map_err(|err| anyhow!("Could not cast into ImageData {:#?}", err))

    let img = ImageReader::open("freak.png")?.decode()?;
    let clamped_buf: Clamped<&[u8]> = Clamped(img.as_bytes());
    ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, img.width(), img.height())
        .map_err(|err| anyhow!("Could not cast into ImageData {:#?}", err))
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

    pub fn put_image(&self, image_data: &ImageData, position: &Point) {
        self.context
            .put_image_data(image_data, position.x.into(), position.y.into())
            .expect("Put Image is throwing exceptions! Unrecoverable error.");
    }
}
