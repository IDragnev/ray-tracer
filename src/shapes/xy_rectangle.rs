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
        AABB,
    },
    textures::{
        TextureCoordinates,
    }
};

pub struct XYRectangle {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    z: f32,
    material: Box<dyn Material>,
}

impl XYRectangle {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, z: f32, material: Box<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            z,
            material,
        }
    }
}

impl Hittable for XYRectangle {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<HitRecord> {
        let t = (self.z - ray.origin.z) / ray.direction.z;
        if t < hit_interval.min() || t > hit_interval.max() { 
            return None;
        }

        let hit_point = ray.at(t);
        if hit_point.x < self.x0 || hit_point.x > self.x1 || 
           hit_point.y < self.y0 || hit_point.y > self.y1 {
            return None;
        }

        let uv = TextureCoordinates {
            u: (hit_point.x - self.x0) / (self.x1 - self.x0),
            v: (hit_point.y - self.y0) / (self.y1 - self.y0),
        };
        Some(HitRecord{
            t,
            uv,
            hit_point,
            material: self.material.as_ref(),
            normal: math::vec3(0.0, 0.0, 1.0),
        })
    }

    fn bounding_box(&self, _: &Interval<f32>) -> Option<AABB> {
        Some(AABB {
            min: Point3::new(self.x0, self.y0, self.z - 0.0001), 
            max: Point3::new(self.x1, self.y1, self.z + 0.0001),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::materials::Dielectric;

    #[test]
    fn ray_through_the_rectangle_hits_it() {
        let material = Box::new(Dielectric::new(1.5));
        let rect = XYRectangle::new(0.0, 10.0, 0.0, 10.0, -4.0, material);
        let hit_interval = Interval::new(0.0, 100.0).unwrap();
        let ray = Ray::new(Point3::new(1.0, -3.0, 0.0), math::vec3(1.0, 3.0, -1.0), 1.0);

        assert!(rect.hit(&ray, &hit_interval).is_some());
    }
    
    #[test]
    fn ray_outside_the_rectangle_does_not_hit_it() {
        let material = Box::new(Dielectric::new(1.5));
        let rect = XYRectangle::new(0.0, 10.0, 0.0, 10.0, -4.0, material);
        let hit_interval = Interval::new(0.0, 100.0).unwrap();
        let ray = Ray::new(Point3::new(1.0, -3.0, 0.0), math::vec3(1.0, -3.0, -1.0), 1.0);

        assert!(rect.hit(&ray, &hit_interval).is_none());
    }
}