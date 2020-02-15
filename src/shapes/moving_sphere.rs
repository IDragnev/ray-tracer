use crate::{
    math::{
        self,
        Point3, 
        Interval,
    },
    core::{
        Hittable, 
        HitRecord,
        Ray,
    },
    materials::{
        Material,
    },
    aabb::{
        self,
        AABB,
    },
    textures::{
        TextureCoordinates,
    },
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
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<HitRecord> {
        use math::{dot, EuclideanSpace};
        
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
        solutions
        .iter()
        .find(|&&x| hit_interval.min() < x && x < hit_interval.max()) 
        .and_then(|x| {
            let t = *x;
            let hit_point = ray.at(t);
            let normal = (hit_point - center) / self.radius;
            let unit_sphere_coords = Point3::from_vec(normal);
            Some(HitRecord {
                t,
                hit_point,
                normal,
                material: self.material.as_ref(),
                uv: TextureCoordinates::from_unit_sphere_coordinates(&unit_sphere_coords),
            })
        })
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::materials::Dielectric;

    #[test]
    fn ray_outside_a_moving_sphere_at_cast_time_does_not_hit_it() {
        let centers = Centers{ 
            starting: Point3::new(2.0, 2.0, 2.0),
            ending: Point3::new(-1.0, 3.0, 3.0),
        };
        let movement_interval = Interval::new(0.0, 10.0).unwrap(); 
        let sphere = MovingSphere::new(
            centers,
            2.0,
            movement_interval,
            Box::new(Dielectric::new(1.5)),
        );
        let ray = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            math::vec3(-1.0, 2.0, 2.0),
            0.0
        );
        let interval = Interval::new(0.0, 100.0).unwrap();

        let hit_record = sphere.hit(&ray, &interval);

        assert!(hit_record.is_none());
    }

    #[test]
    fn ray_through_a_moving_sphere_at_cast_time_hits_it() {
        let centers = Centers {
            starting: Point3::new(2.0, 2.0, 2.0),
            ending: Point3::new(300.0, 200.0, 200.5),
        };
        let movement_interval = Interval::new(0.0, 10.0).unwrap();
        let sphere = MovingSphere::new(
            centers,
            2.0,
            movement_interval,
            Box::new(Dielectric::new(1.5)),
        );
        let ray = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            math::vec3(1.0, 1.0, 1.0),
            0.0
        );
        let interval = Interval::new(0.0, 100.0).unwrap();

        let hit_record = sphere.hit(&ray, &interval);

        assert!(hit_record.is_some());
    }
}