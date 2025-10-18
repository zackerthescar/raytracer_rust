use crate::scene::LightType;
use crate::vec3::Vec3;
use crate::pixel::Pixel;
use crate::scene::Scene;
use crate::scene::Sphere;
use crate::color::Color;

fn ray_sphere_intersection(sphere: &Sphere, origin: Vec3, direction: Vec3) -> Option<f32> {
    let a: f32 = direction.dot(direction);
    let b: f32 = 2.0 * direction.dot(origin.subtract_vector(sphere.pos));
    let c: f32 = origin.subtract_vector(sphere.pos).dot(origin.subtract_vector(sphere.pos)) - (sphere.rad * sphere.rad);
    let discrim = (b * b) - 4.0 * a * c;
    if discrim < 0.0 {
        None
    } else {
        let t1 = (-b - discrim.sqrt()) / (2.0 * a);
        let t2 = (-b + discrim.sqrt()) / (2.0 * a);
        if t1 > 0.0 {
            Some(t1)
        } else if t2 > 0.0 {
            Some(t2)
        } else {
            None
        }
    }
}

fn ray_phong_shading(scene: &Scene, sphere: &Sphere, origin: Vec3, direction: Vec3, distance: f32) -> Color {
    let ambient: Color = sphere.color.ambient_term();
    let intersect: Vec3 = origin.add_vector(direction.multiply_scalar(distance));
    let normal: Vec3 = intersect.subtract_vector(sphere.pos).divide_scalar(sphere.rad);
    let mut acc: Color = Color::new(0.0,0.0,0.0);
    for light in &scene.lights {
        for other_sphere in &scene.spheres {
            if std::ptr::eq(sphere, other_sphere) { continue; }
            match ray_sphere_intersection(other_sphere, intersect.normalize(), light.pos) {
                Some(_) => continue,
                None => {
                    let l: Vec3 = match light.light_type {
                        LightType::Vector => light.pos.normalize().negate(),
                        LightType::Point => light.pos.subtract_vector(intersect).normalize(),
                    };
                    let v: Vec3 = origin.subtract_vector(intersect).normalize();
                    let h: Vec3 = l.add_vector(v).normalize();
                    let nl: f32 = if normal.dot(l) >= 0.0 { normal.dot(l) } else { 0.0 };
                    let nh: f32 = if normal.dot(h) >= 0.0 { normal.dot(h) } else { 0.0 };
                    let diffuse: Color = sphere.color.diffuse_term(nl);
                    let specular: Color = sphere.color.specular_term(nh);
                    let total: Color = diffuse.add(specular).multiply_scalar(light.intensity);
                    acc = acc.add(total);
                }
            }
        }
    }
    ambient.add(acc)
}

pub fn cast_ray(scene: &Scene, direction: Vec3) -> Pixel {
    let mut closest_t: Option<f32> = None;
    let mut closest_sphere: Option<&Sphere> = None;

    for sphere in &scene.spheres {
        if let Some(t) = ray_sphere_intersection(sphere, scene.eye, direction) {
            match closest_t {
                None => {
                    closest_t = Some(t);
                    closest_sphere = Some(sphere);
                }
                Some(ct) => {
                    if t < ct {
                        closest_t = Some(t);
                        closest_sphere = Some(sphere);
                    }
                }
            }
        }
    }

    match (closest_t, closest_sphere) {
        (Some(t), Some(sphere)) => ray_phong_shading(scene, sphere, scene.eye, direction, t).to_pixel(),
        (None, _) => scene.bkgcolor.to_pixel(),
        _ => scene.bkgcolor.to_pixel(),
    }
}
