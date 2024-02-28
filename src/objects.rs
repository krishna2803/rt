use crate::maths::*;
use crate::hittables::*;

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct sphere {
    center: point3,
    radius: f64,
}

impl sphere {
    pub fn new(center: point3, radius: f64) -> sphere {
        sphere { center, radius }
    }
}

impl hittable for sphere {
    fn hit(&self, r: &ray, ray_tmin: f64, ray_tmax: f64, rec: &mut hit_record) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root += 2.0 * sqrtd / a;
            if root <= ray_tmin || root >= ray_tmax {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}
