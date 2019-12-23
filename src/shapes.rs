use crate::math::{Point3, Interval, dot};
use crate::{Hittable, Interaction, Ray};

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<Interaction> {
        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction);
        let b = dot(ray.direction, oc);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant <= 0.0 {
            return None;
        }

        let d_sqrt = discriminant.sqrt();
        let solutions = [(-b - d_sqrt) / a, (-b + d_sqrt) / a];
        if let Some(x) = solutions.iter().find(|&&x| hit_interval.min < x && x < hit_interval.max) {
            let t = *x;
            let hit_point = ray.at(t);
            let normal = (hit_point - self.center) / self.radius;
            Some(Interaction {
                t,
                hit_point,
                normal,
            })
        }
        else { 
            None
        }
    }
}