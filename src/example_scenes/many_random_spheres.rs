use crate::{
    shapes::{
        self,
        Sphere,
        MovingSphere,
    },
    math::{
        self,
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
        Dielectric,
        Metal,
        DiffuseLight,
    },
    textures::{
        ConstantTexture,
        CheckerTexture,
    },
    random::{
        random_float_from_0_to_1,
    },
};

pub fn scene(camera_aspect: f32, time_interval: &Interval<f32>) -> (Scene, Camera) {
    use math::{EuclideanSpace, InnerSpace};

    let mut hittables: Vec<Box<dyn Hittable>> = Vec::with_capacity(512);

    let checker = CheckerTexture{
        even: Box::new(ConstantTexture::from_rgb(vec3(0.2, 0.3, 0.1))),
        odd: Box::new(ConstantTexture::from_rgb(vec3(0.9, 0.9, 0.9))),
    };
    hittables.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(Lambertian::new(Box::new(checker))))));
    
    for a in -10..10 {
        for b in -10..10 {
            let center = Point3::new(a as f32 + 0.9*random_float_from_0_to_1(), 0.2, b as f32 + 0.9*random_float_from_0_to_1());
            if (center.to_vec() - vec3(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                hittables.push(random_sphere(center));
            }
        }
    }
    
    hittables.push(Box::new(Sphere::new(Point3::new(0.0, 5.0, 0.0), 1.0, Box::new(DiffuseLight::new(Box::new(ConstantTexture::from_rgb(vec3(4.0, 4.0, 4.0))))))));
    hittables.push(Box::new(Sphere::new(Point3::new(-4.0, 5.0, 0.0), 1.0, Box::new(DiffuseLight::new(Box::new(ConstantTexture::from_rgb(vec3(4.0, 4.0, 4.0))))))));
    hittables.push(Box::new(Sphere::new(Point3::new(4.0, 5.0, 0.0), 1.0, Box::new(DiffuseLight::new(Box::new(ConstantTexture::from_rgb(vec3(4.0, 4.0, 4.0))))))));
    hittables.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Box::new(Dielectric::new(1.5)))));
    hittables.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Box::new(Lambertian::new(Box::new(ConstantTexture::from_rgb(vec3(0.4, 0.2, 0.1))))))));
    hittables.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Box::new(Metal::new(vec3(0.7, 0.6, 0.5), 0.0)))));
    
    (Scene::new(hittables, time_interval), camera(camera_aspect)) 
}

fn random_sphere(center: Point3) -> Box<dyn Hittable> {
    use shapes::moving_sphere::Centers;

    let rf01 = || random_float_from_0_to_1();
    let randf = rf01();
    let radius = 0.2;
    if randf < 0.7 {  // diffuse
        let centers = Centers {
            starting: center,
            ending: center + vec3(0.0, 0.5*rf01(), 0.0),
        };
        let movement_time_interval = Interval::new(0.0, 1.0).unwrap();
        let abledo = Box::new(ConstantTexture::from_rgb(vec3(rf01()*rf01(), rf01()*rf01(), rf01()*rf01())));
        let material = Box::new(Lambertian::new(abledo));
        Box::new(MovingSphere::new(centers, radius, movement_time_interval, material))
    }
    else if randf < 0.8 { // metal
        let albedo = vec3(rf01(), rf01(), rf01()).map(|c| c + 1.0).map(|c| 0.5*c);
        let fuzz = 0.5*rf01();
        let material = Box::new(Metal::new(albedo, fuzz));
        Box::new(Sphere::new(center, radius, material))
    }
    else if randf < 0.95 { //light
        let material = Box::new(DiffuseLight::new(Box::new(ConstantTexture::from_rgb(vec3(4.0, 4.0, 4.0)))));
        Box::new(Sphere::new(center, radius, material))
    }
    else {  // glass
        let refractive_index = 1.5;
        let material = Box::new(Dielectric::new(refractive_index));
        Box::new(Sphere::new(center, radius, material))
    }
}

fn camera(aspect: f32) -> Camera {
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
