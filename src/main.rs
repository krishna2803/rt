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

fn ray_color(r: &ray) -> color {
    color::from_scalar(0.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 256u32;
    let image_height = ((image_width as f64 / aspect_ratio as f64) as u32).max(1);

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width =  viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = point3::new(0.0, 0.0, 0.0);

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

    let count = image_height;
    let mut pb = ProgressBar::new(count as u64);
    pb.show_speed = false;
    pb.show_time_left = true;
    pb.show_message = true;
    pb.message("Rendering Line ");
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

    create_image("image.png", image_width, image_height, &image_data);
}
