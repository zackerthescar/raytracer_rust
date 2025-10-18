#![allow(dead_code)]

use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf; // For OS-agnostic filenames

use crate::vec3::Vec3;
use crate::color::Color;
use crate::color::Phong;

pub struct Scene {
    pub eye: Vec3,
    pub viewdir: Vec3,
    pub updir: Vec3,
    pub hfov: u32,
    pub height: u32,
    pub width: u32,
    pub bkgcolor: Color,
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
}

pub struct Sphere {
    pub pos: Vec3,
    pub rad: f32,
    pub color: Phong,
}

pub enum LightType {
    Vector,
    Point,
}

pub struct Light {
    pub pos: Vec3,
    pub intensity: f32,
    pub light_type: LightType,
}

impl Scene {
    pub fn from_file(name: &str) -> Result<Self, String> {
        // Set all scene properties to None
        let mut eye = None;
        let mut viewdir = None;
        let mut updir = None;
        let mut hfov = None;
        let mut height = None;
        let mut width = None;
        let mut bkgcolor = None;
        let mut mtlcolor: Option<Phong> = None;
        let mut spheres: Vec<Sphere> = vec![];
        let mut lights: Vec<Light> = vec![];

        let mut path = PathBuf::from(name);
        path.set_extension("obj");

        // Attempt to open the file
        let file = match File::open(&path) {
            Err(e) => panic!("Could not open file {}: {}", path.display(), e),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Err(e) => panic!("Error parsing line: {}", e),
                Ok(line) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    match parts.as_slice() {
                        ["eye", x, y, z] => {
                            match (x.parse(), y.parse(), z.parse()) {
                                (Ok(x), Ok(y), Ok(z)) => eye = Some(Vec3::new(x,y,z)),
                                _ => return Err("Bad eye".to_string()),
                            }
                        }
                        ["viewdir", x, y, z] => {
                            match (x.parse(), y.parse(), z.parse()) {
                                (Ok(x), Ok(y), Ok(z)) => viewdir = Some(Vec3::new(x,y,z)),
                                _ => return Err("Bad viewdir".to_string()),
                            }
                        }
                        ["updir", x, y, z] => {
                            match (x.parse(), y.parse(), z.parse()) {
                                (Ok(x), Ok(y), Ok(z)) => updir = Some(Vec3::new(x,y,z)),
                                _ => return Err("Bad updir".to_string()),
                            }
                        }
                        ["hfov", val] => {
                            match val.parse() {
                                Ok(h) => hfov = Some(h),
                                Err(_) => return Err("Bad hfov".to_string()),
                            }
                        }
                        ["imsize", w, h] => {
                            match (w.parse(), h.parse()) {
                                (Ok(w), Ok(h)) => {
                                    width = Some(w);
                                    height = Some(h);
                                }
                                _ => return Err("Bad imsize".to_string()),
                            }
                        }
                        ["bkgcolor", r, g, b] => {
                            match (r.parse::<f32>(), g.parse::<f32>(), b.parse::<f32>()) {
                                (Ok(r), Ok(g), Ok(b)) => {
                                    bkgcolor = Some(Color::new(r, g, b));
                                },
                                _ => return Err("Bad bgcolor".to_string()),
                            }
                        }
                        // Here we move into objects
                        ["mtlcolor", dr, dg, db, sr, sg, sb, ka, kd, ks, ex] => {
                            match (
                                dr.parse::<f32>(), dg.parse::<f32>(), db.parse::<f32>(),
                                sr.parse::<f32>(), sg.parse::<f32>(), sb.parse::<f32>(),
                                ka.parse::<f32>(), kd.parse::<f32>(), ks.parse::<f32>(),
                                ex.parse::<f32>()
                            ) {
                                (Ok(dr), Ok(dg), Ok(db),Ok(sr), Ok(sg), Ok(sb),
                            Ok(ka), Ok(kd), Ok(ks), Ok(ex)) => {
                                    mtlcolor = Some(Phong::new(dr, dg, db, sr, sg, sb, ka, kd, ks,ex));
                                },
                                _ => return Err("Bad mtlcolor".to_string()),
                            }
                        }
                        ["sphere", x, y, z, r] => {
                            match (x.parse::<f32>(), y.parse::<f32>(), z.parse::<f32>(), r.parse::<f32>()) {
                                (Ok(x), Ok(y), Ok(z), Ok(r)) => {
                                    match mtlcolor {
                                        Some(phong) => {
                                            spheres.push(Sphere::new(x, y, z, r, phong));
                                        },
                                        None => return Err("No color to go with sphere".to_string())
                                    }
                                }
                                _ => return Err("Bad sphere".to_string())
                            }
                        }
                        ["light", x, y, z, t, i] => {
                            match (x.parse::<f32>(), y.parse::<f32>(), z.parse::<f32>(), t.parse(), i.parse()) {
                                (Ok(x), Ok(y), Ok(z), Ok(t), Ok(i)) => {
                                    match t {
                                        0 => lights.push(Light::new(x, y, z,LightType::Vector, i)),
                                        1 => lights.push(Light::new(x, y, z, LightType::Point, i)),
                                        _ => return Err("Bad light type".to_string())
                                    }
                                },
                                _ => return Err("Bad light".to_string())
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }
        Ok(Scene {
            eye: eye.ok_or("Missing 'eye' parameter")?,
            viewdir: viewdir.ok_or("Missing 'viewdir' parameter")?,
            updir: updir.ok_or("Missing 'updir' parameter")?,
            hfov: hfov.ok_or("Missing 'hfov' parameter")?,
            height: height.ok_or("Missing 'height' parameter")?,
            width: width.ok_or("Missing 'width' parameter")?,
            bkgcolor: bkgcolor.ok_or("Missing 'bkgcolor' parameter")?,
            spheres: spheres,
            lights: lights,
        })
    }
    pub fn print_scene_params(&self) {
        println!{"eye {}", self.eye};
        println!{"viewdir {}", self.viewdir};
        println!{"updir {}", self.updir};
        println!{"hfov {}", self.hfov};
        println!{"imsize {} {}", self.width, self.height};
        println!{"bkgcolor {}", self.bkgcolor};
    }
    pub fn print_objects(&self) {
        for sphere in self.spheres.iter() {
            println!("{}", sphere);
        }
    }
}

impl Sphere {
    fn new(x: f32, y: f32, z: f32, r: f32, p: Phong) -> Sphere {
        Sphere {
            pos: Vec3::new(x, y, z),
            rad: r,
            color: p,
        }
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sphere {} {}", self.pos, self.rad)
    }
}

impl Light {
    fn new(x: f32, y: f32, z: f32, lt: LightType, i: f32) -> Light {
        Light {
            pos: Vec3::new(x,y,z),
            light_type: lt,
            intensity: i
        }
    }
}
