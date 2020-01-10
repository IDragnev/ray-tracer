use crate::math::{
    self,
    Point3, 
    Interval,
};
use crate::core::{
    Hittable, 
    Interaction,
    Ray,
};
use crate::materials::Material;
use crate::aabb::AABB;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<Interaction> {
        use math::dot;
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
        if let Some(x) = solutions.iter().find(|&&x| hit_interval.min() < x && x < hit_interval.max()) {
            let t = *x;
            let hit_point = ray.at(t);
            let normal = (hit_point - self.center) / self.radius;
            Some(Interaction {
                t,
                hit_point,
                normal,
                material: self.material.as_ref(),
            })
        }
        else { 
            None
        }
    }

    fn bounding_box(&self, _: &Interval<f32>) -> Option<AABB> {
        let v = math::vec3(self.radius, self.radius, self.radius);
        let b = AABB {
            min: self.center - v,
            max: self.center + v,
        };
        Some(b)
    }
}