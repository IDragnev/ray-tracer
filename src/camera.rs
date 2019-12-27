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
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

pub struct FieldOfView {
    radians: f32,
}

impl FieldOfView {
    pub fn from_degrees(degrees: f32) -> FieldOfView {
        FieldOfView {
            radians: degrees.to_radians(),
        }
    }
    pub fn to_radians(&self) -> f32 {
        self.radians
    }
}

impl Camera {
    pub fn new(axis: CameraAxis, vector_up: Vec3, fov: FieldOfView, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        use math::InnerSpace;
        let theta = fov.to_radians();
        let half_height = f32::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let origin = axis.look_from;
        let w = (axis.look_from - axis.look_at).normalize();
        let u = vector_up.cross(w).normalize();
        let v = w.cross(u);
        let lower_left_corner = origin
            - half_width  * focus_dist * u
            - half_height * focus_dist * v
            - focus_dist * w;
        Camera {
            origin,
            lower_left_corner,
            horizontal: 2.0 * focus_dist * u * half_width,
            vertical: 2.0 * focus_dist * v * half_height,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }
    
    pub fn make_ray(&self, s: f32, t: f32) -> Ray {
        use math::EuclideanSpace;
        let rd = self.lens_radius * random_point_in_unit_disk().to_vec();
        let offset = self.u * rd.x + self.v * rd.y;
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset;
        Ray::new(self.origin + offset, direction)
    }
}

fn random_point_in_unit_disk() -> Point3 {
    use crate::math::{vec3, EuclideanSpace, InnerSpace};
    use crate::random_float_from_0_to_1;
    loop {
        let mut vec = vec3(random_float_from_0_to_1(), random_float_from_0_to_1(), 0.0);
        vec = 2.0 * vec - vec3(1.0, 1.0, 0.0);
        if vec.magnitude() < 1.0 {
            return EuclideanSpace::from_vec(vec);
        }
    }
}