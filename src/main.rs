mod math;
mod ray;
mod shapes;
mod world;
mod camera;

use rand::Rng;
use math::{Point3, Vec3, vec3, Interval};
use ray::Ray;
use shapes::Sphere;
use world::World;
use camera::Camera;

type Colour = Vec3;

fn to_colour(ray: &Ray, world: &World) -> Colour {
    use math::{EuclideanSpace, VectorSpace, InnerSpace};

    let interval = Interval::new(0.001, std::f32::MAX).unwrap();
    if let Some(interaction) = world.hit(ray, &interval) {
        let tangent_unit_sphere_center = interaction.hit_point + interaction.normal;
        let target = tangent_unit_sphere_center + random_point_from_unit_sphere().to_vec();
        let direction = target - interaction.hit_point;
        let ray = Ray::new(interaction.hit_point, direction);
        0.5 * to_colour(&ray, &world)
    }
    else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let start = vec3(1.0, 1.0, 1.0);
        let end = vec3(0.5, 0.7, 1.0);
        start.lerp(end, t)
    }
}

fn random_point_from_unit_sphere() -> Point3 {
    use math::{EuclideanSpace, InnerSpace};
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

fn random_float_from_0_to_1() -> f32 {
    rand::thread_rng().gen_range(0.0, 1.0)
}

pub struct Interaction {
    pub t: f32,
    pub hit_point: Point3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, hit_interval: &Interval<f32>) -> Option<Interaction>;
}

fn make_sample_world() -> World {
    let hittables: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)),
    ];
    World::new(hittables)
}

fn make_sample_camera() -> Camera {
    let origin = Point3::new(0.0, 0.0, 0.0);
    let lower_left_corner = Point3::new(-2.0, -1.0, -1.0);
    let horizontal = vec3(4.0, 0.0, 0.0);
    let vertical = vec3(0.0, 2.0, 0.0);
    Camera::new(
        origin,
        lower_left_corner,
        horizontal,
        vertical
    )
}

fn main() {
    let nx = 200;
    let ny = 100;
    let pixel_samples_count = 100;
    let world = make_sample_world();
    let camera = make_sample_camera();
    
    println!("P3\n{} {}\n255", nx, ny);
    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..pixel_samples_count {
                let u = (x as f32 + random_float_from_0_to_1()) / nx as f32;
                let v = (y as f32 + random_float_from_0_to_1()) / ny as f32;
                let ray = camera.make_ray(u, v);
                colour += to_colour(&ray, &world);
            }
            let colour = (colour / pixel_samples_count as f32).map(|c| c.sqrt());
            let red   = (255.99 * colour[0]) as i32;
            let green = (255.99 * colour[1]) as i32;
            let blue  = (255.99 * colour[2]) as i32;
            println!("{} {} {}", red, green, blue);
        }
    }
}
