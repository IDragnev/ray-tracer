use crate::Interaction;
use crate::ray::Ray;
use crate::math::{Vec3, Point3, vec3, EuclideanSpace, InnerSpace};

pub struct Result {
    pub attenuation: Vec3,
    pub scattered_ray: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, interaction: &Interaction) -> Option<Result>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, interaction: &Interaction) -> Option<Result> {
        let tangent_unit_sphere_center = interaction.hit_point + interaction.normal;
        let target = tangent_unit_sphere_center + random_point_from_unit_sphere().to_vec();
        let direction = target - interaction.hit_point;
        let scattered_ray = Ray::new(interaction.hit_point, direction);
        let attenuation = self.albedo;
        Some(Result{
            attenuation,
            scattered_ray,
        })
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, interaction: &Interaction) -> Option<Result> {
        use crate::math::{reflected, dot};
        let reflected_dir = reflected(&ray.direction.normalize(), &interaction.normal);
        let is_angle_acute = dot(reflected_dir, interaction.normal) > 0.0;
        if is_angle_acute {
            let direction = reflected_dir + self.fuzz * random_point_from_unit_sphere().to_vec();
            let scattered_ray = Ray::new(interaction.hit_point, direction);
            let attenuation = self.albedo;
            Some(Result{
                scattered_ray,
                attenuation,
            })
        }
        else {
            None
        }
    }
}

fn random_point_from_unit_sphere() -> Point3 {
    use crate::random_float_from_0_to_1;
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