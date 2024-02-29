#![allow(non_camel_case_types)]

use crate::hittables::*;
use crate::image::*;
use crate::material_trait;
use crate::maths::*;

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
        let now = std::time::Instant::now();

        let mut image_data: Vec<u8> = Vec::new();
        image_data.reserve_exact((self.image_height * self.image_width << 2) as usize);

        let samples_per_pixel: u32 = 10;
        let max_depth: u32 = 10;

        let mut pb = pbr::ProgressBar::new(self.image_height as u64);
        pb.show_speed = false;
        pb.show_message = true;
        pb.message("Scanline ");
        pb.format("[=> ]");

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = color::zero();

                for _ in 0..samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, max_depth, world);
                }

                self.write_color(pixel_color, &mut image_data, samples_per_pixel);
            }
            pb.inc();
        }

        create_image(
            "image.png",
            self.image_width,
            self.image_height,
            &image_data,
        );
        let elapsed_time = now.elapsed();
        let s = format!("Rendering Complete in {:.2} s", elapsed_time.as_secs_f32());
        pb.finish_print(&s);
        println!();
    }

    fn pixel_sample_square(&self) -> vec3 {
        let px = -0.5 + fastrand::f64();
        let py = -0.5 + fastrand::f64();
        self.pixel_delta_u * px + self.pixel_delta_v * py
    }

    fn get_ray(&self, i: u32, j: u32) -> ray {
        let pixel_center =
            self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        ray::new(ray_origin, ray_direction)
    }

    fn ray_color(&self, r: &ray, depth: u32, world: &hittable_list) -> color {
        let mut rec = hit_record::null();

        if depth <= 0 {
            return color::zero();
        }

        if world.hit(r, &interval::new(0.001, f64::INFINITY), &mut rec) {
            // let direction = rec.normal + vec3::random_unit_vector();
            // let new_ray = ray::new(rec.p, direction);
            // let reflectance = 0.3;

            // self.ray_color(&new_ray, depth-1, world) * reflectance
            // (rec.normal + color::from_scalar(1.0)) * 0.5

            let mut scattered = ray::null();
            let mut attenuation = color::zero();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                attenuation * self.ray_color(&scattered, depth-1, &world)
            } else {
                color::zero()
            }
        } else {
            let unit_direction = r.direction().normalized();
            let a = 0.5 * (unit_direction.y() + 1.0);
            color::new(0.5, 0.7, 1.0) * a + color::new(1.0, 1.0, 1.0) * (1.0 - a)
        }
    }

    fn write_color(&self, pixel_color: color, image_data: &mut Vec<u8>, samples_per_pixel: u32) {
        let mut r = pixel_color.x();
        let mut g = pixel_color.y();
        let mut b = pixel_color.z();

        let scale = 1.0 / samples_per_pixel as f64;

        r *= scale;
        g *= scale;
        b *= scale;

        // gamma correction
        let linear_to_gamma = |x: f64| x.sqrt();

        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        let intensity: interval = interval::new(0.000, 0.999);

        unsafe {
            let ir: u8 = (256.0 * intensity.clamp(r)).to_int_unchecked();
            let ig: u8 = (256.0 * intensity.clamp(g)).to_int_unchecked();
            let ib: u8 = (256.0 * intensity.clamp(b)).to_int_unchecked();
            let alpha: u8 = 255;

            image_data.push(ir);
            image_data.push(ig);
            image_data.push(ib);
            image_data.push(alpha);
        }
    }
}
