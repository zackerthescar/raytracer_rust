use std::env;

mod pixel;
mod ppm;
mod vec3;
mod scene;
mod ray;
mod color;

use crate::vec3::Vec3;
use crate::pixel::{Framebuffer};
use crate::scene::Scene;
use crate::ray::cast_ray;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <obj file> <output file>", args[0]);
        std::process::exit(1);
    }
    let scenefile = &args[1];
    let outfile = &args[2];
    let scene = match Scene::from_file(scenefile) {
        Ok(scene) => {
            // scene.print_scene_params();
            // scene.print_objects();
            scene
        },
        Err(e) => {
            eprintln!("Error loading scene: {}", e);
            std::process::exit(1);
        }
    };
    let mut framebuffer = Framebuffer::new(scene.width, scene.height);
    let _w: Vec3 = scene.viewdir.normalize().negate();
    let u: Vec3 = scene.viewdir.cross(scene.updir).normalize();
    let v: Vec3 = u.cross(scene.viewdir.normalize()).normalize();
    let d: f32 = 10.0;
    let wi: f32 = 2.0 * d * ((scene.hfov / 2) as f32).to_radians().tan();
    let he: f32 = wi * (scene.height as f32 / scene.width as f32); 
    let viewpoint: Vec3 = scene.viewdir.normalize().multiply_scalar(d).add_vector(scene.eye);
    let ul: Vec3 = viewpoint.subtract_vector(u.multiply_scalar(wi / 2.0)).add_vector(v.multiply_scalar(he / 2.0));
    let ur: Vec3 = viewpoint.add_vector(u.multiply_scalar(wi / 2.0)).add_vector(v.multiply_scalar(he / 2.0));
    let ll: Vec3 = viewpoint.subtract_vector(u.multiply_scalar(wi / 2.0)).subtract_vector(v.multiply_scalar(he / 2.0));
    let _lr: Vec3 = viewpoint.add_vector(u.multiply_scalar(wi / 2.0)).subtract_vector(v.multiply_scalar(he / 2.0));
    let dh: Vec3 = ur.subtract_vector(ul).divide_scalar((scene.width - 1) as f32);
    let dv: Vec3 = ll.subtract_vector(ul).divide_scalar((scene.height - 1) as f32);
    framebuffer.fill_with(|i, j| {
        let delta = dh.multiply_scalar(i as f32).add_vector(dv.multiply_scalar(j as f32));
        let pixel_direction = ul.add_vector(delta).subtract_vector(scene.eye).normalize();
        cast_ray(&scene, pixel_direction)
    });
    let f = ppm::write_ppm_header(ppm::new_ppm(outfile), scene.width, scene.height);
    match ppm::write_buffer_data(f, &framebuffer) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error writing ppm: {}", e);
            std::process::exit(1);
        }
    };
}
