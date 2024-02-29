pub mod camera;
pub mod hittables;
pub mod image;
pub mod materials;
pub mod maths;
pub mod objects;

use hittables::*;
use materials::*;
use maths::*;
use objects::*;

fn main() {
    let aspect_ratio = 16.0 / 10.0;
    let image_width: u32 = 512;

    // // World
    let mut world: hittable_list = hittable_list::new();

    let ground = material::lambertian(lambertian::new(color::new(0.8, 0.8, 0.0)));
    let center = material::lambertian(lambertian::new(color::new(0.7, 0.3, 0.3)));
    let left = material::metal(metal::new(color::new(0.8, 0.8, 0.8)));
    let right = material::metal(metal::new(color::new(0.8, 0.6, 0.2)));

    world.add(hittable_obj::sphere(sphere::new(
        point3::new(0.0, -100.5, 1.0),
        100.0,
        ground,
    )));
    world.add(hittable_obj::sphere(sphere::new(
        point3::new(0.0, 0.0, -1.0),
        0.5,
        center,
    )));
    world.add(hittable_obj::sphere(sphere::new(
        point3::new(-1.0, 0.0, -1.0),
        0.5,
        left,
    )));
    world.add(hittable_obj::sphere(sphere::new(
        point3::new(1.0, 0.0, -1.0),
        0.5,
        right,
    )));

    let cam = camera::camera::new(aspect_ratio, image_width);
    cam.render(&world);
}
