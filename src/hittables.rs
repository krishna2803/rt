#![allow(non_camel_case_types)]

use crate::materials::*;
use crate::maths::*;
use crate::objects::*;

#[derive(Clone)]
pub struct hit_record {
    pub p: point3,
    pub normal: vec3,
    pub t: f64,
    front_face: bool,
    pub mat: material,
}

impl hit_record {
    pub fn null() -> hit_record {
        hit_record {
            p: point3::from_scalar(0.0),
            normal: vec3::from_scalar(0.0),
            t: 0.0,
            front_face: false,
            mat: material::lambertian(lambertian::empty()),
        }
    }
    pub fn set_face_normal(&mut self, r: &ray, outward_normal: vec3) {
        self.front_face = vec3::dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait hittable {
    fn hit(&self, r: &ray, ray_t: &interval, rec: &mut hit_record) -> bool;
}

#[derive(Clone)]
pub enum hittable_obj {
    sphere(sphere),
}

impl hittable for hittable_obj {
    fn hit(&self, r: &ray, ray_t: &interval, rec: &mut hit_record) -> bool {
        match self {
            hittable_obj::sphere(x) => x.hit(r, ray_t, rec),
        }
    }
}

#[derive(Clone)]
pub struct hittable_list {
    objects: Vec<hittable_obj>,
}

impl hittable_list {
    pub fn new() -> hittable_list {
        hittable_list {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: hittable_obj) {
        self.objects.push(obj);
    }
}

impl hittable for hittable_list {
    fn hit(&self, r: &ray, ray_t: &interval, rec: &mut hit_record) -> bool {
        let mut temp_rec = hit_record::null();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max();

        for object in self.objects.iter() {
            if object.hit(
                r,
                &interval::new(ray_t.min(), closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.set_face_normal(r, temp_rec.normal);
                rec.mat = temp_rec.mat.clone();
            }
        }

        hit_anything
    }
}
