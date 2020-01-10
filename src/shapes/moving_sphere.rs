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
use crate::aabb::{
    self,
    AABB,
};

pub struct Centers {
    pub starting: Point3,
    pub ending: Point3,
}

pub struct MovingSphere {
    movement_time_interval: Interval<f32>,
    centers: Centers,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl MovingSphere {
    pub fn new(centers: Centers, radius: f32, movement_time_interval: Interval<f32>, material: Box<dyn Material>) -> MovingSphere {
        MovingSphere {
            centers,
            movement_time_interval,
            radius,
            material,
        }
    }

    pub fn center_at(&self, time: f32) -> Point3 {
        let Centers{starting, ending} = self.centers;
        let (tmin, tmax) = (self.movement_time_interval.min(), self.movement_time_interval.max()); 
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
        if let Some(x) = solutions.iter().find(|&&x| hit_interval.min() < x && x < hit_interval.max()) {
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

    fn bounding_box(&self, time_interval: &Interval<f32>) -> Option<AABB> {
        let v = math::vec3(self.radius, self.radius, self.radius);
        let center0 = self.center_at(time_interval.min());
        let center1 = self.center_at(time_interval.max());
        let box0 = AABB {
            min: center0 - v,
            max: center0 + v
        };
        let box1 = AABB {
            min: center1 - v,
            max: center1 + v,
        };
        Some(aabb::surrounding_box(&box0, &box1))
    }
}