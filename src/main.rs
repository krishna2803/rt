pub mod hittables;
pub mod image;
pub mod maths;
pub mod objects;

use hittables::*;
use image::*;
use maths::*;
use objects::*;

fn write_color(c: &color, data: &mut Vec<u8>) {
    unsafe {
        data.push((c.x() * 255.999).to_int_unchecked());
        data.push((c.y() * 255.999).to_int_unchecked());
        data.push((c.z() * 255.999).to_int_unchecked());
        data.push(255u8); // because of the alpha channel
    }
}

fn ray_color(r: &ray, world: &hittable_list) -> color {
    let mut rec = hit_record::null();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        (rec.normal + color::from_scalar(1.0)) * 0.5
    } else {
        let unit_direction = r.direction().normalized();
        let a = 0.5 * (unit_direction.y() + 1.0);
        let start_color = color::new(0.5, 0.7, 1.0);
        let end_color = color::new(1.0, 1.0, 1.0);
        start_color * a + end_color * (1.0 - a)
    }
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 10.0;
    let image_width: u32 = 1000;
    let image_height: u32 = ((image_width as f64 / aspect_ratio as f64) as u32).max(1);

    // World
    let mut world: hittable_list = hittable_list::new();
    world.add(hittable_obj::sphere(sphere::new(
        point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(hittable_obj::sphere(sphere::new(
        point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: vec3 = point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - vec3::new(0.0, 0.0, focal_length) - viewport_u * 0.5 - viewport_v * 0.5;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let mut image_data: Vec<u8> = Vec::new();

    let mut pb = pbr::ProgressBar::new(image_height as u64);
    pb.show_speed = false;
    pb.show_message = true;
    pb.message("Scanline ");
    pb.format("[=> ]");

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            write_color(&pixel_color, &mut image_data);
        }
        pb.inc();
    }
    pb.finish_print("Rendering Complete.");
    println!();

    create_image("image.png", image_width, image_height, &image_data);
}
