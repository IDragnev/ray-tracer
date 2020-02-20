use crate::{
    math::{
        Vec3, 
        Interval,
    },
    core::{
        Hittable, 
        HitRecord,
        Ray,
    },
    aabb::{
        AABB,
    },
};

pub struct Translation {
    hittable: Box<dyn Hittable>,
    offset: Vec3,
}

impl Translation {
    pub fn on(hittable: Box<dyn Hittable>, offset: Vec3) -> Self {
        Translation {
            hittable,
            offset,
        }
    }
}

impl Hittable for Translation {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction - self.offset, ray.time);
        self.hittable.hit(&moved_ray, hit_interval)
        .map(|hit_rec| {
            HitRecord {
                t: hit_rec.t,
                uv: hit_rec.uv,
                normal: hit_rec.normal,
                material: hit_rec.material,
                hit_point: hit_rec.hit_point + self.offset,
            }
        })
    }

    fn bounding_box(&self, time_interval: &Interval<f32>) -> Option<AABB> {
        self.hittable.bounding_box(time_interval)
        .map(|bbox| {
            AABB {
                min: bbox.min + self.offset,
                max: bbox.max + self.offset,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        materials::{
            Dielectric,
        },
        shapes::{
            XYRectangle,
        },
        math::{
            self,
            Point3,
        },
    };

    #[test]
    fn ray_through_a_translated_rectangle_hits_it() {
        let material = Box::new(Dielectric::new(1.5));
        let rect = XYRectangle::new(-5.0, 5.0, 0.0, 10.0, -4.0, material);
        let offset = math::vec3(5.0, 0.0, 0.0);
        let translated_rect = Translation::on(Box::new(rect), offset);
        let hit_interval = Interval::new(0.0, 100.0).unwrap();
        let ray = Ray::new(Point3::new(6.0, -3.0, 0.0), math::vec3(6.0, 3.0, -4.0), 1.0);
        
        assert!(translated_rect.hit(&ray, &hit_interval).is_some());
    }
    
    #[test]
    fn ray_outside_a_translated_rectangle_does_not_hit_it() {
        let material = Box::new(Dielectric::new(1.5));
        let rect = XYRectangle::new(-5.0, 5.0, 0.0, 10.0, -4.0, material);
        let offset = math::vec3(5.0, 0.0, 0.0);
        let translated_rect = Translation::on(Box::new(rect), offset);
        let hit_interval = Interval::new(0.0, 100.0).unwrap();
        let ray = Ray::new(Point3::new(-1.0, -3.0, 0.0), math::vec3(-1.0, 3.0, -4.0), 1.0);
        
        assert!(translated_rect.hit(&ray, &hit_interval).is_none());
    }
}