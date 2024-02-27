pub mod maths;
pub mod hittable;
pub mod image;

use std::time::Duration;

use pbr::ProgressBar;

use maths::*;
use image::*;

fn write_color(c: &color, data: &mut Vec<u8>) {
    unsafe {
        data.push((c.x() * 255.999).to_int_unchecked());
        data.push((c.y() * 255.999).to_int_unchecked());
        data.push((c.z() * 255.999).to_int_unchecked());
        data.push(255u8); // because of the alpha channel
    }
}

fn hit_sphere(center: point3, radius: f64, r: &ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = vec3::dot(oc, r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b * half_b - a*c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &ray) -> color {
    let center = point3::new(0.0, 0.0, -1.0);
    let radius = 0.5;
    let t = hit_sphere(center, radius, r);
    if t > 0.0 {
        let n = (r.at(t) - center).normalized();
        color::new(n.x()+1.0, n.y()+1.0, n.z()+1.0) * 0.5
    } else {
        let unit_direction = r.direction().normalized();
        let a = 0.5 * (unit_direction.y() + 1.0);
        let start_color = color::new(0.5, 0.7, 1.0);
        let end_color = color::new(1.0, 1.0, 1.0);
        start_color * a + end_color * (1.0 - a)
    }
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 10.0;
    let image_width: u32 = 1000;
    let image_height: u32 = ((image_width as f64 / aspect_ratio as f64) as u32).max(1);

    // camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 =  viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: vec3 = point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - vec3::new(0.0, 0.0, focal_length) - viewport_u * 0.5 - viewport_v * 0.5;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let mut image_data: Vec<u8> = Vec::new();

    let mut pb = ProgressBar::new(image_height as u64);
    pb.show_speed = false;
    pb.show_message = true;
    pb.message("Scanline ");
    pb.format("[=> ]");


    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = ray::new(camera_center, ray_direction);
            
            let pixel_color = ray_color(&r);
            write_color(&pixel_color, &mut image_data);
        }
        pb.inc();
    }
    pb.finish_print("Rendering Complete.");
    println!();

    create_image("image.png", image_width, image_height, &image_data);
}
