use cgmath::prelude::InnerSpace;

mod math;
mod ray;
mod shapes;

use math::{Point3, Vec3, vec3};
use ray::Ray;
use shapes::Sphere;

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

fn to_colour(ray: &Ray) -> Colour {
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    if let Some(t) = hit_point(&sphere, ray) {
        let normal = (ray.at(t) - sphere.center).normalize();
        return normal.map(|c| c + 1.0)
                     .map(|c| c * 0.5)
                     .into();
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Colour::from((1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0))
}

fn hit_point(sphere: &Sphere, ray: &Ray) -> Option<f32> {
    let oc = ray.origin - sphere.center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * ray.direction.dot(oc);
    let c = oc.dot(oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    }
    else {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let origin = Point3::new(0.0, 0.0, 0.0);
    let vertical = vec3(0.0, 2.0, 0.0);
    let horizontal = vec3(4.0, 0.0, 0.0);
    let lower_left_corner = vec3(-2.0, -1.0, -1.0);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let direction = lower_left_corner + u * horizontal + v * vertical;
            let ray = Ray::new(origin, direction);
            let colour = to_colour(&ray);
            let ir = (255.99 * colour.r) as i32;
            let ig = (255.99 * colour.g) as i32;
            let ib = (255.99 * colour.b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
