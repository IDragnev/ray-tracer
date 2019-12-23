use cgmath::prelude::InnerSpace;

mod math;
mod ray;
mod shapes;
mod world;

use math::{Point3, Vec3, vec3, Interval};
use ray::Ray;
use shapes::Sphere;
use world::World;

struct Colour {
    r: f32,
    g: f32,
    b: f32,
}

impl Colour {
    fn new(r: f32, g: f32, b: f32) -> Colour {
        Colour {
            r,
            g,
            b,
        }
    }
}

impl From<Vec3> for Colour {
    fn from(v: Vec3) -> Colour {
        Colour::new(v[0], v[1], v[2])
    }
}

fn to_colour(ray: &Ray, world: &World) -> Colour {
    let interval = Interval::new(0.0, std::f32::MAX).unwrap();
    if let Some(interaction) = world.hit(ray, &interval) {
        interaction.normal
            .map(|c| c + 1.0)
            .map(|c| c * 0.5)
            .into()
    }
    else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        Colour::from((1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0))
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

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let origin = Point3::new(0.0, 0.0, 0.0);
    let vertical = vec3(0.0, 2.0, 0.0);
    let horizontal = vec3(4.0, 0.0, 0.0);
    let lower_left_corner = vec3(-2.0, -1.0, -1.0);
    let world = make_sample_world();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let direction = lower_left_corner + u * horizontal + v * vertical;
            let ray = Ray::new(origin, direction);
            let colour = to_colour(&ray, &world);
            let ir = (255.99 * colour.r) as i32;
            let ig = (255.99 * colour.g) as i32;
            let ib = (255.99 * colour.b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
