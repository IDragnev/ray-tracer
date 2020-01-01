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

pub struct Centers {
    pub starting: Point3,
    pub ending: Point3,
}

pub type MovementInterval = Interval<f32>;

pub struct MovingSphere {
    movement_interval: MovementInterval,
    centers: Centers,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl MovingSphere {
    pub fn new(centers: Centers, radius: f32, movement_interval: MovementInterval, material: Box<dyn Material>) -> MovingSphere {
        MovingSphere {
            centers,
            movement_interval,
            radius,
            material,
        }
    }

    pub fn center_at(&self, time: f32) -> Point3 {
        let Centers{starting, ending} = self.centers;
        let MovementInterval{min: tmin, max: tmax} = self.movement_interval; 
        if time <= tmin {
            starting
        }
        else if time >= tmax {
            ending
        }
        else {
            starting + ((time - tmin) / (tmax - tmin))*(ending - starting)
        }
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<Interaction> {
        use math::dot;
        let center = self.center_at(ray.time);
        let oc = ray.origin - center;
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
            let normal = (hit_point - center) / self.radius;
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
}