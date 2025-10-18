#![allow(dead_code)]

use std::fmt;

#[derive(Clone, Copy)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Pixel>
}

// Default pixel
impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Pixel { r, g, b, a }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel { r: 0, g: 0, b: 0, a: 0xFF }
    }
}

// Pixel to String
impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

pub fn set_pixel(mut p: Pixel, r: u8, g: u8, b: u8) {
    p.r = r;
    p.g = g;
    p.b = b;
    p.a = 0xFF;
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Framebuffer {
            width,
            height,
            pixels: vec![Pixel::default(); size],
        }
    }
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        let index = (y * self.width + x) as usize;
        self.pixels[index] = pixel;
    }
    pub fn get_pixel(&mut self, x: u32, y: u32) -> &Pixel {
        let index = (y * self.width + x) as usize;
        return &self.pixels[index];
    }
    pub fn iter(&self) -> impl Iterator<Item = &Pixel> {
        self.pixels.iter()
    }
    pub fn fill_with<F>(&mut self, mut f: F)
    where
        F: FnMut(u32, u32) -> Pixel,
    {
        for j in 0..self.height {
            for i in 0..self.width {
                self.set_pixel(i, j, f(i, j));
            }
        }
    }
}
