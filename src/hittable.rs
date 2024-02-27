use crate::maths::*;

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
struct hit_record {
    p: point3,
    normal: vec3,
    t: f64,
    front_face: bool,
}

impl hit_record {
    pub fn set_face_normal(mut self, r: &ray, outward_normal: vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

#[allow(non_camel_case_types)]
trait hittable {
    fn hit(self, r: &ray, ray_tmin: f64, ray_tmax: f64, rec: &mut hit_record) -> bool;
}

#[allow(non_camel_case_types)]
struct sphere {
    center: point3,
    radius: f64,
}

impl sphere {
    fn new(center: point3, radius: f64) -> sphere {
        sphere { center, radius }
    }
}

impl hittable for sphere {
    fn hit(self, r: &ray, ray_tmin: f64, ray_tmax: f64, rec: &mut hit_record) -> bool {
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
