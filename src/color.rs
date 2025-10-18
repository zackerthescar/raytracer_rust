use crate::pixel::Pixel;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

#[derive(Clone, Copy, Default)]
pub struct Phong {
    diffuse: Color,
    specular: Color,
    ka: f32,
    kd: f32,
    ks: f32,
    specular_exponent: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b }
    }
    pub fn to_pixel(&self) -> Pixel {
        let r_u8 = (self.r * 255.0).clamp(0.0, 255.0) as u8;
        let g_u8 = (self.g * 255.0).clamp(0.0, 255.0) as u8;
        let b_u8 = (self.b * 255.0).clamp(0.0, 255.0) as u8;
        Pixel::new(r_u8, g_u8, b_u8, 0xFF)
    }
    pub fn add(&self, b: Color) -> Color {
        Color {
            r: self.r + b.r,
            g: self.g + b.g,
            b: self.b + b.b,
        }
    }
    pub fn multiply_scalar(&self, n: f32) -> Color {
        Color {
            r: self.r * n,
            g: self.g * n,
            b: self.b * n,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color { r: 0.0, g: 0.0, b: 0.0 }
    }
}

impl Phong {
    pub fn new(dr: f32, dg: f32, db: f32, sr: f32, sg: f32, sb: f32, ka:f32, kd: f32, ks: f32, ex: f32) -> Phong {
        Phong {
            diffuse: Color::new(dr, dg, db),
            specular: Color::new(sr, sg, sb),
            ka, kd, ks, specular_exponent: ex
        }
    }
    pub fn ambient_term(&self) -> Color {
        Color {
            r: self.ka * self.diffuse.r,
            g: self.ka * self.diffuse.g,
            b: self.ka * self.diffuse.b
        }
    }
    pub fn diffuse_term(&self, nl: f32) -> Color {
        Color {
            r: self.kd * self.diffuse.r * nl,
            g: self.kd * self.diffuse.g * nl,
            b: self.kd * self.diffuse.b * nl
        }
    }
    pub fn specular_term(&self, nh: f32) -> Color {
        Color {
            r: self.ks * self.specular.r * nh.powf(self.specular_exponent),
            g: self.ks * self.specular.g * nh.powf(self.specular_exponent),
            b: self.ks * self.specular.b * nh.powf(self.specular_exponent),
        }
    }
}

// Color to String
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} {:.2} {:.2}", self.r, self.g, self.b)
    }
}
