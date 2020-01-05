mod math;
mod shapes;
mod world;
mod camera;
mod materials;
mod core;
mod aabb;
mod bvh;

use rand::Rng;
use shapes::{
    Sphere,
    MovingSphere,
};
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

fn random_scene() -> World {
    use materials::{Lambertian, Metal, Dielectric};
    use math::{EuclideanSpace, InnerSpace};
    
    let mut hittables: Vec<Box<dyn Hittable>> = Vec::with_capacity(512);
    hittables.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(vec3(0.5, 0.5, 0.5))))));
    
    for a in -10..10 {
        for b in -10..10 {
            let center = Point3::new(a as f32 + 0.9*random_float_from_0_to_1(), 0.2, b as f32 + 0.9*random_float_from_0_to_1());
            if (center.to_vec() - vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                hittables.push(random_sphere(center));
            }
        }
    }
    
    hittables.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielectric::new(1.5)))));
    hittables.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(vec3(0.4, 0.2, 0.1))))));
    hittables.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(vec3(0.7, 0.6, 0.5), 0.0)))));
    
    World::new(hittables)
}

fn random_sphere(center: Point3) -> Box<dyn Hittable> {
    use materials::{Lambertian, Metal, Dielectric};
    use shapes::moving_sphere::Centers;

    let rf01 = || random_float_from_0_to_1();
    let randf = rf01();
    let radius = 0.2;
    if randf < 0.8 {  // diffuse
        let centers = Centers {
            starting: center,
            ending: center + vec3(0.0, 0.5*rf01(), 0.0),
        };
        let movement_time_interval = Interval::new(0.0, 1.0).unwrap();
        let abledo = vec3(rf01()*rf01(), rf01()*rf01(), rf01()*rf01());
        let material = Box::new(Lambertian::new(abledo));
        Box::new(MovingSphere::new(centers, radius, movement_time_interval, material))
    }
    else if randf < 0.95 { // metal
        let albedo = vec3(rf01(), rf01(), rf01()).map(|c| c + 1.0).map(|c| 0.5*c);
        let fuzz = 0.5*rf01();
        let material = Box::new(Metal::new(albedo, fuzz));
        Box::new(Sphere::new(center, radius, material))
    }
    else {  // glass
        let refractive_index = 1.5;
        let material = Box::new(Dielectric::new(refractive_index));
        Box::new(Sphere::new(center, radius, material))
    }
}

fn make_sample_camera(aspect: f32) -> Camera {
    let axis = CameraAxis { 
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
    };
    let vector_up = vec3(0.0, 1.0, 0.0);
    let aperture = 0.0;
    let dist_to_focus = 10.0;
    let fov = camera::FieldOfView::from_degrees(20.0);
    Camera::new(axis, vector_up, fov, aspect, aperture, dist_to_focus)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let pixel_samples_count = 100;
    let aspect = nx as f32 / ny as f32;
    let world = random_scene();
    let camera = make_sample_camera(aspect);
    let (tmin, tmax) = (0.0, 1.0);
    println!("P3\n{} {}\n255", nx, ny);
    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..pixel_samples_count {
                let u = (x as f32 + random_float_from_0_to_1()) / nx as f32;
                let v = (y as f32 + random_float_from_0_to_1()) / ny as f32;
                let time = tmin + random_float_from_0_to_1()*(tmax - tmin);
                let ray = camera.make_ray((u, v), time);
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