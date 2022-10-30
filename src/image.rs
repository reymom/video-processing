use anyhow::{anyhow, Result};
use wasm_bindgen::Clamped;
use web_sys::ImageData;

#[derive(Clone)]
pub struct RawImage {
    raw_pixels: Vec<u8>,
    width: u32,
    height: u32,
}

impl RawImage {
    pub fn new() -> Self {
        RawImage {
            raw_pixels: vec![],
            width: 0,
            height: 0,
        }
    }

    pub fn to_image_data(&self) -> Result<ImageData> {
        let mut raw_pixels = &self.raw_pixels;
        let width = self.width;
        let height = self.height;
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut raw_pixels), width, height)
            .map_err(|err| anyhow!("could not create new ImageData from raw image {:?}", err))
    }

    pub fn solarize(&mut self) {
        let end = self.raw_pixels.len();

        for i in (0..end).step_by(4) {
            let r_val = self.raw_pixels[i];

            if 200 - r_val as i32 > 0 {
                self.raw_pixels[i] = 200 - r_val;
            }
        }
    }

    pub fn grayscale(&mut self) {
        let end = self.raw_pixels.len();

        for i in (0..end).step_by(4) {
            let r_val = self.raw_pixels[i] as u32;
            let g_val = self.raw_pixels[i + 1] as u32;
            let b_val = self.raw_pixels[i + 2] as u32;
            let mut avg: u32 = (r_val + g_val + b_val) / 3;
            if avg >= 255 {
                avg = 255
            }

            self.raw_pixels[i] = avg as u8;
            self.raw_pixels[i + 1] = avg as u8;
            self.raw_pixels[i + 2] = avg as u8;
        }
    }

    pub fn alter_red_channel(&mut self, amt: i16) {
        let end = self.raw_pixels.len();

        for i in (0..end).step_by(4) {
            let inc_val: i16 = self.raw_pixels[i] as i16 + amt;
            self.raw_pixels[i] = inc_val.clamp(0, 255) as u8;
        }
    }
}

impl From<ImageData> for RawImage {
    fn from(imgdata: ImageData) -> Self {
        let width = imgdata.width();
        let height = imgdata.height();
        let raw_pixels = imgdata.data().to_vec();
        RawImage {
            raw_pixels,
            width,
            height,
        }
    }
}
