mod interval;

pub type Vec3 = cgmath::Vector3<f32>;
pub type Point3 = cgmath::Point3<f32>;

pub use cgmath::{ 
    vec3,
    dot,
    EuclideanSpace,
    prelude::InnerSpace,
    prelude::VectorSpace,
};
pub use interval::Interval;

pub fn reflected(v: &Vec3, normal: &Vec3) -> Vec3 {
    v - 2.0 * dot(*v, *normal) * normal
}

pub fn refracted(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let unit_v = v.normalize();
    let dt = dot(unit_v, *n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (unit_v - n * dt) - n * discriminant.sqrt())
    }
    else { 
        None
    }
}

pub fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = { 
        let a = (1.0 - refractive_index) / (1.0 + refractive_index);
        a * a 
    };
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub fn random_point_from_unit_sphere() -> Point3 {
    use crate::random::random_float_from_0_to_1;

    loop {
        let vec = 
            vec3(random_float_from_0_to_1(), random_float_from_0_to_1(), random_float_from_0_to_1())
            .map(|c| 2.0 * c)
            .map(|c| c - 1.0);
        if vec.magnitude() < 1.0 {
            return EuclideanSpace::from_vec(vec);
        }
    }
}