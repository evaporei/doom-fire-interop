use crate::random::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct PixelBoard {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

impl PixelBoard {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn create_fire_source(&mut self) {
        let mut column = 0;
        while column <= self.width {
            let overflow_pixel_index = self.pixels.len();
            let pixel_index = (overflow_pixel_index - self.width - 1) + column;

            self.pixels[pixel_index] = 36;

            column += 1;
        }
    }

    pub fn calculate_fire_propagation(&mut self, render: Box<dyn Fn(&[u8])>) {
        for column in 0..self.width {
            for row in 0..self.height {
                let pixel_index = column + (self.width * row);

                self.update_fire_intensity_per_pixel(pixel_index);
            }
        }

        render(&self.pixels);
    }

    fn update_fire_intensity_per_pixel(&mut self, current_pixel_index: usize) {
        let below_pixel_index = current_pixel_index + self.width;

        if below_pixel_index >= self.width * self.height {
            return;
        }

        let decay = (random() * 3.0).floor() as u8; // 0, 1, 2

        let below_pixel_fire_intensity = self.pixels[below_pixel_index];

        let pixel_decay_index = Self::calculate_pixel_decay_index(current_pixel_index, decay);

        let new_intensity = Self::calculate_new_intensity(below_pixel_fire_intensity, decay);

        self.pixels[pixel_decay_index as usize] = new_intensity;
    }

    fn calculate_pixel_decay_index(current_pixel: usize, decay: u8) -> usize {
        let new_intensity = current_pixel as isize - decay as isize;

        if new_intensity >= 0 {
            return new_intensity as usize;
        }

        return 0;
    }

    fn calculate_new_intensity(below_intensity: u8, decay: u8) -> u8 {
        let new_intensity = below_intensity as i8 - decay as i8;

        if new_intensity >= 0 {
            return new_intensity as u8;
        }

        return 0;
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_pixels_len(&self) -> usize {
        self.pixels.len()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_pixels_ptr(&self) -> *const usize {
        self.pixels.as_ptr() as *const usize
    }

    #[cfg(target_os = "android")]
    pub fn get_pixels(&self) -> &[u8] {
        &self.pixels
    }
}
