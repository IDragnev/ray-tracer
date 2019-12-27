use crate::math::{
    self,
    Point3,
    Vec3,
};
use crate::core::Ray;

pub struct CameraAxis {
    pub look_from: Point3,
    pub look_at: Point3,
}

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(axis: CameraAxis, vector_up: Vec3, deg_top_to_bottom: f32, aspect: f32) -> Camera {
        use math::InnerSpace;
        let theta = deg_top_to_bottom.to_radians();
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let origin = axis.look_from;
        let w = (axis.look_from - axis.look_at).normalize();
        let u = vector_up.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            origin,
            lower_left_corner: origin - u*half_width - v*half_height - w,
            horizontal: 2.0 * u * half_width,
            vertical: 2.0 * v * half_height,
        }
    }
    
    pub fn make_ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin, direction)
    }
}