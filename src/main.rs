#[macro_use]
extern crate lazy_static;

mod random;
mod math;
mod shapes;
mod scene;
mod camera;
mod materials;
mod core;
mod aabb;
mod bvh;
mod textures;
mod transformations;
mod example_scenes;
mod volumes;

use math::{
    Vec3, 
    Interval,
};
use crate::core::{
    Ray, 
    Hittable,
};
use scene::{
    Scene,
};
use random::{
    random_float_from_0_to_1,
};

type Colour = Vec3;

fn to_colour(ray: &Ray, scene: &Scene, depth: i32) -> Colour {
    use materials::ScatterResult;

    let interval = Interval::new(0.001, std::f32::MAX).unwrap();
    if let Some(hit_record) = scene.hit(ray, &interval) {
        let scatter_result = hit_record.material.scatter(ray, &hit_record);
        let emitted = hit_record.material.emitted(&hit_record.uv, &hit_record.hit_point);
        if depth < 50 && scatter_result.is_some() {
            let ScatterResult{ scattered_ray, attenuation } = scatter_result.unwrap();
            let colour = to_colour(&scattered_ray, scene, depth + 1);
            Colour::new(
             attenuation[0] * colour[0],
             attenuation[1] * colour[1],
             attenuation[2] * colour[2])
            + emitted
        }
        else {
            emitted
        }
    }
    else {
        Colour::new(0.0, 0.0, 0.0)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let pixel_samples_count = 100;
    let aspect = nx as f32 / ny as f32;
    let time_interval = Interval::new(0.0, 1.0).unwrap();
    let (scene, camera) = example_scenes::cornell_box::scene(aspect, &time_interval);
    let (tmin, tmax) = (time_interval.min(), time_interval.max());
    println!("P3\n{} {}\n255", nx, ny);
    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..pixel_samples_count {
                let u = (x as f32 + random_float_from_0_to_1()) / nx as f32;
                let v = (y as f32 + random_float_from_0_to_1()) / ny as f32;
                let time = tmin + random_float_from_0_to_1()*(tmax - tmin);
                let ray = camera.make_ray((u, v), time);
                colour += to_colour(&ray, &scene, 0);
            }
            let colour = (colour / pixel_samples_count as f32).map(|c| c.sqrt());
            let red   = (255.99 * colour[0]) as i32;
            let green = (255.99 * colour[1]) as i32;
            let blue  = (255.99 * colour[2]) as i32;
            println!("{} {} {}", red, green, blue);
        }
    }
}