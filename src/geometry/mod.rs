pub mod vec3;

use vec3::{dot, Vec3};

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo>;
}

pub struct HitInfo {
    pub t: f32,
    pub normal: Vec3<f32>,
    pub at: Vec3<f32>,
}

pub struct Ray {
    pub origin: Vec3<f32>,
    pub direction: Vec3<f32>,
}

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo> {
        let oc = r.origin - self.center;
        let a = dot(r.direction, r.direction);
        let half_b = dot(oc, r.direction);
        let c = dot(oc, oc) - self.radius * self.radius;

        let discrim = half_b * half_b - a * c;

        if discrim < 0.0 {
            return None;
        }

        let mut t = (-half_b - discrim.sqrt()) / a;
        if t < t_min || t > t_max {
            t = (-half_b + discrim.sqrt()) / a;
            if t < t_min || t > t_max {
                return None;
            }
        }
        let at = r.origin + r.direction * t;
        return Some(HitInfo {
            t,
            normal: (at - self.center) / self.radius,
            at,
        });
    }
}

#[derive(Clone, Copy)]
pub struct Plane {
    pub normal: Vec3<f32>,
    pub point: Vec3<f32>,
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitInfo> {
        let d = dot(self.normal, r.direction);
        if d == 0.0 {
            return None;
        }
        let t = dot(self.point - r.origin, self.normal) / d;
        if t <= t_min || t >= t_max {
            return None;
        }
        return Some(HitInfo {
            t,
            normal: self.normal,
            at: r.origin + r.direction * t,
        });
    }
}
