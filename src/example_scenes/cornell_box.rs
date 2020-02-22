use crate::{
    shapes::{
        XYRectangle,
        XZRectangle,
        YZRectangle,
        FlipNormals,
        Parallelepiped,
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
        Material,
        Lambertian, 
        DiffuseLight,
    },
    textures::{
        ConstantTexture,
    },
    transformations::{
        Translation,
        YRotation,
    },
};

pub fn scene(camera_aspect: f32, time_interval: &Interval<f32>) -> (Scene, Camera) {
    let red = Box::new(Lambertian::new(Box::new(ConstantTexture::from_rgb(vec3(0.65, 0.05, 0.05))))) as Box<dyn Material>;
    let green = Box::new(Lambertian::new(Box::new(ConstantTexture::from_rgb(vec3(0.12, 0.45, 0.15))))) as Box<dyn Material>;
    let light = Box::new(DiffuseLight::new(Box::new(ConstantTexture::from_rgb(vec3(15.0, 15.0, 15.0))))) as Box<dyn Material>;
    let diffuse_light_mat_generator = || -> Box<dyn Material> { 
        Box::new(Lambertian::new(Box::new(ConstantTexture::from_rgb(vec3(0.73, 0.73, 0.73))))) 
    };
    
    let hittables: Vec<Box<dyn Hittable>> = vec![
        Box::new(FlipNormals::new(YZRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, green))),
        Box::new(YZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red)),
        Box::new(XZRectangle::new(213.0, 343.0, 227.0, 332.0, 554.0, light)),
        Box::new(XZRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, diffuse_light_mat_generator())),
        Box::new(FlipNormals::new(XYRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, diffuse_light_mat_generator()))),
        Box::new(FlipNormals::new(XZRectangle::new(0.0, 555.0, 0.0, 555.0, 555.0, diffuse_light_mat_generator()))),
        Box::new(Translation::on(
            Box::new(YRotation::from_degrees(
                Box::new(Parallelepiped::new(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 165.0, 165.0), diffuse_light_mat_generator)),
                -18.0
            )),
            vec3(130.0, 0.0, 65.0)
        )),
        Box::new(Translation::on(
            Box::new(YRotation::from_degrees(
                Box::new(Parallelepiped::new(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 330.0, 165.0), diffuse_light_mat_generator)),
                15.0
            )),
            vec3(265.0, 0.0, 295.0)
        )),
    ];

    (Scene::new(hittables, time_interval), camera(camera_aspect))
}

fn camera(aspect: f32) -> Camera {
    let camera_axis = CameraAxis{ 
        look_from: Point3::new(278.0, 278.0, -800.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
    };
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let fov = camera::FieldOfView::from_degrees(40.0);
    let v_up = vec3(0.0, 1.0, 0.0);

    Camera::new(camera_axis, v_up, fov, aspect, aperture, dist_to_focus)
}