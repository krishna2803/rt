use crate::hittables::*;
use crate::image::*;
use crate::maths::*;

#[allow(non_camel_case_types)]
pub struct camera {
    // aspect_ratio: f64,
    image_width: u32,
    image_height: u32,

    center: point3,
    pixel00_loc: point3,
    pixel_delta_u: vec3,
    pixel_delta_v: vec3,
}

impl camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> camera {
        let image_height = ((image_width as f64 / aspect_ratio as f64) as u32).max(1);
        let center = point3::zero();

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - vec3::new(0.0, 0.0, focal_length) - viewport_u * 0.5 - viewport_v * 0.5;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        camera {
            // aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &hittable_list) {
        let mut image_data: Vec<u8> = Vec::new();

        let mut pb = pbr::ProgressBar::new(self.image_height as u64);
        pb.show_speed = false;
        pb.show_message = true;
        pb.message("Scanline ");
        pb.format("[=> ]");

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center =
                    self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
                let ray_direction = pixel_center - self.center;
                let r = ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(&r, &world);

                unsafe {
                    image_data.push((pixel_color.x() * 255.999).to_int_unchecked());
                    image_data.push((pixel_color.y() * 255.999).to_int_unchecked());
                    image_data.push((pixel_color.z() * 255.999).to_int_unchecked());
                    image_data.push(255); // because of the alpha channel
                }
            }
            pb.inc();
        }

        create_image("image.png", self.image_width, self.image_height, &image_data);
        pb.finish_print("Rendering Complete.");
        println!();
    }

    fn ray_color(&self, r: &ray, world: &hittable_list) -> color {
        let mut rec = hit_record::null();

        if world.hit(r, &interval::new(0.0, f64::INFINITY), &mut rec) {
            (rec.normal + color::from_scalar(1.0)) * 0.5
        } else {
            let unit_direction = r.direction().normalized();
            let a = 0.5 * (unit_direction.y() + 1.0);
            color::new(0.5, 0.7, 1.0) * a + color::new(1.0, 1.0, 1.0) * (1.0 - a)
        }
    }
}
