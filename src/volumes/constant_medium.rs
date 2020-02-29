use crate::{
    core::{
        Hittable,
        HitRecord,
        Ray,
    },
    materials::{
        Material,
        Isotropic,
    },
    random,
    aabb::{
        AABB,
    },
    math::{
        self,
        Interval,
    },
    textures::{
        Texture,
        TextureCoordinates,
    },
};

pub struct ConstantMedium {
    density: f32,
    boundary: Box<dyn Hittable>,
    phase_function: Box<dyn Material>,
}

impl ConstantMedium {
    pub fn new(density: f32, boundary: Box<dyn Hittable>, texture: Box<dyn Texture>) -> Self {
        Self {
            density,
            boundary,
            phase_function: Box::new(Isotropic::new(texture)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<HitRecord> {
        use math::InnerSpace;
        
        let interval = Interval::new(-std::f32::MAX, std::f32::MAX).unwrap();
        self.boundary.hit(ray, &interval)
        .and_then(|entry_hit_rec| {
            let interval = Interval::new(entry_hit_rec.t + 0.0001, std::f32::MAX).unwrap();
            self.boundary.hit(ray, &interval) 
            .and_then(|leave_hit_rec| {
                let entry_hit_t = if entry_hit_rec.t < hit_interval.min() { hit_interval.min() } else { entry_hit_rec.t };
                let leave_hit_t = if leave_hit_rec.t > hit_interval.max() { hit_interval.max() } else { leave_hit_rec.t };
                if entry_hit_t >= leave_hit_t {
                    return None;
                }

                let entry_hit_t = if entry_hit_t < 0.0 { 0.0 } else { entry_hit_t };
                let direction_magnitude = ray.direction.magnitude();
                let distance_inside_boundary = (leave_hit_t - entry_hit_t) * direction_magnitude;
                let hit_distance = -(1.0 / self.density) * random::random_float_from_0_to_1().log(std::f32::consts::E);
                
                if hit_distance < distance_inside_boundary {
                    let t = entry_hit_t + hit_distance / direction_magnitude; 
                    Some(HitRecord {
                        t,
                        hit_point: ray.at(t),
                        normal: math::vec3(1.0, 0.0, 0.0),
                        material: self.phase_function.as_ref(),
                        uv: TextureCoordinates::zero(),
                    })
                }
                else { 
                    None
                }
            })
        })
    }

    fn bounding_box(&self, time_interval: &Interval<f32>) -> Option<AABB> {
        self.boundary.bounding_box(time_interval)
    }
}