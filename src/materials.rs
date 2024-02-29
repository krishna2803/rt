#![allow(non_camel_case_types)]

use crate::hittables::*;
use crate::maths::*;

#[derive(Clone)]
pub enum material {
    lambertian(lambertian),
    metal(metal),
}

impl material_trait for material {
    fn scatter(
        &self,
        r_in: &ray,
        rec: &hit_record,
        attenuation: &mut color,
        scattered: &mut ray,
    ) -> bool {
        match self {
            material::lambertian(x) => {
                // eprintln!("typ1 loll");
                x.scatter(r_in, rec, attenuation, scattered)
            }
            material::metal(x) => {
                x.scatter(r_in, rec, attenuation, scattered)
            }
        }
    }

    fn empty() -> Self {
        material::lambertian(lambertian::new(color::zero()))
    }
}

pub trait material_trait {
    fn scatter(
        &self,
        r_in: &ray,
        rec: &hit_record,
        attenuation: &mut color,
        scattered: &mut ray,
    ) -> bool;
    fn empty() -> Self;
}

#[derive(Clone, Copy)]
pub struct lambertian {
    albedo: color,
}

impl lambertian {
    pub fn new(albedo: color) -> lambertian {
        lambertian { albedo }
    }
}

impl material_trait for lambertian {
    fn scatter(
        &self,
        _r_in: &ray,
        rec: &hit_record,
        attenuation: &mut color,
        scattered: &mut ray,
    ) -> bool {
        let scatter_direction = rec.normal + vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            *scattered = ray::new(rec.p, rec.normal);
        } else {
            *scattered = ray::new(rec.p, scatter_direction);
        }
        
        *attenuation = self.albedo;
        true
    }

    fn empty() -> lambertian {
        lambertian {
            albedo: color::zero(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct metal {
    albedo: color,
}

impl metal {
    pub fn new(albedo: color) -> metal {
        metal { albedo }
    }
}

impl material_trait for metal {
    fn scatter(
        &self,
        r_in: &ray,
        rec: &hit_record,
        attenuation: &mut color,
        scattered: &mut ray,
    ) -> bool {
        let reflected = vec3::reflect(&r_in.direction().normalized(), &rec.normal);
        *scattered = ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        true
    }

    fn empty() -> metal {
        metal {
            albedo: color::zero(),
        }
    }
}
