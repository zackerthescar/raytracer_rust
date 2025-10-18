#![allow(dead_code)]

use std::fmt;

#[derive(Clone, Copy, Default)]
pub struct Vec3{ 
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
    // Vector magnitude
    pub fn magnitude(&self) -> f32 {
        let sum = self.x * self.x + self.y * self.y + self.z * self.z;
        sum.sqrt()
    }
    // Vector dot product
    pub fn dot(&self, b: Vec3) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    // Vector cross product, "self" is first argument
    pub fn cross(&self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
        }
    }
    // Vector normalization
    pub fn normalize(&self) -> Vec3 {
        let len = self.magnitude();
        if len > 0.0 {
            Vec3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }
    // Vector in-place negation
    pub fn negate(&self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
    // Vector scalar multiplication
    pub fn multiply_scalar(&self, n: f32) -> Vec3 {
        Vec3 {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n,
        }
    }
    // Vector scalar division
    pub fn divide_scalar(&self, n: f32) -> Vec3 {
        if n != 0.0 {
            Vec3 {
                x: self.x / n,
                y: self.y / n,
                z: self.z / n,
            }
        } else {
            eprintln!("DIVIDE BY ZERO ENCOUNTERED, CHECK ARITHMETIC");
            Vec3 {
                x: self.x,
                y: self.y,
                z: self.z,
            }
        }
    }
    // Vector-vector addition
    pub fn add_vector(&self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }
    // Vector-vector subtraction
    pub fn subtract_vector (&self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} {:.2} {:.2}", self.x, self.y, self.z)
    }
}
