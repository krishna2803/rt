pub mod hittables;
pub mod image;
pub mod maths;
pub mod objects;
pub mod camera;

use hittables::*;
use maths::*;
use objects::*;

fn main() {
    let aspect_ratio = 16.0 / 10.0;
    let image_width: u32 = 256;

    // // World
    let mut world: hittable_list = hittable_list::new();
    world.add(hittable_obj::sphere(sphere::new(
        point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(hittable_obj::sphere(sphere::new(
        point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let cam = camera::camera::new(aspect_ratio, image_width);
    cam.render(&world);
}
