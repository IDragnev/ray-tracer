use cgmath::prelude::InnerSpace;
use cgmath::prelude::VectorSpace;
use rand::Rng;

mod math;
mod ray;
mod shapes;
mod world;
mod camera;

use math::{Point3, Vec3, vec3, Interval};
use ray::Ray;
use shapes::Sphere;
use world::World;
use camera::Camera;

type Colour = Vec3;

fn random_float() -> f32 {
    rand::thread_rng().gen_range(0.0, 1.0)
}

fn to_colour(ray: &Ray, world: &World) -> Colour {
    let interval = Interval::new(0.0, std::f32::MAX).unwrap();
    if let Some(interaction) = world.hit(ray, &interval) {
        interaction.normal
            .map(|c| c + 1.0)
            .map(|c| c * 0.5)
    }
    else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let start = vec3(1.0, 1.0, 1.0);
        let end = vec3(0.5, 0.7, 1.0);
        start.lerp(end, t)
    }
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
                let u = (x as f32 + random_float()) / nx as f32;
                let v = (y as f32 + random_float()) / ny as f32;
                let ray = camera.make_ray(u, v);
                colour += to_colour(&ray, &world);
            }
            colour /= pixel_samples_count as f32;

            let red   = (255.99 * colour[0]) as i32;
            let green = (255.99 * colour[1]) as i32;
            let blue  = (255.99 * colour[2]) as i32;
            println!("{} {} {}", red, green, blue);
        }
    }
}
