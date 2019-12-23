use crate::math::{Point3, Vec3};
use crate::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Point3, lower_left_corner: Point3, horizontal: Vec3, vertical: Vec3) -> Camera {
        Camera {
            origin, 
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    
    pub fn make_ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin, direction)
    }
}