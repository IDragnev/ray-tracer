use crate::{
    shapes::{
        Sphere,
        XYRectangle,
    },
    math::{
        Point3,
        vec3, 
        Interval,
    },
    core::{
        Hittable,
    },
    scene::{
        Scene,
    },
    camera::{
        self,
        CameraAxis,
        Camera,
    },
    materials::{
        Lambertian, 
        DiffuseLight,
    },
    textures::{
        ConstantTexture,
        NoiseTexture,
    },
};

pub fn scene(camera_aspect: f32, time_interval: &Interval<f32>) -> (Scene, Camera) {
    let perlin_texture = Box::new(NoiseTexture::new(4.0));
    let mut hittables: Vec<Box<dyn Hittable>> = Vec::with_capacity(4);
    hittables.push(Box::new(Sphere::new(Point3::new(0.0,-1000.0, 0.0), 1000.0, Box::new(Lambertian::new(perlin_texture.clone())))));
    hittables.push(Box::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Box::new(Lambertian::new(perlin_texture)))));
    hittables.push(Box::new(Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, Box::new(DiffuseLight::new(Box::new(ConstantTexture::from_rgb(vec3(4.0, 4.0, 4.0))))))));
    hittables.push(Box::new(XYRectangle::new(3.0, 5.0, 1.0, 3.0, -2.0, Box::new(DiffuseLight::new(Box::new(ConstantTexture::from_rgb(vec3(4.0, 4.0, 4.0))))))));
   
    (Scene::new(hittables, time_interval), camera(camera_aspect))
}

fn camera(aspect: f32) -> Camera {
    let axis = CameraAxis { 
        look_from: Point3::new(17.0, 7.0, 20.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
    };
    let vector_up = vec3(0.0, 1.0, 0.0);
    let aperture = 0.0;
    let dist_to_focus = 10.0;
    let fov = camera::FieldOfView::from_degrees(20.0);
    Camera::new(axis, vector_up, fov, aspect, aperture, dist_to_focus)
}