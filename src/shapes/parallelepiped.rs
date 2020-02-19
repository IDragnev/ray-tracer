use crate::{
    math::{
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
        AABB,
    },
    shapes::{
        XYRectangle,
        XZRectangle,
        YZRectangle,
        FlipNormals,
    }
};

pub struct Parallelepiped {
    min: Point3,
    max: Point3,
    walls: [Box<dyn Hittable>; 6],
}

impl Hittable for Parallelepiped {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<HitRecord> {
        let mut interval = *hit_interval;
        let mut result = None;
        for hittable in &self.walls {
            if let Some(hit_record) = hittable.hit(ray, &interval) {
                interval = interval.with_max(hit_record.t).expect("invalid interval");
                result = Some(hit_record);
            }
        }       
        result
    }

    fn bounding_box(&self, _: &Interval<f32>) -> Option<AABB> {
        Some(AABB{
            min: self.min,
            max: self.max,
        })
    }
}

impl Parallelepiped {
    pub fn new<F: Fn() -> Box<dyn Material>>(min: &Point3, max: &Point3, material_generator: F) -> Self {
        let walls : [Box<dyn Hittable>; 6] = [
            Box::new(XYRectangle::new(min.x, max.x, min.y, max.y, max.z, material_generator())),
            Box::new(FlipNormals::new(XYRectangle::new(min.x, max.x, min.y, max.y, min.z, material_generator()))),
            Box::new(XZRectangle::new(min.x, max.x, min.z, max.z, max.y, material_generator())),
            Box::new(FlipNormals::new(XZRectangle::new(min.x, max.x, min.z, max.z, min.y, material_generator()))),
            Box::new(YZRectangle::new(min.y, max.y, min.z, max.z, max.x, material_generator())),
            Box::new(FlipNormals::new(YZRectangle::new(min.y, max.y, min.z, max.z, min.x, material_generator()))),
        ];

        Self {
            min: *min,
            max: *max,
            walls,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        materials::{
            Dielectric,
        },
        math,
    };

    #[test]
    fn ray_through_the_parallelepiped_hits_it() {
        let material_gen = || Box::new(Dielectric::new(1.5)) as Box<dyn Material>;
        let (min, max) = (Point3::new(0.0, 0.0, 0.0), Point3::new(3.0, 3.0, 3.0));
        let parallelepiped = Parallelepiped::new(&min, &max, material_gen);
        let hit_interval = Interval::new(0.0, std::f32::MAX).unwrap();
        let ray = Ray::new(Point3::new(1.0, -1.0, 1.0), math::vec3(1.0, 2.0, 1.0), 1.0);

        assert!(parallelepiped.hit(&ray, &hit_interval).is_some());
    }
    
    #[test]
    fn ray_outside_the_parallelepiped_does_not_hit_it() {
        let material_gen = || Box::new(Dielectric::new(1.5)) as Box<dyn Material>;
        let (min, max) = (Point3::new(0.0, 0.0, 0.0), Point3::new(3.0, 3.0, 3.0));
        let parallelepiped = Parallelepiped::new(&min, &max, material_gen);
        let hit_interval = Interval::new(0.0, std::f32::MAX).unwrap();
        let ray = Ray::new(Point3::new(1.0, -5.0, -5.0), math::vec3(5.0, 5.0, 5.0), 1.0);

        assert!(parallelepiped.hit(&ray, &hit_interval).is_none());
    }
}
