mod math;
mod shapes;
mod world;
mod camera;
mod materials;
mod core;

use rand::Rng;
use shapes::Sphere;
use math::{
    Point3,
    Vec3,
    vec3, 
    Interval,
};

use crate::core::{
    Ray, 
    Hittable,
};
use world::World;
use camera::{
    CameraAxis,
    Camera,
};

type Colour = Vec3;

fn to_colour(ray: &Ray, world: &World, depth: i32) -> Colour {
    use materials::Result;
    use math::{InnerSpace, VectorSpace};

    let interval = Interval::new(0.001, std::f32::MAX).unwrap();
    if let Some(interaction) = world.hit(ray, &interval) {
        let scatter_result = interaction.material.scatter(ray, &interaction);
        if depth < 50 && scatter_result.is_some() {
            let Result{ scattered_ray, attenuation } = scatter_result.unwrap();
            let colour = to_colour(&scattered_ray, world, depth + 1);
            Colour::new(
             attenuation[0] * colour[0],
             attenuation[1] * colour[1],
             attenuation[2] * colour[2]
            )
        }
        else {
            Colour::new(0.0, 0.0, 0.0)
        }
    }
    else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let start = vec3(1.0, 1.0, 1.0);
        let end = vec3(0.5, 0.7, 1.0);
        start.lerp(end, t)
    }
}

fn random_float_from_0_to_1() -> f32 {
    rand::thread_rng().gen_range(0.0, 1.0)
}

fn make_sample_world() -> World {
    use materials::{Lambertian, Metal, Dielectric};

    let hittables: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(vec3(0.1, 0.2, 0.5))))),
        Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0, Box::new(Lambertian::new(vec3(0.8, 0.8, 0.0))))),
        Box::new(Sphere::new(Point3::new(1.0, 0.0,-1.0), 0.5, Box::new(Metal::new(vec3(0.8, 0.6, 0.2), 0.3)))),
        Box::new(Sphere::new(Point3::new(-1.0, 0.0,-1.0), 0.5, Box::new(Dielectric::new(1.5)))),
        Box::new(Sphere::new(Point3::new(-1.0, 0.0,-1.0), -0.45, Box::new(Dielectric::new(1.5)))),
    ];
    World::new(hittables)
}

fn make_sample_camera(aspect: f32) -> Camera {
    use math::InnerSpace;
    let axis = CameraAxis { 
        look_from: Point3::new(3.0, 3.0, 2.0),
        look_at: Point3::new(0.0, 0.0, -1.0),
    };
    let vector_up = vec3(0.0, 1.0, 0.0);
    let aperture = 2.0;
    let dist_to_focus = (axis.look_from - axis.look_at).magnitude();
    let fov = camera::FieldOfView::from_degrees(20.0);
    Camera::new(axis, vector_up, fov, aspect, aperture, dist_to_focus)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let pixel_samples_count = 100;
    let aspect = nx as f32 / ny as f32;
    let world = make_sample_world();
    let camera = make_sample_camera(aspect);
    
    println!("P3\n{} {}\n255", nx, ny);
    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..pixel_samples_count {
                let u = (x as f32 + random_float_from_0_to_1()) / nx as f32;
                let v = (y as f32 + random_float_from_0_to_1()) / ny as f32;
                let ray = camera.make_ray(u, v);
                colour += to_colour(&ray, &world, 0);
            }
            let colour = (colour / pixel_samples_count as f32).map(|c| c.sqrt());
            let red   = (255.99 * colour[0]) as i32;
            let green = (255.99 * colour[1]) as i32;
            let blue  = (255.99 * colour[2]) as i32;
            println!("{} {} {}", red, green, blue);
        }
    }
}