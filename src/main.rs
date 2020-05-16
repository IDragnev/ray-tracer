#[macro_use]
extern crate lazy_static;
extern crate clap;

mod cmd;
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
use camera::{
    Camera,
};
use random::{
    random_float_from_0_to_1,
};
use std::{
    fs::File,
    io::{
        Write,
        BufWriter,
    },
    path::Path,
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

fn run(file: impl Write, scene_name: &str, nx :i32, ny: i32, samples_per_pixel: i32) -> Result<(), String> {
    let aspect = nx as f32 / ny as f32;
    let time_interval = Interval::new(0.0, 1.0).unwrap();
    let (scene, camera) = make_scene(scene_name, aspect, &time_interval)?;
    let (tmin, tmax) = (time_interval.min(), time_interval.max());
    
    let mut file = BufWriter::new(file);
    let _ = writeln!(file, "P3\n{} {}\n255", nx, ny);
    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut colour = Colour::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f32 + random_float_from_0_to_1()) / nx as f32;
                let v = (y as f32 + random_float_from_0_to_1()) / ny as f32;
                let time = tmin + random_float_from_0_to_1()*(tmax - tmin);
                let ray = camera.make_ray((u, v), time);
                colour += to_colour(&ray, &scene, 0);
            }
            let colour = (colour / samples_per_pixel as f32).map(|c| c.sqrt());
            let red   = (255.99 * colour[0]) as i32;
            let green = (255.99 * colour[1]) as i32;
            let blue  = (255.99 * colour[2]) as i32;
            let _ = writeln!(file, "{} {} {}", red, green, blue);
        }
    }
    file.flush().unwrap();
    
    Ok(())
}

fn make_scene(name: &str, aspect: f32, time_interval: &Interval<f32>) -> Result<(Scene, Camera), String> {
    match name {
        "cornell-box"    => Ok(example_scenes::cornell_box::scene(aspect, time_interval)),
        "simple-light"   => Ok(example_scenes::simple_light::scene(aspect, time_interval)),
        "random-spheres" => Ok(example_scenes::many_random_spheres::scene(aspect, time_interval)),
        _                => Err(format!("Invalid scene: `{}`", name)),
    }
}

fn main() {
    let args = cmd::parse();
    let output = Path::new(&args.output);
    
    match File::create(&output) {
        Ok(file) => {
            if let Err(e) = run(file, &args.scene, args.width, args.height, args.pixel_samples) {
                panic!("Error! {}", e);
            }
        },
        Err(e) => panic!("Error! Couldn't create {}: {}", output.display(), e.to_string()),
    };
}