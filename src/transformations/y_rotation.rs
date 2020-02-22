use crate::{
    math::{
        self,
        Vec3,
        vec3, 
        Point3,
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

pub struct YRotation {
    hittable: Box<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
}

impl YRotation {
    pub fn from_degrees(hittable: Box<dyn Hittable>, theta: f32) -> Self {
        let theta = theta.to_radians();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let bbox = hittable
            .bounding_box(&Interval::new(0.0, 1.0).unwrap())
            .map(|bbox| {
                let mut min = Point3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
                let mut max = Point3::new(-std::f32::MAX, -std::f32::MAX, -std::f32::MAX);
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let (ii, jj, kk) = (i as f32, j as f32, k as f32);
                            let x = ii * bbox.max.x + (1.0 - ii) * bbox.min.x;
                            let y = jj * bbox.max.y + (1.0 - jj) * bbox.min.y;
                            let z = kk * bbox.max.z + (1.0 - kk) * bbox.min.z;
                            let new_x =  cos_theta * x + sin_theta * z;
                            let new_z = -sin_theta * x + cos_theta * z;
                            let test_vec = vec3(new_x, y, new_z);
                            for c in 0..3 {
                                if test_vec[c] > max[c] {
                                    max[c] = test_vec[c];
                                }
                                if test_vec[c] < min[c] {
                                    min[c] = test_vec[c];
                                }
                            }
                        }
                    }
                }
                
                AABB { min, max }
            });

            Self {
                hittable,
                sin_theta,
                cos_theta,
                bbox,
            }
    }
}

impl Hittable for YRotation {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<HitRecord> {
        use math::EuclideanSpace;

        let rotate_y_theta = |v: Vec3| {
            let mut result = v;
            result.x = self.cos_theta * v.x - self.sin_theta * v.z;
            result.z = self.sin_theta * v.x + self.cos_theta * v.z;
            result
        };
        let rotate_y_minus_theta = |v: Vec3| {
            let mut result = v;
            result.x =  self.cos_theta * v.x + self.sin_theta * v.z;
            result.z = -self.sin_theta * v.x + self.cos_theta * v.z;
            result
        };

        let origin = Point3::from_vec(rotate_y_theta(ray.origin.to_vec()));
        let direction = rotate_y_theta(ray.direction);
        let rotated_ray = Ray::new(origin, direction, ray.time);
        
        self.hittable.hit(&rotated_ray, hit_interval)
        .map(|hit_rec| {
            let normal = rotate_y_minus_theta(hit_rec.normal);
            let hit_point = Point3::from_vec(rotate_y_minus_theta(hit_rec.hit_point.to_vec()));
            HitRecord {
                t: hit_rec.t,
                uv: hit_rec.uv,
                material: hit_rec.material,
                normal,
                hit_point,
            }
        })
    }

    fn bounding_box(&self, _: &Interval<f32>) -> Option<AABB> {
        self.bbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        materials::{
            Material,
            Dielectric,
        },
        math,
        shapes::{
            Parallelepiped,
        },
    };

    #[test]
    fn ray_through_a_rotated_parallelepiped_hits_it() {
        let material_gen = || Box::new(Dielectric::new(1.5)) as Box<dyn Material>;
        let (min, max) = (Point3::new(0.0, 0.0, 0.0), Point3::new(3.0, 3.0, 3.0));
        let parallelepiped = Parallelepiped::new(&min, &max, material_gen);
        let rotated_parallelepiped = YRotation::from_degrees(Box::new(parallelepiped), 10.0);
        let hit_interval = Interval::new(0.0, std::f32::MAX).unwrap();
        let ray = Ray::new(Point3::new(1.5, 4.5, 1.5), math::vec3(0.0, -1.0, 0.0), 1.0);
        
        assert!(rotated_parallelepiped.hit(&ray, &hit_interval).is_some());
    }
    
    #[test]
    fn ray_outside_a_rotated_parallelepiped_does_not_hit_it() {
        let material_gen = || Box::new(Dielectric::new(1.5)) as Box<dyn Material>;
        let (min, max) = (Point3::new(0.0, 0.0, 0.0), Point3::new(3.0, 3.0, 3.0));
        let parallelepiped = Parallelepiped::new(&min, &max, material_gen);
        let rotated_parallelepiped = YRotation::from_degrees(Box::new(parallelepiped), 10.0);
        let hit_interval = Interval::new(0.0, std::f32::MAX).unwrap();
        let ray = Ray::new(Point3::new(1.0, -5.0, -5.0), math::vec3(5.0, 5.0, 5.0), 1.0);

        assert!(rotated_parallelepiped.hit(&ray, &hit_interval).is_none());
    }
}
